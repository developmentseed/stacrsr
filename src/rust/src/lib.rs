use extendr_api::prelude::*;
use stac::{Format, Value};

/// Return string `"Hello world!"` to R.
/// @export
#[extendr]
fn hello_world() -> &'static str {
    "Hello world!"
}

#[extendr]
fn read(
    href: String,
    //format: Option<String>,
    //options: Option<Vec<(String, String)>>,
) -> String {
    //let format = format
    //    .and_then(|f| f.parse::<Format>().ok())
    //    .or_else(|| Format::infer_from_href(&href))
    //    .unwrap_or_default();
    //let options = options.unwrap_or_default();
    href.to_string()
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod stacrsr;
    fn hello_world;
    fn read;
}
