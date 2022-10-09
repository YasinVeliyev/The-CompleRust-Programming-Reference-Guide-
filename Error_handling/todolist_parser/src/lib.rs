use std::{error::Error, fs::read_to_string, path::Path};

mod error;
use error::*;

#[derive(Debug)]
pub struct TodoList {
    tasks: Vec<String>,
}

impl TodoList {
    pub fn get_todos<P>(path: P) -> Result<TodoList, Box<dyn Error>>
    where
        P: AsRef<Path>,
    {
        let read_todos = read_todos(path)?;
        let parsed_todos = parse_todos(&read_todos)?;
        Ok(parsed_todos)
    }
}

pub fn parse_todos(todo_str: &str) -> Result<TodoList, Box<dyn Error>> {
    let mut tasks = vec![];
    for i in todo_str.lines() {
        tasks.push(i.to_owned());
    }
    if tasks.is_empty() {
        Err(ParseErr::Empty.into())
    } else {
        Ok(TodoList { tasks })
    }
}

pub fn read_todos<P>(path: P) -> Result<String, Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let raw_todos = read_to_string(path).map_err(|err| ReadErr {
        child_err: Box::new(err),
    })?;
    Ok(raw_todos)
}
