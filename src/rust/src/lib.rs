pub mod read;

#[derive(serde::Serialize)]
struct Json<T: serde::Serialize>(T);
