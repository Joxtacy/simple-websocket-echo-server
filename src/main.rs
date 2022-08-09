//! Simple echo WebSocket server
//!
//! Open `http://localhost:8080/` in browser to test.

mod server;
use actix_web::{HttpServer, App, Error, web, HttpRequest, HttpResponse, middleware};
use actix_web_actors::ws;

use crate::server::MyWebSocket;

async fn echo_ws(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(MyWebSocket::new(), &req, stream)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        App::new()
            // WebSocket route
            .service(web::resource("/ws").route(web::get().to(echo_ws)))
            // enable logger
            .wrap(middleware::Logger::default())
    })
    .workers(2)
        .bind(("127.0.0.1", 8081))?
        .run()
        .await
}
