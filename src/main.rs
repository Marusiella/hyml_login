use actix_web::{get, web};
use actix_web::{post, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct QueryParams {
    email:String,
    password:String,
}

#[post("/login")]
async fn login(req_body: String) -> impl Responder {
    println!("{}", req_body);
    let v:QueryParams = serde_qs::from_str(&req_body).unwrap();
    HttpResponse::Ok().body(format!("email: {} \npassword: {}", v.email, v.password))
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::scope("/api").service(login))
            .service(
                actix_files::Files::new("/", "./html")
                    .show_files_listing()
                    .index_file("login.html"),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
