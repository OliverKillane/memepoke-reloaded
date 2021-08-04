use yew::{prelude::*, props};
use yew_router::{Switch, prelude::*, switch::Permissive};


#[path = "pages/about.rs"]
mod about;

#[path = "pages/error.rs"]
mod error;

#[path = "pages/memepoke.rs"]
mod memepoke;

#[derive(Switch, Clone)]
enum SiteRoutes {
    #[to = "/about"]
    About,
    #[to = "/memepoke?state={}&code={}#_"]
    MemePoke(String, String),
    #[to = "/mempoke?state={}&error=access_denied#_"]
    Denied(String),
    #[to = "/"]
    Login,
    PageNotFound(Permissive<String>),
}

// the main page
struct MainPage{
    link: ComponentLink<Self>,
    state: String
}

impl Component for MainPage {
    type Properties = ();
    type Message = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            state: String::from("hey")
        }
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
                        SiteRoutes::About => html! {<about::AboutPage/>},
                        SiteRoutes::MemePoke(state, code) => {
                            if true { //place the condition for state here
                                html! { <p>
                                    {format!("hello the state is {} and the code is {}", state, code)} </p> }
                            } else {
                                html! { <error::ErrorPage/> }
                            }
                        },
                        SiteRoutes::Denied(state) => html! {
                            <p> {format!("hello the state is {} and the code is", state)} </p>
                        },
                        SiteRoutes::Login => html!{
                            <p> {format!("hello this is login")} </p>
                        },
                        SiteRoutes::PageNotFound(Permissive(None)) => html! { <p>
                            {"hello non permissive page not found"} </p>
                        },
                        SiteRoutes::PageNotFound(Permissive(Some(invalid_route))) => html! {
                            <>
                                <p> {format!("You this is an invalid route {}", invalid_route)}</p>
                            </>
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