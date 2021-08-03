use yew::prelude::*;


pub struct PokePage {
    link: ComponentLink<Self>,
    user: i64,
    requests: Vec<i64>,
    requested: Vec<i64>,
    friends: Vec<i64>
}

pub enum PokeMessage {
    Update,
    GetMatch
}

impl Component for PokePage {
    type Properties = ();
    type Message = PokeMessage;

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let (requests, requested, friends) = get_social(0);
        Self {
            link,
            user : 0,
            requests,
            requested,
            friends
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            PokeMessage::GetMatch => {
                self.get_match();
                true
            },
            PokeMessage::Update => true
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html!{
            <>
                <h1>{"Sent Requests:"}</h1>
                <div class="d-flex flex-wrap align-content-stretch">
                    <div class="card" style="width: 18rem;">
                        <img class="card-img-top" src="img/question_mark.png" alt="Card image cap" style="height: 18rem;" />
                        <div class="card-body">
                            <h5 class="card-title">{"Get a new Match"}</h5>
                            <p class="card-text">{"Find someone with the same good taste in der memes"}</p>
                            <button onclick={self.link.callback(|_| PokeMessage::GetMatch)}type="button" class="btn btn-success">{"Get"}</button>
                        </div>
                    </div>

                    {
                        self.requests.iter().map(|_|
                            html! {
                                <div class="card" style="width: 18rem;">
                                    <img class="card-img-top" src="img/reddit_profile.png" alt="Card image cap" style="height: 18rem;"/>
                                    <div class="card-body">
                                        <h5 class="card-title">{"Reddit User"}</h5>
                                        <p class="card-text">{"Also liked that meme"}</p>
                                        <button type="button" class="btn btn-danger">{"cancel"}</button>
                                    </div>
                                </div>
                            }
                        ).collect::<Html>()
                    }
                </div>

                <h1>{"Recieved Requests:"}</h1>

                <div class="d-flex flex-wrap align-content-stretch">
                    {
                        self.requested.iter().map(|_| 
                            html! {
                                <div class="card" style="width: 18rem;">
                                    <img class="card-img-top" src="img/reddit_profile.png" alt="Card image cap" style="height: 18rem;"/>
                                    <div class="card-body">
                                    <h5 class="card-title">{"Reddit User"}</h5>
                                    <p class="card-text">{"Also liked that meme"}</p>
                                    <div class="btn-group btn-group-lg" role="group" aria-label="Basic example">
                                        <button type="button" class="btn btn-success">{"accept"}</button>
                                        <button type="button" class="btn btn-danger">{"ignore"}</button>
                                    </div>
                                    </div>
                                </div>
                            }
                        ).collect::<Html>()
                    }
                </div>

                <h1>{"Friends:"}</h1>

                <div class="d-flex flex-wrap align-content-stretch">
                    {
                        self.friends.iter().map(|_| 
                            html! {
                                <div class="card" style="width: 18rem;">
                                    <img class="card-img-top" src="img/reddit_profile.png" alt="Card image cap" style="height: 18rem;"/>
                                    <div class="card-body">
                                        <h5 class="card-title">{"Reddit User"}</h5>
                                        <p class="card-text">{"Also liked that meme"}</p>
                                        <div class="btn-group btn-group-lg" role="group" aria-label="Basic example">
                                            <button type="button" class="btn btn-primary">{"Chat"}</button>
                                            <button type="button" class="btn btn-secondary">{"Remove"}</button>
                                        </div>
                                    </div>
                                </div>
                            }
                        ).collect::<Html>()
                    }
                </div>
            </>
        }
    }
}

impl PokePage {
    fn get_match(&mut self) {
        // code for the matcher
        self.requests.push(0);
    }
}

fn get_social(user: i64) -> (Vec<i64>,Vec<i64>,Vec<i64>) {
    // get requests, requested, friends
    (vec![0;0],vec![0;5],vec![0;7])
}
