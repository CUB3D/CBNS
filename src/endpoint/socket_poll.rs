use crate::notification_server::NotificationServer;
use crate::notification_session::WSNotificationSession;
use actix::Addr;
use actix_web::Error;
use actix_web::{web, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PollExtractor {
    token: String,
}

pub fn socket_poll(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<NotificationServer>>,
    path_params: web::Path<PollExtractor>,
) -> Result<HttpResponse, Error> {
    let params = path_params.into_inner();

    ws::start(
        WSNotificationSession {
            server_address: srv.get_ref().clone(),
            uid: 0,
            token: params.token,
        },
        &req,
        stream,
    )
}
