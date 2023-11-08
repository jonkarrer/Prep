use crate::domain::model::UserRoles;

pub struct UserId(pub String);

pub struct User {
    pub email: String,
    pub profile_pic_url: Option<String>,
    pub role: UserRoles,
}
