pub mod channels;
mod handler;
mod model;
pub mod search;
pub mod videos;

pub use handler::YouTubeDataApiHandler;
pub use model::ListResponse;

#[cfg(test)]
pub(crate) fn get_develop_key() -> String {
    use dotenv::dotenv;

    dotenv().ok();
    std::env::var("TEST_API_KEY").unwrap()
}
