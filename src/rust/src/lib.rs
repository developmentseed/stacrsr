use extendr_api::deserializer::from_robj;
use extendr_api::prelude::*;
use extendr_api::serializer::to_robj;
use extendr_api::Error;
use serde;
use stac::{Format, Value};
use tokio;

#[derive(serde::Serialize)]
struct Json<T: serde::Serialize>(T);

#[extendr]
pub fn read(href: String, format: Option<String>, options: List) -> Result<Robj> {
    let format = format
        .and_then(|f| f.parse::<Format>().ok())
        .or_else(|| Format::infer_from_href(&href))
        .unwrap_or_default();
    let options = options
        .into_hashmap()
        .iter()
        .map(|(key, value)| (key.to_string(), from_robj::<String>(&value).unwrap()))
        .collect::<Vec<_>>();
    // Initialize async runtime
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|e| Error::Other(e.to_string()))?;

    runtime.block_on(async {
        format
            .get_opts::<Value, _, _, _>(href, options)
            .await
            .map_err(|e| Error::Other(e.to_string()))
            .and_then(|value| to_robj(&Json(value)))
    })
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod stacrsr;
    fn read;
}
