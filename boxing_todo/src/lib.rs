mod err;
pub use err::{ParseErr, ReadErr};
use std::error::Error;
use std::fs;
#[derive(Eq, PartialEq, Debug)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub level: u32,
}

#[derive(Eq, PartialEq, Debug)]
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

        let mut tasks: Vec<Task> = Vec::new();

        for i in 0..parsed["tasks"].len() {
            tasks.push(Task {
                id: parsed["tasks"][i]["id"].to_string().parse().unwrap(),
                description: parsed["tasks"][i]["description"].to_string(),
                level: parsed["tasks"][i]["level"].to_string().parse().unwrap(),
            });
        }

        Ok(Self {
            title: parsed["title"].to_string(),
            tasks: tasks,
        })
    }
}
