use actix_web::Error;
use actix_web::HttpResponse;
use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    connected_devices: u32,
}

pub async fn root_handler() -> Result<HttpResponse, Error> {
    let tpl = IndexTemplate {
        connected_devices: 0,
    }
    .render()
    .unwrap();

    Ok(HttpResponse::Ok().body(tpl))
}
