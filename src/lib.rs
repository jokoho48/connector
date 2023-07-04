use arma_rs::{arma, Extension};
use reqwest::blocking::Client;

#[arma]
fn init() -> Extension {
    Extension::build()
        .command("resolve", resolve)
        .finish()
}

pub fn resolve() -> String {
    let client = Client::new();
    let response = client.get("REPLACE_ME_WITH_URL")
    .header("secret_key", "REPLACE_ME_WITH_SECRET_KEY").send();
    match response {
        Ok(response) => {
            match response.text() {
                Ok(text) => {
                    text
                },
                Err(_) => {
                    "Error".to_string()
                }
            }
        },
        Err(_) => {
            "Error".to_string()
        }
    }
}