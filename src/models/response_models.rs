use crate::models::claans::Claan;
use crate::models::records::Record;
use crate::models::seasons::Season;
use crate::models::tasks::Task;
use crate::models::users::User;
use rocket::serde::Serialize;

#[derive(Serialize)]
pub enum ResponseBody {
    Message(String),
    Claan(Claan),
    Claans(Vec<Claan>),
    Record(Record),
    Records(Vec<Record>),
    Season(Season),
    Seasons(Vec<Season>),
    Task(Task),
    Tasks(Vec<Task>),
    User(User),
    Users(Vec<User>),
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Response {
    pub body: ResponseBody,
}
