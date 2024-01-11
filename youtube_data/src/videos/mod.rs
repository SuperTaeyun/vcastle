pub mod model;

use crate::handler::YouTube;

#[derive(Clone)]
pub struct VideosService {
    youtube: Box<YouTube>,
}

impl VideosService {
    pub fn new(youtube: Box<YouTube>) -> Self {
        Self { youtube }
    }

    pub fn list(&self) -> VideoList {
        todo!()
    }
}

pub struct VideoList {}

impl VideoList {
    pub fn request() {}
}

#[cfg(test)]
mod tests {}
