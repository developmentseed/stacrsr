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
    format: Option<String>,
    //options: Option<Vec<(String, String)>>,
) -> Result<Robj> {
    let format = format
        .and_then(|f| f.parse::<Format>().ok())
        .or_else(|| Format::infer_from_href(&href))
        .unwrap_or_default();
    let options = options.unwrap_or_default();
    tokio::future_into_py(py, async move {
        let value = format
            .get_opts::<Value, _, _, _>(href, options)
            .await
            .map_err(Error::from)?;
        Ok(Json(value))
    })

}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod stacrsr;
    fn hello_world;
    fn read;
}
