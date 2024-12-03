pub mod benchmark;
pub mod data_processing;
pub mod text_processing;

pub use benchmark::check_memory;
pub use data_processing::{decode_base_64, decode_bytes, encode_base_64};
pub use text_processing::generate_markdown;
