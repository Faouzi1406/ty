use actix::{Actor, StreamHandler};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;

pub struct WsVideoUploadSession;

impl Actor for WsVideoUploadSession {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsVideoUploadSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        // Todo: Handle message and upload video correctly
        println!("WEBSOCKET MESSAGE: {:?}", msg);
    }
}

pub async fn video_upload_socket(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(WsVideoUploadSession {}, &req, stream);
    println!("Response: {:?}", resp);
    resp
}
