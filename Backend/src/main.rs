use actix_web::{App, HttpResponse, HttpServer, Responder, get, middleware::Logger, web};
use serde::{Deserialize,Serialize};
use env_logger::Env;

#[derive(Deserialize)]
struct UserReq {
    username : String,
    authtoken : String
}

// UPDATE USER DESCRIPTION:
#[derive(Deserialize)]
struct UpdateDescrReq {
    username : String,
    authtoken : String,
    descr : String
}

#[get("/updatedescr&user={username}&token={authtoken}&descr={descr}")]
async fn update_user_descr(info : web::Path<UpdateDescrReq>) -> impl Responder {
    HttpResponse::Ok()
}


// LOGIN THE USER:
#[get("/loginuser&user={username}&token={authtoken}")]
async fn login_user(info : web::Path<UserReq>) -> impl Responder {
    HttpResponse::Ok().body("This is the user description")
}


// GET USER SOCIAL INFORMATION:
#[derive(Serialize)]
struct User {
    pub username: String,
    pub id: String,
    pub description: Option<String>,
    pub profile_pic_url: String,
}

#[derive(Serialize)]
struct Social {
    requests : Vec<User>,
    requested : Vec<User>,
    friends : Vec<User>
}


#[get("/getsocial&user={username}&token={authtoken}")]
async fn get_user_social(info : web::Path<UserReq>) -> impl Responder {

    //**************************************************************************
    let placeholder = User {
        username: String::from("Username Here"),
        id: String::from("Id here"),
        description: Some(String::from("This is the description of the user")),
        profile_pic_url: String::from("https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcTAs_TDUTeHiZQ1tqLJlvItaBOjcmRTeoSbHw&usqp=CAU"),
    };

    let example_social = Social {
        requests: vec![placeholder],
        requested: vec![],
        friends: vec![],
    };
    //**************************************************************************

    HttpResponse::Ok().json( example_social )
}

// DO USER ACTION
#[derive(Deserialize)]
struct SetSocialReq {
    username : String,
    authtoken : String,
    action : String,
    otheruser : String
}

#[get("/setsocial&user={username}&token={authtoken}&action={action}&other={otheruser}")]
async fn set_user_social(info : web::Path<SetSocialReq>) -> impl Responder {
    HttpResponse::Ok()
}

// GET USER NEW MATCH
#[get("/getmatch&user={username}&token={authtoken}")]
async fn get_match(info : web::Path<UserReq>) -> impl Responder {
    HttpResponse::Ok()
}

// GET USER NEW MEME
#[derive(Serialize)]
struct Meme {
    id: u64,
    post : String,
    image_url: String,
    original_poster: String
}

#[get("/getmeme&user={username}&token={authtoken}")]
async fn get_meme(info : web::Path<UserReq>) -> impl Responder {

    //**************************************************************************
    let placeholder = Meme {
        id: 111,
        post: String::from("https://www.reddit.com/r/Shrekmemes/comments/oxipnn/when_the_villagers_dont_find_onions/"),
        image_url: String::from("https://i.redd.it/jp1bitzz29f71.jpg"),
        original_poster: String::from("Cwf97"),
    };

    //**************************************************************************

    HttpResponse::Ok().json( placeholder )
}

// USER REACT TO MEME
struct MemeReactReq {
    username : String,
    authtoken : String,
}

// CHECKING USER:

#[derive(Deserialize)]
struct UserResp {
    comment_karma : i32,
    created : String,
    created_utc : String,
    has_mail : String,
    has_mod_mail : bool,
    has_verified_email : String,
    id : String,
    is_gold : bool,
    is_mod : bool,
    link_karma : i32,
    name : String,
    over_18 : bool
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // initialize the logger
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    HttpServer::new(|| {
        App::new()
        .wrap(Logger::new("IP: %a | TIME: %t | REQUEST: %r | PROCESSED IN: %Dms"))
        .service(get_meme)
        .service(get_match)
        .service(set_user_social)
        .service(get_user_social)
        .service(login_user)
        .service(update_user_descr)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}