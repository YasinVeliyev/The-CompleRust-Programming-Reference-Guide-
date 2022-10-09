extern crate todolist_parser;
use todolist_parser::TodoList;

fn main() {
    let todos = TodoList::get_todos("./examples/basics.rs");
    match todos {
        Ok(list) => println!("{:?}", list),
        Err(e) => {
            println!("{}", e.to_string());
            println!("{:?}", e);
        }
    }
}
