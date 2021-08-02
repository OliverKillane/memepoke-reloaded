use yew::prelude::*;

// The pages files to be included
#[path ="./pages/welcome.rs"]
mod welcome;

#[path ="./pages/meme.rs"]
mod meme;


enum MainState {
    MemePage,
    PokePage,
    WelcomePage,
}

struct MainPage {
    link: ComponentLink<Self>,
    state: MainState
}

impl Component for MainPage {
    type Properties = ();
    type Message = MainState;

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            state: MainState::WelcomePage
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
        html! { 
            <>
                <nav class="class=navbar fixed-top navbar-light bg-light">
                    <div class="d-flex text-white">
                        <div class="p-2 w-100">
                            <div class="d-flex justify-content-center">
                                <div class="btn-group btn-group-toggle" data-toggle="buttons">
                                    {
                                        match self.state {
                                            MainState::MemePage => html!{
                                                <>
                                                    <button type="button" class="btn btn-primary">{"Meme"}</button>
                                                    <button onclick={self.link.callback(|_| MainState::PokePage)} type="button" class="btn btn-outline-primary">{"Poke"}</button>
                                                </>
                                            },
                                            MainState::PokePage =>  html!{
                                                <>
                                                    <button onclick={self.link.callback(|_| MainState::MemePage)} type="button" class="btn btn-outline-primary">{"Meme"}</button>
                                                    <button type="button" class="btn btn-primary">{"Poke"}</button>
                                                </>
                                            },
                                            MainState::WelcomePage =>  html!{
                                                <>
                                                    <button type="button" class="btn btn-outline-primary">{"Meme"}</button>
                                                    <button type="button" class="btn btn-outline-primary">{"Poke"}</button>
                                                </>
                                            }
                                        }
                                    }
                                </div>
                            </div>
                        </div>
                        <div class="p-2 flex-shrink-0">
                            {if let MainState::WelcomePage = self.state 
                                {html!{<button onclick={self.link.callback(|_| MainState::MemePage)} type="button" class="btn btn-success">{"Login"}</button>}} 
                            else 
                                {html!{<button onclick={self.link.callback(|_| MainState::WelcomePage)} type="button" class="btn btn-danger">{"Logout"}</button>}}}
                        </div>
                    </div>
                </nav>
                {match self.state {
                    MainState::WelcomePage => html!{ <welcome::WelcomePage/> },
                    MainState::MemePage => html!{ <meme::MemePage/> },
                    MainState::PokePage => html!{ {"This is the poke page yo"} },
                }}
            </>
        }
    }
}


fn main() {
    yew::start_app::<MainPage>();
}