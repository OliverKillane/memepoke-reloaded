use actix_files as fs;
use actix_web::{App, HttpResponse, HttpServer, Responder, get, middleware::Logger, web::{self, Json}};
use serde::{Deserialize,Serialize};
use env_logger::Env;
use awc::Client;
use serde_json::Value;
use std::str;

// CONSTANTS:
const CLIENT_ID : &str = "WI5Q-r9Nu3EjSxIjz5SzEA";
const REDIRECT_URI : &str = "http://127.0.0.1:8080/memepoke";

#[derive(Deserialize)]
struct UserReq {
    username : String,
    authtoken : String
}


// LOGIN THE USER:
#[derive(Deserialize)]
struct LoginReq {
    code : String
}

/*
{
    "access_token": Your access token,
    "token_type": "bearer",
    "expires_in": Unix Epoch Seconds,
    "scope": A scope string,
    "refresh_token": Your refresh token
}
*/

#[derive(Deserialize)]
struct RedditAuthResp {
    access_token : String,
    token_type : String,
    expires_in : i32,
    scope : String,
}

#[derive(Serialize)]
struct User {
    username: String,
    description: String,
    profile_pic_url: String,
    auth_token: String
}

#[get("/loginuser&code={code}")]
async fn login_user(info : web::Path<LoginReq>) -> impl Responder {
    let client = Client::default();

    // use code to get auth token
    if let Ok(mut resp) = 
        client
            .post("https://www.reddit.com/api/v1/access_token")
            .basic_auth(CLIENT_ID, None)
            .header("User-Agent", "Memepoke/2.0 Running from actix web server")
            .send_form(&[("grant_type","authorization_code"), ("code", info.code.as_str()), ("redirect_uri", REDIRECT_URI)])
            .await
    {
        if let Ok(data) = resp.json::<RedditAuthResp>().await {
            let auth = data.access_token;
            println!("Auth: {}", auth);

            // use auth token to get user id
            if let Ok(mut resp) = 
                client
                    .get("https://oauth.reddit.com/api/v1/me")
                    .header("Authorization", format!("Bearer {}", auth))
                    .header("User-Agent", "Memepoke/2.0 Running from actix web server")
                    .send()
                    .await 
            {
                let s = resp.body().await.unwrap(); 
                let y = str::from_utf8(&s).unwrap();
                let v = serde_json::from_str::<Value>(y).unwrap();

                return HttpResponse::Ok().json(User {
                    username: v["name"].to_string(),
                    description: String::from("this is descr"),
                    profile_pic_url: v["icon_img"].to_string(),
                    auth_token: auth
                });
            }
        } 
    }
    HttpResponse::Forbidden().body("Invalid Request")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // initialize the logger
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    HttpServer::new(|| {
        App::new()
        .wrap(Logger::new("IP: %a | TIME: %t | REQUEST: %r | PROCESSED IN: %Dms"))
        .service(login_user)
        .service(fs::Files::new("/memepoke", "./static").index_file("index.html"))
        .service(fs::Files::new("/", "./static/"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}