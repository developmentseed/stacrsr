use extendr_api::prelude::*;
use extendr_api::serializer::to_robj;
use extendr_api::ToVectorValue;
use serde;
use stac::mime::JSON;
use stac::{Error, Format, Value};
use tokio;

/// Return string `"Hello world!"` to R.
/// @export
#[extendr]
fn hello_world() -> &'static str {
    "Hello world!"
}

struct Json<T: serde::Serialize>(T);

#[extendr]
pub fn read(
    href: String,
    format: Option<String>,
    options: Option<Vec<(String, String)>>,
) -> Result<Json<Value>> {
    //Result<Bound<'_, Any>> {
    let format = format
        .and_then(|f| f.parse::<Format>().ok())
        .or_else(|| Format::infer_from_href(&href))
        .unwrap_or_default();
    let options = options.unwrap_or_default();

    // Initialize async runtime
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(async {
        let value = format
            .get_opts::<Value, _, _, _>(href, options)
            .await
            .map_err(Error::from)
            .unwrap();
        Ok(JSON(value))
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
