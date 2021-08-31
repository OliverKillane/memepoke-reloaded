use yew::prelude::*;

// user for sending details to and receiving from the server
#[path = "utils.rs"]
mod utils;

#[path = "user.rs"]
mod user;
use user::User;

#[derive(Clone)]
pub struct Meme {
    pub id: u64,
    pub post : String,
    pub image_url: String,
    pub original_poster: String
}

#[derive(Clone, Copy)]
pub enum Reaction {
    Like,
    Neutral,
    Dislike
}

#[derive(Clone, Copy)]
pub enum SocialAction {
    CancelRequest,
    AcceptRequested,
    RejectRequested,
    RemoveFriend
}

// states for the gui
pub enum MemePokeState {
    MemePage(MemeState),
    PokePage(PokeState),
    AccountPage(AccountState)
}

pub enum MemeState {
    GetMeme,
    React(Reaction, Meme),
    Display(Meme),
}

pub enum PokeState {
    GetSocial,
    GetNewMatch,
    Action(SocialAction, String),
    Display(Social),
}

pub enum AccountState{
    GetDescr,
    UpdateDescr,
    Display,
}


// social Struct to contain user info
struct Social {
    requests: Vec<User>,
    requested: Vec<User>,
    friends: Vec<User>
}


// Memepoke Component Struct
pub struct MemePokePage {
    link: ComponentLink<Self>,
    state: MemePokeState,
    user: User,
    meme: Option<Meme>,
    social: Option<Social>,
    descr_box : NodeRef
}

impl Component for MemePokePage {
    type Properties = User;
    type Message = MemePokeState;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            state: MemePokeState::MemePage(MemeState::GetMeme),
            user : User { username: props.username, description: props.description, profile_pic_url: props.profile_pic_url, auth_token: props.auth_token },
            meme: None,
            social: None,
            descr_box: NodeRef::default()
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        self.state = msg;

