
use yew::prelude::*;

struct Meme {
    meme_id : i64,
    image_url : String,
    post_link : String,
    post_author : String,
}

pub enum MemeMessage {
    Like,
    Neutral,
    Dislike
}

pub struct MemePage {
    link : ComponentLink<Self>,
    display_meme : Meme
}

impl Component for MemePage {
    type Properties = ();
    type Message = MemeMessage;

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            display_meme: example_meme()
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            MemeMessage::Like => {println!("like");},
            MemeMessage::Neutral => {println!("neutral");},
            MemeMessage::Dislike => {println!("dislike");}
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class = "d-flex justify-content-center">
                <div class="card  mw-50 mh-50">
                    <img class="card-img-top rounded" src={self.display_meme.image_url.clone()} alt="Card image cap"/>
                    <div class="d-flex justify-content-center">
                        <h1>{"Yo is this meme cheese or what?"}</h1>
                    </div>
                    <div class="d-flex justify-content-center">
                        <div class="btn-group btn-group-lg" role="group" aria-label="Basic example">
                            <button type="button" class="btn btn-success">{"Fab"}</button>
                            <button type="button" class="btn btn-secondary">{"Meh"}</button>
                            <button type="button" class="btn btn-danger">{"Nah"}</button>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}


// example meme
fn example_meme() -> Meme {
    Meme {
        meme_id: 23875,
        image_url: String::from("https://i.redd.it/oyzhewq7uze71.jpg"),
        post_link: String::from("https://www.reddit.com/r/memes/comments/owlh9g/tell_me_how_that_air_con_was_working_again_ben/?utm_source=share&utm_medium=web2x&context=3"),
        post_author: String::from("ReeZedd")
    }
}