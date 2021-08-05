use yew::prelude::*;

// user for sending details to and receiving from the server
#[path = "../structs/user.rs"]
mod user;
use user::*;

#[path = "../structs/meme.rs"]
mod meme;
use meme::*;

#[derive(Properties, Clone)]
pub struct AuthCode {
    pub code : String
}

pub enum MemePokeState {
    MemePage(MemePageMsg),
    PokePage(PokePageMsg),
    AccountPage(AccountPageMsg)
}

enum MemePageMsg {
    Update,
    Like,
    Neutral,
    Dislike
}

enum PokePageMsg {
    Update,
    GetNewMatch,
    CancelRequest(String),
    AcceptRequested(String),
    RejectRequested(String),
    RemoveFriend(String)
}

enum AccountPageMsg {
    Update,
    UpdateDescription(String)
}

struct Social {
    requests: Vec<User>,
    requested: Vec<User>,
    friends: Vec<User>
}

pub struct MemePokePage {
    link: ComponentLink<Self>,
    state: MemePokeState,
    user: User,
    meme: Meme,
    social: Social
    
}

//for testing:
fn getmeme() -> Meme {
    Meme {
        post : String::from("When you realise the end of the world is made in china"),
        image_url: String::from("https://static.toiimg.com/photo/74674393.cms"),
        original_poster: String::from("Shrek")
    }
}


impl Component for MemePokePage {
    type Properties = AuthCode;
    type Message = MemePokeState;

