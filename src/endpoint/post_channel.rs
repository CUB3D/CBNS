use crate::messages::ChannelNotificationMsg;
use crate::notification_definition::Notification;
use crate::notification_server::NotificationServer;
use actix::Addr;
use actix_web::Error;
use actix_web::{web, HttpResponse};
use futures::future::ok;
use futures::Future;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PostChannelRequest {
    channel: String,
}

pub async fn post_channel_handle(
    path: web::Path<PostChannelRequest>,
    notification_body: web::Json<Notification>,
    srv: web::Data<Addr<NotificationServer>>,
) -> Result<HttpResponse, Error> {
    let msg = serde_json::to_string(&notification_body.0);

    if let Ok(msg) = msg {
        srv.send(ChannelNotificationMsg {
            channel: path.channel.clone(),
            message: msg,
        })
        .await
        .unwrap();
    } else {
        println!("Unable to handle message {:?}", msg);
    }

    Ok(HttpResponse::Ok().body("Ok"))
}
