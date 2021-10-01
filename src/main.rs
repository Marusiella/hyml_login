use actix_web::{get, web};
use actix_web::{post, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde::{Serialize, Deserialize};
use urlencoding::decode;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct Login {
    email:String,
    password:String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct Register {
    email:String,
    password:String,
    dwa: String,
    sex: String
}


#[post("/login")]
async fn login(req_body: String) -> impl Responder {
    println!("{}", req_body);
    let v:Login = serde_qs::from_str(&req_body).unwrap();
    HttpResponse::Ok().body(format!("email: {} \npassword: {}", decode(&v.email).unwrap().to_string(), v.password))
}

#[post("/register")]
async fn register(req_body: String) -> impl Responder {
    println!("{}", req_body);
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::scope("/api").service(login).service(register))
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
