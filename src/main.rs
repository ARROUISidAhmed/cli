use std::{path::Path, fs::File, io::Write};

use actix_web::{web, HttpServer,HttpResponse,App};
use futures::StreamExt;
use serde_derive::Deserialize;

#[derive(Deserialize)]
struct Info {
    name: String,
}

async fn upload_file(mut payload: web::Payload,query: web::Query<Info>) -> Result<HttpResponse, actix_web::Error> {
    let file_name = &query.name;
    let file_path = Path::new(&file_name);
    let mut file = File::create(file_path).unwrap();

    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        file.write_all(&chunk).unwrap();
    }

    Ok(HttpResponse::Ok().body(format!("File '{}' uploaded successfully", file_name)))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().route("/upload", web::post().to(upload_file))
    })
    .bind("localhost:8080")?
    .run()
    .await
}