use serde::Deserialize;

#[derive(Deserialize)]
struct Info {}

pub async fn post() -> String {
    format!("Welcome from things!")
}
