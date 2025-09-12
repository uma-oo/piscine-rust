pub mod err;
use err::{ParseErr, ReadErr};
use std::error::Error;
use std::fs;
use serde::Deserialize;
#[derive(Debug,Eq, PartialEq, Deserialize)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub level: u32,
}

#[derive(Debug,Eq, PartialEq, Deserialize)]
pub struct TodoList {
    pub title: String,
    pub tasks: Vec<Task>,
}

impl TodoList {
    pub fn get_todo(path: &str) -> Result<TodoList, Box<dyn Error>> {
        // hna anreadiw l file
        let contents = match fs::read_to_string(path) {
            Ok(value) => value,
            Err(err) => {
                return Err(Box::new(ReadErr { child_err: Box::new(err) }));
            }
        };
        let parsed = match json::parse(&contents) {
            Ok(data) => data,
            Err(err) => {
                return Err(Box::new(ParseErr::Malformed(Box::new(err))));
            }
        };

        if parsed["tasks"].is_empty() {
            return Err(Box::new(ParseErr::Empty));
        }

        let todo_list: TodoList =serde_json::from_str(&parsed.clone().to_string())?;
        

        Ok(todo_list)
    }
}
