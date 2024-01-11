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

pub struct SearchList {}

impl SearchList {
    pub fn request() {}
}

#[cfg(test)]
mod tests {}
