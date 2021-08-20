use yew::Properties;

#[derive(Properties, Clone, PartialEq, Default)]
pub struct User {
    pub username: String,
    pub id: String,
    pub description: Option<String>,
    pub profile_pic_url: String,
    pub auth_token: String
}