use serde::Deserialize;
use yew::{format::{Json, Nothing}, prelude::*, services::{ConsoleService, fetch::{FetchService, FetchTask, Request, Response}}};
use anyhow;
use serde_json::json;

#[path = "utils.rs"]
mod utils;

#[path = "memepoke.rs"]
mod memepoke;

#[path = "user.rs"]
mod user;
use user::*;


// Reddit API responses:
#[derive(Deserialize)]
struct AuthTokResp {
    access_token : String,
    token_type : String,
    expires_in : String,
    scope : String,
    refresh_token : String
}

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

// Login Page (can display error and loading page):
pub struct LoginPage {
    link : ComponentLink<Self>,
    fetch_task : Option<FetchTask>,
    state : LoginMsg
}

#[derive(Properties, Clone)]
pub struct AuthCode {
    pub code: String
}

pub enum LoginMsg {
    Start,
    GotToken(String, String),
    GotUser(User),
    Failure(String)
}

impl Component for LoginPage {
    type Properties = AuthCode;
    type Message = LoginMsg;
    
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let ft = get_auth_code(props.code, &link);
        Self {
            link, 
            fetch_task : Some(ft),
            state : LoginMsg::Start
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        self.state = msg;
        self.fetch_task = match &self.state {
            LoginMsg::GotToken(authToken, RefreshToken) => Some(get_user(authToken.clone(), &self.link)),
            _ => None,
        };
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender { false }

    fn view(&self) -> Html {
        match &self.state {
            LoginMsg::Start => utils::view_loading("Getting Authentication Token"),
            LoginMsg::GotToken(_,_) => utils::view_loading("Getting Account Details"),
            LoginMsg::Failure(error) => utils::view_error(error),
            LoginMsg::GotUser(user) => 
                html! {
                    <memepoke::MemePokePage username=user.username.clone() id=user.id.clone() description=user.description.clone() profile_pic_url=user.profile_pic_url.clone() auth_token=user.auth_token.clone()/>
                }
        }
    }
}

fn get_auth_code(code: String, link: &ComponentLink<LoginPage>) -> FetchTask {

    let body = json!({
        "grant_type" : "authorization_code",
        "code" : code,
        "redirect_uri" : utils::redirect_uri
    }); 

    let get_token_req = 
        Request::post("https://www.reddit.com/api/v1/access_token")
            .header("Access-Control-Allow-Origin", "*")
            .header("Access-Control-Allow-Methods", "GET,HEAD,OPTIONS,POST,PUT")
            .header("Access-Control-Allow-Headers", "Origin, X-Requested-With, Content-Type, Accept, Authorization")
            .header("User-Agent", "MemePoke/2.0.0 (from client browser)")
            .header("Authorization", format!("Basic {}:", utils::client_id))
            .body(Json(&body))
            .unwrap();

    let callback = link.callback(|response: Response<Json<anyhow::Result<AuthTokResp>>>| {
        ConsoleService::info(&*format!("get auth code: {}", response.status().as_str()));
        if let Json(Ok(data)) = response.into_body() {
            LoginMsg::GotToken(data.access_token, data.refresh_token)
        } else {
            LoginMsg::Failure(String::from(format!("Unable to get access token")))
        }
    });

    FetchService::fetch(get_token_req, callback).unwrap()
}

fn get_user(token : String, link : &ComponentLink<LoginPage>) -> FetchTask {
    let get_username_req = 
        Request::get("https://oauth.reddit.com/api/v1/me")
            .header("Authorization", format!("bearer {}", token.clone()))
            .body(Nothing)
            .unwrap();
        
    let callback = link.callback(move |response: Response<Json<anyhow::Result<UserResp>>>| {
        if let Json(Ok(data)) = response.into_body() {
            LoginMsg::GotUser(User {
                username : data.name,
                id : data.id,
                description : None,
                profile_pic_url : String::from("https://www.redditstatic.com/avatars/avatar_default_10_FF585B.png"),
                auth_token : token.clone()
            })
        } else {
            LoginMsg::Failure(String::from("Unable to get user data"))
        }
    });

    FetchService::fetch(get_username_req, callback).unwrap()
}