use std::fmt::Formatter;
use std::fmt::Display;
use std::fmt;
pub struct Feedback {
    message: Option<String>,
    date: String,
    user_age: u8,
    user_name: String,
    user_id: String,
    stars: f32,
}

impl Display for Feedback {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "mark: {},
        {}

        date{}, user: {},
        private information {}, {}",
        self.stars, 
        if let Some(message) = &self.message {
            message.clone()
        } else {
            " ".to_string()
        },
        self.date, 
        self.user_name,
        self.user_id,
        self.user_age)
    }
}

pub struct User {
    pub username :String,
    pub age: u8,
    pub user_id: String,
}

impl User {
    pub fn crate_feedback(&self, message: Option<String>, date: String, stars: f32) -> Feedback {
        Feedback {
            message,
            date,
            user_age: self.age,
            user_name: self.username.clone(),
            user_id: self.user_id.clone(),
            stars,
        }
    }
}
