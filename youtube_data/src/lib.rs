pub mod channels;
pub mod search;
pub mod videos;

mod handler;
mod model;

pub use handler::YouTube;

#[cfg(test)]
pub(crate) fn get_develop_key() -> String {
    use dotenv::dotenv;

    dotenv().ok();
    std::env::var("TEST_API_KEY").unwrap()
}
