use actix::{Actor, StreamHandler};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;

pub struct WsVideoUploadSession;

impl Actor for WsVideoUploadSession {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsVideoUploadSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(value) => match value {
                ws::Message::Text(value) => {
                    println!("Text: {}", value);
                }
                ws::Message::Binary(value) => {
                    println!("Binary: {:?}", value);
                }
                ws::Message::Ping(value) => {
                    println!("Ping: {:?}", value);
                }
                ws::Message::Pong(value) => {
                    println!("Pong: {:?}", value);
                }
                ws::Message::Close(value) => {
                    println!("Close: {:?}", value);
                }
                ws::Message::Continuation(value) => {
                    println!("Continuation: {:?}", value);
                }
                ws::Message::Nop => {
                    println!("Nop");
                }
            },
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }
}

pub async fn video_upload_socket(
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    let resp = ws::start(WsVideoUploadSession {}, &req, stream);
    println!("Response: {:?}", resp);
    resp
}
