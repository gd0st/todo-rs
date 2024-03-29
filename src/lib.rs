use serde::{Deserialize, Serialize};

#[derive(Debug)]
enum State {
    Complete,
    Started,
    Stopped,
}

struct TodoList {
    name: String,
    todos: Vec<Todo>,
}

impl TodoList {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            todos: vec![],
        }
    }

    fn new_todo(&mut self, subject: &str) {
        self.todos.push(Todo::new(subject))
    }

    fn get(&self, subject: &str) -> Option<&Todo> {
        self.todos.iter().find(|todo| todo.subject == subject)
    }
}

struct Todo {
    subject: String,
    status: State,
}

impl Todo {
    pub fn new(subject: &str) -> Self {
        Self {
            subject: subject.to_string(),
            status: State::Stopped,
        }
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
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_todo_add_to_list_and_find() {
        let mut todo_list = TodoList::new("My first list");
        todo_list.new_todo("dishes");

        if let Some(todo) = todo_list.get("dishes") {
            assert_eq!(todo.subject, "dishes".to_string())
        }
    }

    #[test]
    fn make_new_todo_start_and_complete() {
        let todo = Todo::new("dishes").start().pause().complete();

        assert_eq!(todo.subject, "dishes".to_string())
    }

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
