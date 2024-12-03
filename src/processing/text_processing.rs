use comrak::{markdown_to_html, ComrakOptions};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn generate_markdown(markdown_input: &str) -> String {
    let mut options = ComrakOptions::default();
    options.extension.table = true;
    options.extension.tasklist = true;
    options.extension.strikethrough = true;

    let mut html_output = markdown_to_html(markdown_input, &options);

    html_output = html_output
        .replace("[ ]", r#"<input type="checkbox" disabled>"#)
        .replace("[x]", r#"<input type="checkbox" checked disabled>"#);

    html_output
}
