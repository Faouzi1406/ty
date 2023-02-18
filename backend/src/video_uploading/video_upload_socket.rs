use serde::{Deserialize, Serialize};
use std::{io::Write, fs::OpenOptions};

use actix::{Actor, StreamHandler};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;

pub struct WsVideoUploadSession {
    file_name: String,
    file: Vec<u8>,
    file_size: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileUpload {
    file_name: String,
    file_size: usize,
}

impl Actor for WsVideoUploadSession {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsVideoUploadSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(value) => match value {
                ws::Message::Text(value) => {
                    let file_upload: Result<FileUpload, serde_json::Error> =
                        serde_json::from_str(&value);

                    let file_upload = match file_upload {
                        Ok(value) => {
                            self.file_name = value.file_name;
                            self.file_size = value.file_size
                        }
                        Err(e) => {
                            println!("Error: {:?}", e);
                            return;
                        }
                    };
                }
                ws::Message::Binary(value) => {
                    self.file.extend(value);
                    let mut file = OpenOptions::new()
                        .write(true)
                        .create(true)
                        .open(&self.file_name)
                        .unwrap();

                    if self.file.len() == self.file_size {
                        file.write(&self.file).unwrap();
                        ctx.text("File uploaded successfully");
                    }
                }
                ws::Message::Ping(value) => {
                    println!("ping")
                }
                ws::Message::Pong(value) => {
                    println!("pong")
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
    let resp = ws::start(
        WsVideoUploadSession {
            file_name: "".to_string(),
            file_size: 0,
            file: vec![],
        },
        &req,
        stream,
    );
    println!("Response: {:?}", resp);
    resp
}
