mod direction;
mod vec_helpers;

pub use direction::*;
pub use vec_helpers::*;

pub fn is_local_build() -> bool {
    std::env::var("LOCAL_BUILD") == Ok("2".to_string())
}
