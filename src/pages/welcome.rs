use yew::prelude::*;


pub struct WelcomePage;

impl Component for WelcomePage {
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
                    </div>
                </div>
            //</div>
        }
    }
}
