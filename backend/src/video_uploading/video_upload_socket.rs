use crate::traits::db::Create;
use actix::{Actor, StreamHandler};
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};
use std::{fs::OpenOptions, io::Write};

use crate::models::video::VideoCreate;

pub struct WsVideoUploadSession {
    file_name: String,
    file: Vec<u8>,
    file_size: usize,
    user_id: i32,
    thumb_mail_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileUpload {
    file_size: usize,
    user_id: i32,
    title: String,
    description: Option<String>,
    thumb_mail_url: Option<String>,
}

impl Actor for WsVideoUploadSession {
    type Context = ws::WebsocketContext<Self>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoUploaded {
    pub file_name: String,
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsVideoUploadSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(value) => match value {
                ws::Message::Text(value) => {
                    let file_upload: Result<FileUpload, serde_json::Error> =
                        serde_json::from_str(&value);

                    let file = if let Ok(file_upload) = file_upload {
                        let file_name = String::from("./videos/")
                            + uuid::Uuid::new_v4().to_string().as_str()
                            + ".mp4";

                        if file_upload.thumb_mail_url.is_some() {
                            let thumb_mail_url = String::from("./thumbmails/")
                                + uuid::Uuid::new_v4().to_string().as_str()
                                + ".jpeg";

                            let create_image = std::fs::File::create(&thumb_mail_url);
                            self.thumb_mail_url = Some(thumb_mail_url);
                            //println!("Ok");

                            if create_image.is_ok() {
                                let base_64_string = general_purpose::STANDARD_NO_PAD
                                    .decode(file_upload.thumb_mail_url.unwrap())
                                    .expect("Me is not very smart?");
                                create_image
                                    .unwrap()
                                    .write_all(&base_64_string)
                                    .expect("file go brrrrrrr....");
                            }
                        }

                        self.file_name = file_name.clone();
                        self.file_size = file_upload.file_size;
                        self.user_id = file_upload.user_id;

                        let video_create = VideoCreate {
                            title: file_upload.title,
                            url: self.file_name.clone().replace("./videos/", ""),
                            user_id: self.user_id,
                            description: file_upload.description,
                            thumb_mail_url: self.thumb_mail_url.clone(),
                        };

                        video_create.create().expect("Video create failed");
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
                        ctx.text("uploaded");
                    }
                }

                ws::Message::Close(value) => {
                    println!("Close: {:?}", value);
                }
                _ => (),
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
            user_id: 0,
            thumb_mail_url: None,
        },
        &req,
        stream,
    );
    println!("Response: {:?}", resp);
    resp
}
