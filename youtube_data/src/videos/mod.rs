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

pub struct VideoList<'a> {
    service: &'a VideosService,
}

impl<'a> VideoList<'a> {
    pub fn request() {}
}

pub enum VideoListPart {
    Id,
    Snippet,
}

impl std::fmt::Display for VideoListPart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            VideoListPart::Id => "id",
            VideoListPart::Snippet => "snippet",
        }
        .to_string();
        write!(f, "{}", str)
    }
}

#[cfg(test)]
mod tests {}
