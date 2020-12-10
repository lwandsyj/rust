use actix_web::{get, post, web, App, HttpResponse,HttpRequest, HttpServer, Responder,Result};

use serde::Serialize;
#[get("/")]
async fn hello(req:HttpRequest) -> impl Responder {
    println!("{}",req.query_string());
    HttpResponse::Ok().json(IndexResponse{
        message:"hello ".to_string()
    })
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
#[derive(Serialize)]
struct  IndexResponse{
    message:String,
}
async fn manual_hello(req: HttpRequest) -> Result<web::Json<IndexResponse>> {
    Ok(web::Json(IndexResponse {
        message: "hello".to_owned(),
    }))
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}