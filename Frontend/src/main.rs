use yew::{prelude::*};
use yew_router::{Switch, prelude::*};

#[path = "pages/login.rs"]
mod login;

#[path = "pages/error.rs"]
mod error;

#[path = "pages/memepoke.rs"]
mod memepoke;

#[derive(Switch, Clone)]
enum SiteRoutes {
    #[to = "/memepoke?state={}&error=access_denied#_"]
    Denied(String),
    #[to = "/memepoke?state={}&code={}#_"]
    MemePoke(String, String),
    #[to = "/"]
    Login,
}

// the main page
struct MainPage;

impl Component for MainPage {
    type Properties = ();
    type Message = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <Router<SiteRoutes>
                render = Router::render(|switch: SiteRoutes| {
                    match switch {
                        SiteRoutes::MemePoke(state, code) => {
                            if state == "RANDOMSTRING" { //place the condition for state here
                                html! { <memepoke::MemePokePage code=code /> }
                            } else {
                                html! { <error::ErrorPage/> }
                            }
                        },
                        SiteRoutes::Denied(state) => html! {
                            <p> {format!("hello the state is {} and the code is", state)} </p>
                        },
                        SiteRoutes::Login => html!{ <login::LoginPage/> }
                    }
                })
            />
        }
    }
}


fn main() {
    yew::start_app::<MainPage>();
}

/*

reddit:
appID: WI5Q-r9Nu3EjSxIjz5SzEA
direct to url: https://www.reddit.com/api/v1/authorize?client_id=WI5Q-r9Nu3EjSxIjz5SzEA&response_type=code&state=RANDOMSTRING&redirect_uri=http://127.0.0.1:8080/memepoke&duration=temporary&scope=identity

redirects:
http://127.0.0.1/memepoke?state=RANDOMSTRING&code=rx-5Ps6T60jF1UfbhS2QIsuTYHqXJw#_
http://127.0.0.1/memepoke?state=RANDOMSTRING&error=access_denied#_

*/