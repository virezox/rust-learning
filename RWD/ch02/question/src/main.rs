use std::{
    io::{Error, ErrorKind},
    str::FromStr,
};

#[derive(Debug)]
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

#[derive(Debug)]
struct QuestionId(String);

impl Question {
    fn new(id: QuestionId, title: String, content: String, tags: Option<Vec<String>>) -> Self {
        Question {
            id,
            title,
            content,
            tags,
        }
    }
}

impl FromStr for QuestionId {
    type Err = std::io::Error;

    fn from_str(id: &str) -> Result<Self, Self::Err> {
        match id.is_empty() {
            false => Ok(QuestionId(id.to_string())),
            true => Err(Error::new(ErrorKind::InvalidInput, "No id provided")),
        }
    }
}

fn main() {
    let question = Question::new(
        QuestionId::from_str("1").expect("No id provided"),
        String::from("First Question"),
        String::from("Content of question"),
        Some(vec![String::from("foo")]),
    );
    println!("{:?}", question);
}
