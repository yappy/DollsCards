use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde::{Serialize, Deserialize};
use serde_json;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[derive(Serialize)]
struct InfoObj {
    description: String,
    version: String,
}

#[get("/info")]
async fn info() -> impl Responder {
    let info = InfoObj {
        description: "now testing...".to_string(),
        version: "0.1".to_string(),
    };
    let body = serde_json::to_string(&info).unwrap();

    HttpResponse::Ok().content_type("application/json").body(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(info)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
