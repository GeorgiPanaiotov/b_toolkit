pub mod benchmark;
pub mod processing;

pub use benchmark::check_memory;
pub use processing::data_processing::{decode_base_64, decode_bytes, encode_base_64};
pub use processing::text_processing::generate_markdown;
