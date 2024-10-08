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
        |err| err.to_string(),
        |response| {
            response
                .text()
                .map_or_else(|err| err.to_string(), |text| text)
        },
    )
}

#[test]
fn test_server_0() {
    let extension = init().testing();
    let (result, code) = extension.call("resolve", Some(vec!["0".to_string()]));
    // Assert if result is Empty
    assert_eq!(code, 0);
    print!("{}", result);
    print!("{}", code);
}
