use yew::Properties;
use serde::Deserialize;

#[derive(Properties, Clone, PartialEq, Default, Deserialize)]
pub struct User {
    pub username: String,
    pub description: String,
    pub profile_pic_url: String,
    pub auth_token: String
}