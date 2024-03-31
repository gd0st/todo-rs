use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum State {
    Complete,
    Started,
    Stopped,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TodoList {
    name: String,
    todos: HashMap<usize, Todo>,
}

impl TodoList {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            todos: HashMap::new(),
        }
    }

    pub fn add(&mut self, todo: Todo) {
        let key: usize = self.todos.values().len() + 1;

        self.todos.insert(key, todo.id(key));
    }

    pub fn get(&self, id: usize) -> Option<&Todo> {
        self.todos.get(&id)
    }

    pub fn complete(&mut self, id: usize) {
        if let Some(todo) = &mut self.todos.get(&id) {
            todo.complete();
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Todo> {
        self.todos.values()
    }
}

impl From<&str> for Todo {
    fn from(value: &str) -> Self {
        Todo::new(value)
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Todo {
    id: Option<usize>,
    subject: String,
    status: State,
}

impl Todo {
    pub fn new(subject: &str) -> Self {
        Self {
            id: None,
            subject: subject.to_string(),
            status: State::Stopped,
        }
    }

    pub fn id(self, id: usize) -> Self {
        Self {
            id: Some(id),
            subject: self.subject,
            status: self.status,
        }
    }

    pub fn subject(&self) -> String {
        self.subject.to_string()
    }

    pub fn complete(self) -> Self {
        Self {
            subject: self.subject,
            status: State::Complete,
        }
    }

    pub fn start(self) -> Self {
        Self {
            subject: self.subject,
            status: State::Started,
        }
    }

    pub fn pause(self) -> Self {
        Self::new(&self.subject)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_todo_add_to_list_and_find() {
        let mut todo_list = TodoList::new("My first list");
        let todo = Todo::from("Dishes");
        todo_list.add(todo);
    }
    #[test]
    fn find_todo() {
        let mut todo_list = TodoList::new("My first list");
        let todo = Todo::from("Dishes");
        todo_list.add(todo);
        println!("{:?}", todo_list);
        if let Some(found_todo) = todo_list.get("Dishes") {
            assert_eq!(found_todo.subject(), "Dishes".to_string())
        } else {
            assert!(false)
        }
    }

    #[test]
    fn make_new_todo_start_and_complete() {
        let todo = Todo::new("dishes").start().pause().complete();

        assert_eq!(todo.subject, "dishes".to_string())
    }
}
