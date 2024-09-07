use domain::models::claans::Claan;
use domain::models::records::Record;
use domain::models::seasons::Season;
use domain::models::tasks::Task;
use domain::models::users::User;
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