        match &self.state {
            MemePokeState::MemePage(state) => 
                match state {
                    MemeState::GetMeme => todo!(),
                    MemeState::React(_,_) => todo!(),
                    MemeState::Display(_) => todo!(),
                }
            ,
            MemePokeState::PokePage(state) => {
                match state {
                    PokeState::Action(_, _) => todo!(),
                    PokeState::Display(_) => todo!(),
                    PokeState::GetSocial => todo!(),
                    PokeState::GetNewMatch => todo!(),
                }
            },
            MemePokeState::AccountPage(state) => {
                match state {
                    AccountState::GetDescr => todo!(),
                    AccountState::UpdateDescr => todo!(),
                    AccountState::Display => todo!(),
                }
            },
        };
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div style=  "padding-top: 65px;">
                {self.view_navbar()}
                <div>
                    {
                        match &self.state {
                            MemePokeState::MemePage(state) => 
                                match state {
                                    MemeState::GetMeme => utils::view_loading("Getting Meme"),
                                    MemeState::React(reaction, _) => utils::view_loading(
                                        &*format!(
                                            "Reacting {} to Meme",
                                            match reaction {
                                                Reaction::Like =>  "👍",
                                                Reaction::Neutral => "🤔",
                                                Reaction::Dislike => "👎"
                                            }
                                        )
                                    ),
                                    MemeState::Display(meme) => self.view_memepage(meme)
                                }
                            ,
                            MemePokeState::PokePage(state) => {
                                match state {
                                    PokeState::GetNewMatch => utils::view_loading("Getting a new match"),
                                    PokeState::Action(action, user) => utils::view_loading(
                                        &*format!(
                                            "Sending {} to {}", 
                                            match action {
                                                SocialAction::CancelRequest => "cancel request",
                                                SocialAction::AcceptRequested => "request acceptance",
                                                SocialAction::RejectRequested => "request rejection",
                                                SocialAction::RemoveFriend => "friend removal"
                                            },
                                            user
                                        )
                                    ),
                                    PokeState::GetSocial => utils::view_loading("Getting Social Information"),
                                    PokeState::Display(social) => self.view_pokepage(social), // display poke page here
                                }
                            },
                            MemePokeState::AccountPage(state) => {
                                match state {
                                    AccountState::GetDescr => utils::view_loading("Loading Account Info"),
                                    AccountState::UpdateDescr => utils::view_loading("Sending updated description"),
                                    AccountState::Display => todo!(),
                                }
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

impl MemePokePage {

    fn view_navbar(&self) -> Html {
        html! {
            <nav class="class=navbar fixed-top navbar-light bg-light">
                <div class="d-flex text-white">
                    <div class="p-2 flex-shrink-0">
                        {
                            if let MemePokeState::AccountPage(_) = self.state{
                                html! { <button type="button" class="btn btn-primary">{"Profile"}</button> }
                            } else {
                                html! { <button onclick={self.link.callback(|_| MemePokeState::AccountPage(AccountState::GetDescr))} type="button" class="btn btn-outline-primary">{"Profile"}</button> }
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
                                                <button onclick={self.link.callback(|_| MemePokeState::PokePage(PokeState::GetSocial))} type="button" class="btn btn-outline-primary">{"Poke"}</button>
                                            </>
                                        },
                                        MemePokeState::PokePage(_) =>  html!{
                                            <>
                                                <button onclick={self.link.callback(|_| MemePokeState::MemePage(MemeState::GetMeme))} type="button" class="btn btn-outline-primary">{"Meme"}</button>
                                                <button type="button" class="btn btn-primary">{"Poke"}</button>
                                            </>
                                        },
                                        MemePokeState::AccountPage(_) =>  html!{
                                            <>
                                                <button onclick={self.link.callback(|_| MemePokeState::MemePage(MemeState::GetMeme))} type="button" class="btn btn-outline-primary">{"Meme"}</button>
                                                <button onclick={self.link.callback(|_| MemePokeState::PokePage(PokeState::GetSocial))} type="button" class="btn btn-outline-primary">{"Poke"}</button>
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
        }
    }

    fn view_pokepage(&self, social : &Social) -> Html {
        html! {
            <>
                <h1>{"Sent Requests:"}</h1>
                <div class="d-flex flex-wrap align-content-stretch">
                    <div class="card" style="width: 18rem;">
                        <img class="card-img-top" src="img/question_mark.png" alt="Card image cap" style="height: 18rem;" />
                        <div class="card-body">
                            <h5 class="card-title">{"Get a new Match"}</h5>
                            <p class="card-text">{"Find someone with the same good taste in der memes"}</p>
                            <button onclick={self.link.callback(|_| MemePokeState::PokePage(PokeState::GetNewMatch))}type="button" class="btn btn-success">{"Get"}</button>
                        </div>
                    </div>

                    {
                        social.requests.iter().map(|request| {
                            let req_str = request.username.clone();
                            html! {
                                <div class="card" style="width: 18rem;">
                                    <img class="card-img-top" src={request.profile_pic_url.clone()} alt="Card image cap" style="height: 18rem;"/>
                                    <div class="card-body">
                                        <h5 class="card-title">{&request.username}</h5>
                                        <p class="card-text">{&request.description.clone()}</p>
                                        <button onclick={self.link.callback(move |_| MemePokeState::PokePage(PokeState::Action(SocialAction::CancelRequest, req_str.clone())))} type="button" class="btn btn-danger">{"cancel"}</button>
                                    </div>
                                </div>
                            }
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
                        social.requested.iter().map(|requested| {  
                            let req_1 = requested.username.clone();
                            let req_2 = requested.username.clone();
                            html! {
                                <div class="card" style="width: 18rem;">
                                    <img class="card-img-top" src={requested.profile_pic_url.clone()} alt="Card image cap" style="height: 18rem;"/>
                                    <div class="card-body">
                                    <h5 class="card-title">{&requested.username}</h5>
                                    <p class="card-text">{&requested.description.clone()}</p>
                                    <div class="btn-group btn-group-lg" role="group" aria-label="Basic example">
                                        <button onclick={self.link.callback(move |_| MemePokeState::PokePage(PokeState::Action(SocialAction::AcceptRequested, req_1.clone())))} type="button" class="btn btn-success">{"accept"}</button>
                                        <button onclick={self.link.callback(move |_| MemePokeState::PokePage(PokeState::Action(SocialAction::RejectRequested, req_2.clone())))} type="button" class="btn btn-danger">{"ignore"}</button>
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
                        social.friends.iter().map(|friend| {
                            let fr_str = friend.username.clone();
                            html! {
                                <div class="card" style="width: 18rem;">
                                    <img class="card-img-top" src={friend.profile_pic_url.clone()} alt="Card image cap" style="height: 18rem;"/>
                                    <div class="card-body">
                                        <h5 class="card-title">{&friend.username}</h5>
                                        <p class="card-text">{&friend.description.clone()}</p>
                                        <div class="btn-group btn-group-lg" role="group" aria-label="Basic example">
                                            <a type="button" class="btn btn-primary">{"Chat"}</a>
                                            <button onclick={self.link.callback(move |_| MemePokeState::PokePage(PokeState::Action(SocialAction::RemoveFriend, fr_str.clone())))} type="button" class="btn btn-secondary">{"Remove"}</button>
                                        </div>
                                    </div>
                                </div>
                            }}
                        ).collect::<Html>()
                    }
                </div>
            </> 
        }
    }

    fn view_accountpage(&self) -> Html {
        let descr = self.user.description.clone();
        html! {
            <div class = "d-flex justify-content-center">
                <div class="card mh-50 w-25">
                    <h1 class="h-5">{&self.user.username}</h1>
                    <img src={self.user.profile_pic_url.clone()} class="img-fluid" alt="Responsive image"/>
                    <div class="form-group">
                        <label for="exampleFormControlTextarea1">{"Description:"}</label>
                        <textarea ref=self.descr_box.clone() class="form-control rounded-0" id="exampleFormControlTextarea1" rows="2">{&descr}</textarea>
                        <button onclick={self.link.callback(|_| MemePokeState::AccountPage(AccountState::UpdateDescr))}class="btn btn-primary btn-lg btn-block">{"Update"}</button>
                    </div>
                </div>
            </div>
        }
    }

    fn view_memepage(&self, meme : &Meme) -> Html {
        let meme1 = meme.clone();
        let meme2 = meme.clone();
        let meme3 = meme.clone();
        html! {
            <div class = "d-flex justify-content-center">
                <div class="card  mw-50 mh-50">
                    <img class="card-img-top rounded" src={meme.image_url.clone()} alt="Card image cap"/>
                    <div class="d-flex justify-content-center">
                        <h1>{"Yo is this meme cheese or what?"}</h1>
                    </div>
                    <div class="d-flex justify-content-center">
                        <div class="btn-group btn-group-lg" role="group" aria-label="Basic example">
                            <button onclick={self.link.callback(move |_| MemePokeState::MemePage(MemeState::React(Reaction::Like, meme1.clone())))} type="button" class="btn btn-success">{"Fab"}</button>
                            <button onclick={self.link.callback(move|_| MemePokeState::MemePage(MemeState::React(Reaction::Neutral, meme2.clone())))} type="button" class="btn btn-secondary">{"Meh"}</button>
                            <button onclick={self.link.callback(move |_| MemePokeState::MemePage(MemeState::React(Reaction::Dislike, meme3.clone())))} type="button" class="btn btn-danger">{"Nah"}</button>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
