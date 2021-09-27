use yew::prelude::*;
use yew_router::{prelude::*, Switch};

#[path = "login.rs"]
mod login;

#[path = "utils.rs"]
mod utils;

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

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self { Self }

    fn update(&mut self, _: Self::Message) -> ShouldRender { false }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender { false }

    fn view(&self) -> Html {
        html! {
            <Router<SiteRoutes>
                render = Router::render(|switch: SiteRoutes| {
                    match switch {
                        SiteRoutes::MemePoke(state, code) => {
                            if state == "RANDOMSTRING" {
                                html! { <login::LoginPage code=code /> }
                            } else {
                                utils::view_error("Unexpected state returned on login")
                            }
                        },
                        SiteRoutes::Denied(state) => html! {
                            utils::view_error(format!("Memepoke reddit access denied: {}", state).as_str())
                        },
                        SiteRoutes::Login => html! {
                            <div class="bg-image" style="background-image: url('img/shrek.gif');height: 100vh">
                                <div class = "d-flex justify-content-center">
                                    <div class="card mh-50 w-25">
                                        <h1 class="h-5">{"MemePoke Reloaded!"}</h1>
                                        <img src="img/memepoke.png" class="img-fluid" alt="Responsive image"/>
                                        <p>{"Developed by Oliver Killane as the successor to MemePoke! from the IC Hello World Hackathon."}</p>
                                        <p>{"Made entirely in Rust using the Yew stack"}</p>
                                        <a class="btn btn-primary btn-lg btn-block" href="https://www.reddit.com/api/v1/authorize?client_id=WI5Q-r9Nu3EjSxIjz5SzEA&response_type=code&state=RANDOMSTRING&redirect_uri=http://127.0.0.1:8080/memepoke&duration=temporary&scope=identity">{"Login"}</a>
                                        <a class="btn btn-primary btn-lg btn-block" href="https://github.com/OliverKillane/memepoke-reloaded">{"Project Repo"}</a>
                                    </div>
                                </div>
                            </div>
                        }
                    }
                })
            />
        }
    }
}

fn main() {
    yew::start_app::<MainPage>();
}