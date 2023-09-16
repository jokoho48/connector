use arma_rs::{arma, Extension};
use reqwest::blocking::Client;

#[arma]
fn init() -> Extension {
    Extension::build().command("resolve", resolve).finish()
}
pub fn resolve(idx: i32) -> String {
    let client = Client::new();
    let response = client
        .get("REPLACE_ME_WITH_URL")
        .header("secret_key", "REPLACE_ME_WITH_SECRET_KEY")
        .header("server_index", idx)
        .send();
    response.map_or_else(
        |_| "Error".to_string(),
        |response| {
            response
                .text()
                .map_or_else(|_| "Error".to_string(), |text| text)
        },
    )
}
