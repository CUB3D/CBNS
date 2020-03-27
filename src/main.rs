use actix::*;

use actix_web::{middleware, web, App, Error as AWError, HttpResponse, HttpServer};

use serde::Deserialize;

extern crate futures;
#[macro_use]
extern crate debug_rs;

mod messages;
use messages::*;

mod notification_server;
use notification_server::*;

mod notification_session;

mod notification_definition;

mod client_action;

mod endpoint;
use endpoint::post_channel::post_channel_handle;
use endpoint::post_device::post_device_handle;
use endpoint::root::root_handler;
use endpoint::socket_poll::socket_poll;

use dotenv::dotenv;

#[derive(Deserialize)]
struct PostRequest {
    destination: String,
    data: String,
}

#[derive(Deserialize)]
struct TokenExtractor {
    token: String,
}

async fn message_post(
    path: web::Path<PostRequest>,
    srv: web::Data<Addr<NotificationServer>>,
) -> Result<HttpResponse, AWError> {
    srv.send(ChannelNotificationMsg {
        channel: path.destination.clone(),
        message: path.data.clone(),
    })
    .await
    .unwrap();

    Ok(HttpResponse::Ok().body("Ok"))
}

async fn message_device_status(
    srv: web::Data<Addr<NotificationServer>>,
    path: web::Path<TokenExtractor>,
) -> Result<HttpResponse, AWError> {
    let status = srv
        .send(DeviceStatusRequestMsg {
            token: path.token.clone(),
        })
        .await
        .unwrap();

    Ok(HttpResponse::Ok().json(status))
}

async fn status_handle(srv: web::Data<Addr<NotificationServer>>) -> Result<HttpResponse, AWError> {
    let status = srv.send(StatusRequestMsg {}).await.unwrap();

    Ok(HttpResponse::Ok().json(status))
}

//TODO: tokio, status page, tcp socket for polling as well, client action channel message
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let server = NotificationServer::default().start();

    HttpServer::new(move || {
        App::new()
            .data(server.clone())
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(web::resource("/").to(root_handler))
            .service(web::resource("/poll/{token}").to(socket_poll))
            .service(web::resource("/status").route(web::get().to(status_handle)))
            .service(web::resource("/status/{token}").route(web::get().to(message_device_status)))
            //TODO: remove this
            .service(
                web::resource("/post/{destination}/{data}").route(web::post().to(message_post)),
            )
            .service(
                web::resource("/channel/{channel}/post").route(web::post().to(post_channel_handle)),
            )
            .service(
                web::resource("/device/{token}/post").route(web::post().to(post_device_handle)),
            )
    })
    .bind("0.0.0.0:8080")
    .unwrap()
    .run()
    .await
}
