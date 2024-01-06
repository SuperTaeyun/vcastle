use reqwest::Client;

pub mod model;

pub struct SearchApi {
    api_key: String,
    client: Client,
}

impl SearchApi {
    pub fn new(api_key: String, client: Client) -> Self {
        Self { api_key, client }
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
