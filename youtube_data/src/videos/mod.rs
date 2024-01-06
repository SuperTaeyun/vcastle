use reqwest::Client;

pub mod model;

pub struct VideoApi {
    api_key: String,
    client: Client,
}

impl VideoApi {
    pub fn new(api_key: String, client: Client) -> Self {
        Self { api_key, client }
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
