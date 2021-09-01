use yew::{format::{Json, Nothing}, prelude::*, services::{fetch::{FetchService, FetchTask, Request, Response}}};
use anyhow;

#[path = "utils.rs"]
mod utils;
use utils::{view_error, view_loading};

#[path = "memepoke.rs"]
mod memepoke;

#[path = "user.rs"]
mod user;
use user::User;

#[derive(Properties, Clone)]
pub struct AuthCode {
    pub code : String
}

// Login Page (can display error and loading page):
pub struct LoginPage {
    fetch_task : Option<FetchTask>,
    state : LoginMsg
}

pub enum LoginMsg {
    Loading,
    GotUser(User),
    Failure
}

impl Component for LoginPage {
    type Properties = AuthCode;
    type Message = LoginMsg;
    
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let ft = get_user(props.code, &link);

        Self {
            fetch_task : Some(ft),
            state : LoginMsg::Loading
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        self.state = msg;
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender { false }

    fn view(&self) -> Html {
        match &self.state {
            LoginMsg::Loading => view_loading("Getting user details"),
            LoginMsg::Failure => view_error("Failed to get fetch user details."),
            LoginMsg::GotUser(user) => 
                html! {
                    <memepoke::MemePokePage username=user.username.clone() id = user.id description=user.description.clone() profile_pic_url=user.profile_pic_url.clone() auth_token=user.auth_token.clone()/>
                }
        }
    }
}

pub fn get_user(code: String, link: &ComponentLink<LoginPage>) -> FetchTask {
    let req = Request::get(format!("{}/loginuser&code={}", utils::site_uri, code)).body(Nothing).unwrap();

    let callback = link.callback(|response: Response<Json<anyhow::Result<User>>>| {
        if let Json(Ok(data)) = response.into_body() {
            LoginMsg::GotUser(data)
        } else {
            LoginMsg::Failure
        }
    });

    FetchService::fetch(req, callback).unwrap()
}