    fn create(auth_code: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            state: MemePokeState::MemePage(MemePageMsg::Update),
            user: User::get_user(auth_code.code),
            meme: getmeme(),
            social: Social { requests: vec![], requested: vec![], friends: vec![] }
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        self.state = msg;
        // match &self.state {
        //     MemePokeState::MemePage(action) => {
        //         match action {
        //             MemePageMsg::Update => todo!(),
        //             MemePageMsg::Like => todo!(),
        //             MemePageMsg::Neutral => todo!(),
        //             MemePageMsg::Dislike => todo!(),
        //         }
        //     },
        //     MemePokeState::PokePage(action) => {
        //         match action {
        //             PokePageMsg::Update => todo!(),
        //             PokePageMsg::GetNewMatch => todo!(),
        //             PokePageMsg::CancelRequest(_) => todo!(),
        //             PokePageMsg::AcceptRequested(_) => todo!(),
        //             PokePageMsg::RejectRequested(_) => todo!(),
        //             PokePageMsg::RemoveFriend(_) => todo!(),
        //         }
        //     },
        //     MemePokeState::AccountPage(action) => {
        //         match action {
        //             AccountPageMsg::Update => todo!(),
        //             AccountPageMsg::UpdateDescription(_) => todo!(),
        //         }
        //     },
        // };
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div style=  "padding-top: 65px;">
                <nav class="class=navbar fixed-top navbar-light bg-light">
                    <div class="d-flex text-white">
                        <div class="p-2 flex-shrink-0">
                            {
                                if let MemePokeState::AccountPage(_) = self.state{
                                    html! { <button type="button" class="btn btn-primary">{"Profile"}</button> }
                                } else {
                                    html! { <button onclick={self.link.callback(|_| MemePokeState::AccountPage(AccountPageMsg::Update))} type="button" class="btn btn-outline-primary">{"Profile"}</button> }
                                }
                            }
                        </div>
                        <div class="p-2 w-100">
                            <div class="d-flex justify-content-center">
                                <div class="btn-group btn-group-toggle" data-toggle="buttons">
                                    {
                                        match self.state {
                                            MemePokeState::MemePage(_) => html!{
                                                <>
                                                    <button type="button" class="btn btn-primary">{"Meme"}</button>
                                                    <button onclick={self.link.callback(|_| MemePokeState::PokePage(PokePageMsg::Update))} type="button" class="btn btn-outline-primary">{"Poke"}</button>
                                                </>
                                            },
                                            MemePokeState::PokePage(_) =>  html!{
                                                <>
                                                    <button onclick={self.link.callback(|_| MemePokeState::MemePage(MemePageMsg::Update))} type="button" class="btn btn-outline-primary">{"Meme"}</button>
                                                    <button type="button" class="btn btn-primary">{"Poke"}</button>
                                                </>
                                            },
                                            MemePokeState::AccountPage(_) =>  html!{
                                                <>
                                                    <button onclick={self.link.callback(|_| MemePokeState::MemePage(MemePageMsg::Update))} type="button" class="btn btn-outline-primary">{"Meme"}</button>
                                                    <button onclick={self.link.callback(|_| MemePokeState::PokePage(PokePageMsg::Update))} type="button" class="btn btn-outline-primary">{"Poke"}</button>
                                                </>
                                            }
                                        }
                                    }
                                </div>
                            </div>
                        </div>
                        <div class="p-2 flex-shrink-0">
                            <a href="#" type="button" class="btn btn-danger">{"Logout"}</a>
                        </div>
                    </div>
                </nav>
                <div>
                    {
                        match self.state {
                            MemePokeState::MemePage(_) => html! {
                                <div class = "d-flex justify-content-center">
                                    <div class="card  mw-50 mh-50">
                                        <img class="card-img-top rounded" src={self.meme.image_url.clone()} alt="Card image cap"/>
                                        <div class="d-flex justify-content-center">
                                            <h1>{"Yo is this meme cheese or what?"}</h1>
                                        </div>
                                        <div class="d-flex justify-content-center">
                                            <div class="btn-group btn-group-lg" role="group" aria-label="Basic example">
                                                <button onclick={self.link.callback(|_| MemePokeState::MemePage(MemePageMsg::Like))} type="button" class="btn btn-success">{"Fab"}</button>
                                                <button onclick={self.link.callback(|_| MemePokeState::MemePage(MemePageMsg::Neutral))} type="button" class="btn btn-secondary">{"Meh"}</button>
                                                <button onclick={self.link.callback(|_| MemePokeState::MemePage(MemePageMsg::Dislike))} type="button" class="btn btn-danger">{"Nah"}</button>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            },
                            MemePokeState::PokePage(_) => html! {
                                <>
                                    <h1>{"Sent Requests:"}</h1>
                                    <div class="d-flex flex-wrap align-content-stretch">
                                        <div class="card" style="width: 18rem;">
                                            <img class="card-img-top" src="img/question_mark.png" alt="Card image cap" style="height: 18rem;" />
                                            <div class="card-body">
                                                <h5 class="card-title">{"Get a new Match"}</h5>
                                                <p class="card-text">{"Find someone with the same good taste in der memes"}</p>
                                                <button onclick={self.link.callback(|_| MemePokeState::PokePage(PokePageMsg::GetNewMatch))}type="button" class="btn btn-success">{"Get"}</button>
                                            </div>
                                        </div>

                                        {
                                            self.social.requests.iter().map(|request|
                                                html! {
                                                    <div class="card" style="width: 18rem;">
                                                        <img class="card-img-top" src={request.profile_pic_url.clone()} alt="Card image cap" style="height: 18rem;"/>
                                                        <div class="card-body">
                                                            <h5 class="card-title">{&request.username}</h5>
                                                            <p class="card-text">{&request.description}</p>
                                                            //<button onclick={self.link.callback(|_| MemePokeState::PokePage(PokePageMsg::CancelRequest(request.username)))} type="button" class="btn btn-danger">{"cancel"}</button>
                                                        </div>
                                                    </div>
                                                }
                                            ).collect::<Html>()
                                        }
                                    </div>

                                    <h1>{"Recieved Requests:"}</h1>

                                    <div class="d-flex flex-wrap align-content-stretch">
                                        {
                                            // I am in no way proud, or happy with this closure:
                                            // as request &Self has '_ and we need 'static for callback, create a new var with the username copied, move into closure, as a redult by inference, lifetime required by callback closure and the new variable are the same?
                                            // There must be a simpler way to -> e,g make Self 'Static but I am not confident enough with lifetimes to try yet
                                            // Hence the create var -> move closure -> clone mess you see below:
                                            // I am very tired, will fix tommorow
                                            self.social.requested.iter().map(|requested| {  
                                                let friend = requested.username.clone();
                                                let friend2 = requested.username.clone();
                                                html! {
                                                    <div class="card" style="width: 18rem;">
                                                        <img class="card-img-top" src={requested.profile_pic_url.clone()} alt="Card image cap" style="height: 18rem;"/>
                                                        <div class="card-body">
                                                        <h5 class="card-title">{&requested.username}</h5>
                                                        <p class="card-text">{&requested.description}</p>
                                                        <div class="btn-group btn-group-lg" role="group" aria-label="Basic example">
                                                            <button onclick={self.link.callback(move |_| MemePokeState::PokePage(PokePageMsg::AcceptRequested(friend.clone())))} type="button" class="btn btn-success">{"accept"}</button>
                                                            <button onclick={self.link.callback(move |_| MemePokeState::PokePage(PokePageMsg::RejectRequested(friend2.clone())))} type="button" class="btn btn-danger">{"ignore"}</button>
                                                        </div>
                                                        </div>
                                                    </div>
                                                }
                                            }
                                            ).collect::<Html>()
                                        }
                                    </div>

                                    <h1>{"Friends:"}</h1>

                                    <div class="d-flex flex-wrap align-content-stretch">
                                        {
                                            self.social.friends.iter().map(|friend| 
                                                html! {
                                                    <div class="card" style="width: 18rem;">
                                                        <img class="card-img-top" src={friend.profile_pic_url.clone()} alt="Card image cap" style="height: 18rem;"/>
                                                        <div class="card-body">
                                                            <h5 class="card-title">{&friend.username}</h5>
                                                            <p class="card-text">{&friend.description}</p>
                                                            <div class="btn-group btn-group-lg" role="group" aria-label="Basic example">
                                                                <a type="button" class="btn btn-primary">{"Chat"}</a>
                                                                //<button onclick={self.link.callback(|_| MemePokeState::PokePage(PokePageMsg::RemoveFriend(friend.username)))} type="button" class="btn btn-secondary">{"Remove"}</button>
                                                            </div>
                                                        </div>
                                                    </div>
                                                }
                                            ).collect::<Html>()
                                        }
                                    </div>
                                </>
                                
                            },
                            MemePokeState::AccountPage(_) => html! {
                                <div class = "d-flex justify-content-center">
                                    <div class="card mh-50 w-25">
                                        <h1 class="h-5">{"MemePoke Reloaded!"}</h1>
                                        <img src={self.user.profile_pic_url.clone()} class="img-fluid" alt="Responsive image"/>
                                        <div class="input-group">
                                            <div class="input-group-prepend">
                                                <button class="btn btn-success" type="button">{"Update"}</button>
                                            </div>
                                            <input type="text" class="form-control" placeholder={self.user.description.clone()} aria-label="" aria-describedby="basic-addon1"/>
                                        </div>
                                    </div>
                                </div>
                            },
                        }
                    }
                </div>
            </div>
        }
    }

    fn rendered(&mut self, _first_render: bool) {}

    fn destroy(&mut self) {}
}