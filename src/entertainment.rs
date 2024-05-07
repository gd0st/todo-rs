use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Book {
    title: String,
    author: String,
    pages: usize,
    current_page: usize,
}

impl Book {
    pub fn new(title: &str, author: &str, pages: usize) -> Result<Self, String> {
        if pages > 0 {
            Ok(Book {
                title: title.to_string(),
                author: author.to_string(),
                pages,
                current_page: 0,
            })
        } else {
            Err(
                "A blank page is no empty space. It is brimming with potential... - AA Patawarn"
                    .to_string(),
            )
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn author(&self) -> &str {
        &self.author
    }

    pub fn page_count(&self) -> usize {
        self.pages
    }

    pub fn current_page(&self) -> usize {
        self.current_page
    }

    pub fn set_current_page(&mut self, pages: usize) -> Result<(), String> {
        if pages > self.pages {
            return Err("Pages entered are over max page count.".to_string());
        }
        self.current_page = pages;
        Ok(())
    }
}

// TODO Make part of TaskList through interface etc.
struct Movie {
    title: String,
    release_year: usize,
    genre: String,
}
