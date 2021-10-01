use actix_session::{CookieSession, Session};
use actix_web::web;
use actix_web::{post, App, HttpResponse, HttpServer, Responder};
use mongodb::{bson::doc, sync::Client};
use serde::{Deserialize, Serialize};
use urlencoding::decode;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct Login {
    email: String,
    password: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct Register {
    email: String,
    password: String,
    dwa: String,
    sex: String,
}

#[post("/login")]
async fn login(req_body: String) -> impl Responder {
    // session: Session
    let v: Login = match serde_qs::from_str(&req_body) {
        Ok(v) => v,
        Err(_) => return HttpResponse::Ok().body("coś tam kombinujesz  ;("),
    };
    // if let Some(count) = session.get::<i32>("counter").unwrap() {
    //     println!("SESSION value: {}", count);
    //     session.set("counter", count + 1).unwrap();
    // } else {
    //     session.set("counter", 1).unwrap();
    // }

    HttpResponse::Ok().body(format!(
        "email: {} \npassword: {}",
        decode(&v.email).unwrap().to_string(),
        v.password
    ))
}

#[post("/register")]
async fn register(req_body: String) -> impl Responder {
    println!("{}", req_body);
    let v: Register = match serde_qs::from_str(&req_body) {
        Ok(v) => v,
        Err(_) => {
            serde_qs::from_str("email=none%40none.none&password=none&dwa=none&sex=none").unwrap()
        }
    };
    if v.password == v.dwa {
        let client = Client::with_uri_str("mongodb://localhost:27017").unwrap();
        let database = client.database("user");
        let collection = database.collection::<&Register>("users");
        collection.insert_one(&v, None).unwrap();
        return HttpResponse::Ok().body(format!(
            "email: {} \npassword: {}\ndwa: {}\nsex: {}",
            decode(&v.email).unwrap().to_string(),
            v.password,
            v.dwa,
            v.sex
        ));
    } else {
        return HttpResponse::Ok().body(format!(r#"<script type="text/javascript">
        window.location.href = "{a}"
    </script>         If you are not redirected automatically, follow this <a href='{a}'>link to example</a>.
    "#,a="/register_f_not_same_passwd.html"));
    }
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
