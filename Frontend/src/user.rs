use yew::prelude::*;

pub struct Meme {
    pub id: u64,
    pub post : String,
    pub image_url: String,
    pub original_poster: String
}

pub struct User {
    pub username: String,
    pub description: String,
    pub profile_pic_url: String,
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

impl User {

    // getting user info
    pub fn get_user(code: &str) -> Self {

        /* Get token
         * - post request to url: https://www.reddit.com/api/v1/access_token
         * - post data: grant_type=authorization_code&code=CODE&redirect_uri=URI
         */

        /*
         * #Get user identity
         *  - get request to /api/v1/me to get identity
         *  - use webserver request for User table (using id)
         *  - get fields
         */

        // give up the token (no longer needed)

        Self {
            username: String::from("Bob"),
            description: String::from("Mixologist"),
            profile_pic_url: String::from("https://i.pinimg.com/474x/ba/f8/04/baf8044a230c27941d6fedd5e97ab6a2.jpg")
        }

    }


    // getting social information
    pub fn get_friends(&self) -> Vec<Self> {

        /* Make a request to the webserver to get friends
         * Get the returned friend list
         */

         vec![]
    }

    pub fn get_requests(&self) -> Vec<Self> {

        /* Make a request to the webserver to get friends
         * Get the returned requests list
         */

         unimplemented!();
    }

    pub fn get_requested(&self) -> Vec<Self> {

        /* Make a request to the webserver to get friends
         * Get the returned requested list
         */

         vec![]
    }

    pub fn get_new_match(&self) {
        
        /*
            Send new friend request to server
            - Server takes api request
            - Server creates new friend
            - use self.link.callback(Update) to re-fetch requests, friends etc
         */

    }

    // user requests

    pub fn social(&self, username: &str, action: SocialAction) {
        
    }


    // getting memes
    pub fn get_meme(&self) -> Meme {

        /* 
          Get a new meme from the server
        */

        Meme {
            id: 111,
            post: String::from("https://www.reddit.com/r/Shrekmemes/comments/oxipnn/when_the_villagers_dont_find_onions/"),
            image_url: String::from("https://i.redd.it/jp1bitzz29f71.jpg"),
            original_poster: String::from("Cwf97"),
        }
        
    }

    pub fn react_meme(&self, meme_id: u64, reaction: Reaction) {
        
    }


    // description
    pub fn update_description(&self, description: &str) {

    }
}