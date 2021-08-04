use yew::prelude::*;


// including necessary page components
#[path ="about.rs"]
mod about;

// user for sending details to and receiving from the server
#[path = "../user.rs"]
mod user;

#[derive(Properties, Clone)]
pub struct AuthCode {
    code : String
}

pub enum MemePokeState {
    MemePage,
    PokePage,
    AboutPage,
    AccountPage
}

pub struct MemePokePage {
    link: ComponentLink<Self>,
    state: MemePokeState,
    user: user::User,
}

impl Component for MemePokePage {
    type Properties = AuthCode;
    type Message = MemePokeState;

    fn create(auth_code: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            state: MemePokeState::MemePage,
            user: user::User::get_user(auth_code.code)
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        self.state = msg;
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! { }
    }
}