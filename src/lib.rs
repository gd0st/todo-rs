use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub mod entertainment;
use entertainment::Book;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum State {
    Complete,
    Started,
    Stopped,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TaskList<T> {
    name: String,
    tasks: HashMap<usize, T>,
}

impl<T> TaskList<T> {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            tasks: HashMap::new(),
        }
    }

    pub fn name(&self) -> String {
        self.name.to_string()
    }

    pub fn add(&mut self, task: T) {
        let key: usize = self.tasks.values().len() + 1;
        self.tasks.insert(key, task);
    }

    pub fn get(&self, id: usize) -> Option<&T> {
        self.tasks.get(&id)
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.tasks.values()
    }
}

impl TaskList<Book> {
    pub fn search_titles(&self, title: &str) -> Vec<&Book> {
        self.tasks
            .values()
            .filter(move |book| book.title().starts_with(title))
            .collect()
    }

    pub fn search_author(&self, author: &str) -> Vec<&Book> {
        self.tasks
            .values()
            .filter(move |book| book.author().starts_with(author))
            .collect()
    }
}

impl<T> TaskList<T>
where
    T: Task,
{
    pub fn complete(&mut self, id: usize) {
        if let Some(task) = self.tasks.remove(&id) {
            self.add(task.complete())
        }
    }
}

impl From<&str> for Todo {
    fn from(value: &str) -> Self {
        Todo::new(value)
    }
}

impl Task for Book {
    fn name(&self) -> String {
        self.title().to_string()
    }

    fn progress(&self) -> State {
        if self.current_page() == self.page_count() {
            return State::Complete;
        }

        if self.current_page() > 0 {
            return State::Started;
        }

        State::Stopped
    }

    fn complete(mut self) -> Self {
        self.set_current_page(self.page_count()).unwrap();
        self
    }

    fn start(mut self) -> Self {
        match self.current_page() {
            0 => {
                self.set_current_page(1).unwrap();
                self
            }
            _ => self,
        }
    }
}

pub trait Task {
    fn name(&self) -> String;
    fn progress(&self) -> State;
    fn complete(self) -> Self;
    fn start(self) -> Self;
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
            id: self.id,
            subject: self.subject,
            status: State::Complete,
        }
    }

    pub fn start(self) -> Self {
        Self {
            id: self.id,
            subject: self.subject,
            status: State::Started,
        }
    }

    pub fn pause(self) -> Self {
        Self::new(&self.subject)
    }
}

impl Task for Todo {
    fn complete(mut self) -> Self {
        self.status = State::Complete;
        self
    }

    fn name(&self) -> String {
        self.subject.to_string()
    }

    fn progress(&self) -> State {
        self.status.clone()
    }

    fn start(mut self) -> Self {
        self.status = State::Started;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_todo_add_to_list_and_find() {
        let mut todo_list = TaskList::new("My first list");
        let todo = Todo::from("Dishes");
        todo_list.add(todo);
    }
    #[test]
    fn find_todo() {
        let mut todo_list = TaskList::new("My first list");
        let todo = Todo::from("Dishes");
        todo_list.add(todo);
        println!("{:?}", todo_list);
        if let Some(found_todo) = todo_list.get(0) {
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

    #[test]
    fn test_book() {
        let mut book = Book::new("The Godfather", "Mario Puzo", 448).unwrap();
        book = book.start();
        assert_eq!(book.current_page(), 1);

        book.set_current_page(50).unwrap();
        book = book.complete();

        assert_eq!(book.progress(), State::Complete);
    }
}
