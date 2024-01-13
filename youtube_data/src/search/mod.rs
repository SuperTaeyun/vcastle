pub mod model;

use crate::handler::YouTube;

#[derive(Clone)]
pub struct SearchService {
    youtube: Box<YouTube>,
}

impl SearchService {
    pub fn new(youtube: Box<YouTube>) -> Self {
        Self { youtube }
    }

    pub fn list(&self) -> SearchList {
        todo!()
    }
}

pub struct SearchList<'a> {
    service: &'a SearchService,
}

impl<'a> SearchList<'a> {
    pub fn request() {}
}

pub enum SearchListPart {
    Id,
    Snippet,
}

impl std::fmt::Display for SearchListPart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            SearchListPart::Id => "id",
            SearchListPart::Snippet => "snippet",
        }
        .to_string();
        write!(f, "{}", str)
    }
}

#[cfg(test)]
mod tests {}
