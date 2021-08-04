use yew::prelude::*;

pub struct AboutPage;

impl Component for AboutPage {
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
        html!{
            //<div class="bg-image" style="background-image: url('img/shrek.gif');height: 100vh">
                <div class = "d-flex justify-content-center">
                    <div class="card mh-50 w-25">
                        <h1 class="h-5">{"MemePoke Reloaded!"}</h1>
                        <img src="img/memepoke.png" class="img-fluid" alt="Responsive image"/>
                        <p>{"Developed by Oliver Killane as the successor to MemePoke! from the IC Hello World Hackathon."}</p>
                        <p>{"Made entirely in Rust using the Yew stack"}</p>
                        <a class="btn btn-primary btn-lg btn-block" href="https://github.com/OliverKillane">{"Project Repo"}</a>
                        <a class="btn btn-primary btn-lg btn-block" href="">{"Return"}</a>
                    </div>
                </div>
            //</div>
        }
    }
}
