use yew::prelude::*;

pub struct User {
    pub username: String,
    pub description: String,
    pub profile_pic_url: String,
}

impl User {

    pub fn get_user(code: String) -> Self {

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
            profile_pic_url: String::from("")
        }

    }

    fn get_friends(&self) -> Vec<Self> {

        /* Make a request to the webserver to get friends
         * Get the returned friend list
         */

         unimplemented!();
    }

    fn get_requests(&self) -> Vec<Self> {

        /* Make a request to the webserver to get friends
         * Get the returned requests list
         */

         unimplemented!();
    }

    fn get_requested(&self) -> Vec<Self> {

        /* Make a request to the webserver to get friends
         * Get the returned requested list
         */

         unimplemented!();
    }

    fn get_new_match(&self) {
        
        /*
            Send new friend request to server
            - Server takes api request
            - Server creates new friend
            - use self.link.callback(Update) to re-fetch requests, friends etc
         */

         unimplemented!();
    }
}