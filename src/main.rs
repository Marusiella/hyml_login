use actix_session::{CookieSession, Session};
use actix_web::web;
use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use mongodb::{bson::doc, sync::Client};
use serde::{Deserialize, Serialize};
use actix_web::{middleware::Logger};
// use urlencoding::decode;

#[derive(Debug, PartialEq, Deserialize, Serialize,Clone)]
struct Login {
    email: String,
    password: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize,Clone)]
struct Register {
    email: String,
    password: String,
    dwa: String,
    sex: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize,Clone)]
struct Post {
    title: String,
    message: String,
    date: String,
    user: String,
    like: i32,
}


#[post("/postpost")]
async fn post(req_body: String) -> impl Responder {
    let v: Post = match serde_qs::from_str(&req_body) {
        Ok(v) => v,
        Err(_) => return HttpResponse::Ok().body("coś tam kombinujesz  ;("),
    };
    let client = Client::with_uri_str("mongodb://localhost:27017").unwrap();
    let database = client.database("post");
    let collection = database.collection::<Post>("posts");
    collection
        .insert_one(
            Post {
                title: v.title,
                message: v.message,
                date: v.date,
                user: v.user,
                like: 0,
            },
            None,
        )
        .unwrap();

    HttpResponse::Ok().body("success")
}

#[get("/getposts")]
async fn get_posts(session: Session) -> impl Responder {
    let client = Client::with_uri_str("mongodb://localhost:27017").unwrap();
    let database = client.database("post");
    let collection = database.collection::<Post>("posts");
    // if let Some(other_login) = session.get::<String>("login").ok() {
    //     match other_login {
    //         Some(other_login) => println!("{}", other_login),
    //         None => return HttpResponse::Ok().body(format!(r#"<script type="text/javascript">
    //         window.location.href = "{a}"
    //     </script>         If you are not redirected automatically, follow this <a href='{a}'>link to example</a>.
    //     "#,a="/login_session_lost.html"))
    //     }
    // }
    let other_login = session
        .get::<String>("login")
        .unwrap_or(Some("none".to_owned()))
        .unwrap_or("none".to_owned());

    if other_login == String::from("none") {
        return HttpResponse::Ok().body(format!(r#"<script type="text/javascript">
             window.location.href = "{a}"
         </script>         If you are not redirected automatically, follow this <a href='{a}'>link to example</a>.
         "#,a="/login_session_lost.html"));
    }

    let mut posts_html = format!(
        r#"<html lang="en">
    <head>
        <meta charset="UTF-8">
        <title>Posty</title>
        <link
            rel="stylesheet"
            href="https://cdn.jsdelivr.net/npm/bulma@0.9.3/css/bulma.min.css"
            />
    </head>
    <body>
        <div class="box" style="width: 300px; text-align: center;margin-left: auto;
        margin-right: auto; margin-top: 30px;">
        Login: {login}
            Posty:
        </div>
    "#,
        login = other_login
    );
    let posts = collection.find(doc! {}, None).unwrap();
    if collection.find(doc! {}, None).unwrap().count() == 0 {
        return HttpResponse::Ok().body(
            r#"<html lang="en">
        <head>
            <meta charset="UTF-8">
            <title>Posty</title>
            <link
                rel="stylesheet"
                href="https://cdn.jsdelivr.net/npm/bulma@0.9.3/css/bulma.min.css"
                />
        </head>
        <body>
            <div class="box" style="width: 300px; text-align: center;margin-left: auto;
            margin-right: auto; margin-top: 30px;">
                Brak postów :(
            </div>
        </body>
    </html>"#,
        );
    } else {
        for x in posts {
            posts_html = posts_html
                + &format!(
                    r#"
                    <div class="box" style="width: 300px; text-align: center;margin-left: auto;
        margin-right: auto; margin-top: 30px;">
            {}
            <h6 style="text-align: right">Date: {}</h6>
        </div>"#,
                    x.clone().unwrap().message,&x.unwrap().date
                );
        }
        posts_html = posts_html
            + r#"</body>
    </html>"#;
        return HttpResponse::Ok().body(posts_html);
    }

    // HttpResponse::Ok()
}

#[post("/login")]
async fn login(req_body: String, session: Session) -> impl Responder {
    let v: Login = match serde_qs::from_str(&req_body) {
        Ok(v) => v,
        Err(_) => return HttpResponse::Ok().body("coś tam kombinujesz  ;("),
    };

    let client = Client::with_uri_str("mongodb://localhost:27017").unwrap();
    let database = client.database("user");
    let collection = database.collection::<Register>("users");
    let x = collection
        .find_one(doc! {"email":&v.email,"password":&v.password}, None)
        .unwrap();

    if x.is_none() {
        return HttpResponse::Ok().body(format!(r#"<script type="text/javascript">
        window.location.href = "{a}"
    </script>         If you are not redirected automatically, follow this <a href='{a}'>link to example</a>.
    "#,a="/login_f_wrong_passwd.html"));
    } else {
        session.set("login", &v.email).unwrap()
    }
    HttpResponse::Ok().body(format!(r#"<script type="text/javascript">
        window.location.href = "{a}"
    </script>         If you are not redirected automatically, follow this <a href='{a}'>link to example</a>.
    "#,a="/getposts"))
}

#[post("/register")]
async fn register(req_body: String) -> impl Responder {
    let v: Register = match serde_qs::from_str(&req_body) {
        Ok(v) => v,
        Err(_) => {
            serde_qs::from_str("email=none%40none.none&password=none&dwa=none&sex=none").unwrap()
        }
    };
    let client = Client::with_uri_str("mongodb://localhost:27017").unwrap();
    let database = client.database("user");
    let collection = database.collection::<Register>("users");
    let x = collection.find_one(doc! {"email":&v.email}, None).unwrap();
    if !x.is_some() {
        if v.password == v.dwa {
            let client = Client::with_uri_str("mongodb://localhost:27017").unwrap();
            let database = client.database("user");
            let collection = database.collection::<&Register>("users");
            collection.insert_one(&v, None).unwrap();
            // return HttpResponse::Ok().body(format!(
            //     "email: {} \npassword: {}\ndwa: {}\nsex: {}",
            //     decode(&v.email).unwrap().to_string(),
            //     v.password,
            //     v.dwa,
            //     v.sex
            // ));
            return HttpResponse::Ok().body(format!(r#"<script type="text/javascript">
            window.location.href = "{a}"
        </script>         If you are not redirected automatically, follow this <a href='{a}'>link to example</a>.
        "#,a="/login.html"));
        } else {
            return HttpResponse::Ok().body(format!(r#"<script type="text/javascript">
            window.location.href = "{a}"
        </script>         If you are not redirected automatically, follow this <a href='{a}'>link to example</a>.
        "#,a="/register_f_not_same_passwd.html"));
        }
    } else {
        return HttpResponse::Ok().body(format!(r#"<script type="text/javascript">
            window.location.href = "{a}"
        </script>         If you are not redirected automatically, follow this <a href='{a}'>link to example</a>.
        "#,a="/register_user_exist.html"));
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .service(get_posts)
            .service(
                web::scope("/api")
                    .service(login)
                    .service(register)
                    .service(post),
            )
            .service(actix_files::Files::new("/", "./html").index_file("login.html"))
            .wrap(Logger::new("%a %{User-Agent}i"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
