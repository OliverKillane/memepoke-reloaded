use yew::{Html, html};


// app constants
pub const client_id : &str = "WI5Q-r9Nu3EjSxIjz5SzEA";
pub const redirect_uri : &str = "http://127.0.0.1:8080/memepoke";
pub const about_uri : &str = "http://127.0.0.1:8080/about";
pub const site_uri : &str = "http://127.0.0.1:8080";

// useful pages
pub fn view_error(message : &str) -> Html {
    html! {
        <div class="bg-image" style="background-image: url('img/shrek.gif');height: 100vh">
            <div class = "d-flex justify-content-center">
                <div class="card mh-50 w-25">
                    <h1 class="h-5">{"Error!"}</h1>
                    <img src="img/memepoke.png" class="img-fluid" alt="Responsive image"/>
                    <p>{message}</p>
                    <a class="btn btn-primary btn-lg btn-block" href=site_uri>{"Back to Login"}</a>
                </div>
            </div>
        </div>
    }
}

pub fn view_loading(message : &str) -> Html {
    html! {
        <div class="bg-image" style="background-image: url('img/shrek.gif');height: 100vh">
            <div class = "d-flex justify-content-center">
                <div class="card mh-50 w-25">
                    <h1 class="h-5">{"Loading"}</h1>
                    <img src="img/memepoke.png" class="img-fluid" alt="Responsive image"/>
                    <div class="spinner-border" style="width: 3rem; height: 3rem;" role="status">
                        <span class="sr-only">{message}</span>
                    </div>
                    <a class="btn btn-primary btn-lg btn-block" href=site_uri>{"Back to Login"}</a>
                </div>
            </div>
        </div>
    }
}



/*
NOTES:

reddit:
appID: WI5Q-r9Nu3EjSxIjz5SzEA
direct to url: https://www.reddit.com/api/v1/authorize?client_id=WI5Q-r9Nu3EjSxIjz5SzEA&response_type=code&state=RANDOMSTRING&redirect_uri=http://127.0.0.1:8080/memepoke&duration=temporary&scope=identity

redirects:
http://127.0.0.1/memepoke?state=RANDOMSTRING&code=rx-5Ps6T60jF1UfbhS2QIsuTYHqXJw#_
http://127.0.0.1/memepoke?state=RANDOMSTRING&error=access_denied#_
*/