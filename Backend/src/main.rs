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
    id: i32,
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
                    id : 0, // to be changed later, PLACEHOLDER
                    description: String::from("this is descr"),
                    profile_pic_url: v["icon_img"].to_string(),
                    auth_token: auth
                });
            }
        } 
    }
    HttpResponse::Forbidden().body("Invalid Request")
}

// GET A NEW MEME
#[derive(Deserialize)]
struct UserAuth {
    user : String,
    id : i32
}

#[derive(Serialize)]
pub struct Meme {
    pub meme_id: u64,
    pub post : String,
    pub image_url: String,
    pub original_poster: String
}

fn get_new_meme(id : i32) -> Meme {

    //PLACEHOLDER
    Meme {
        meme_id: 0,
        post: String::from("https://www.reddit.com/r/memes/comments/pfnpjr/hes_back/"),
        image_url: String::from("https://i.redd.it/m1yj6uixztk71.jpg"),
        original_poster: String::from("daffox123"),
    }
}


#[get("/getmeme&user={user}&id={id}")]
async fn get_meme(info : web::Path<UserAuth>) -> impl Responder {
    // check auth for id, username

    HttpResponse::Ok().json(get_new_meme(info.id))
}

#[derive(Deserialize)]
struct ReactMeme {
    user : String,
    id : i32,
    meme : u64,
    react : i32
}

#[get("/reactmeme&user={user}&id={id}&meme={meme}&react={react}")]
async fn react_meme(info : web::Path<ReactMeme>) -> impl Responder {
    // check auth for id, username

    println!("User reacted: {}", match info.react {
        0 => "Good",
        1 => "Neutral",
        2 => "Bad",
        _ => "Unknown"
    });

    HttpResponse::Ok().json(get_new_meme(info.id))
}


// GET SOCIAL INFORMATION

#[derive(Serialize)]
struct Social {
    requests: Vec<User>,
    requested: Vec<User>,
    friends: Vec<User>
}

fn get_social(user : String) -> Social {
    Social {
        requests : Vec::new(),
        requested: Vec::new(),
        friends: Vec::new(),
    }
}

#[derive(Deserialize)]
struct SocialActionReq {
    user : String,
    id : i32,
    other : String,
    action : i32
}

#[get("/getsocial&user={user}&id={id}")]
async fn get_user_social(info : web::Path<UserAuth>) -> impl Responder {
    // check auth for id, username

    HttpResponse::Ok().json(get_social(info.user.clone()))
}

#[get("/getnewmatch&user={user}&id={id}")]
async fn get_new_match(info : web::Path<UserAuth>) -> impl Responder {
    // check out id

    // find a new match

    HttpResponse::Ok().json(get_social(info.user.clone()))
}

#[get("/socialaction&user={user}&id={id}&other={other}&action={action}")]
async fn social_action(info : web::Path<SocialActionReq>) -> impl Responder {
    // check auth for id, username

    // do action based on info.other, info.action
    println!("Got social action {}", match info.action {
        0 => "CancelRequest",
        1 => "AcceptRequested",
        2 => "RejectRequested",
        3 => "RemoveFriend",
        _ => "Error!!"
    });

    // get social information and return

    HttpResponse::Ok().json(get_social(info.user.clone()))
}





#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // initialize the logger
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::new("IP: %a | TIME: %t | REQUEST: %r | PROCESSED IN: %Dms"))
            .service(login_user)
            .service(get_meme)
            .service(react_meme)
            .service(get_user_social)
            .service(get_new_match)
            .service(social_action)
            .service(fs::Files::new("/memepoke", "./static").index_file("index.html"))
            .service(fs::Files::new("/", "./static/"))
        })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}