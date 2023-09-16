# Connector
Connector is a small mod that is suppose to fetch server IPs from a Server so that you can change automatically the server you connect to via the Main Menus Spotlight

## How to use
Connector is a mod that needs to be customised for every community, you need to follow the following steps

## Required Tools
- [hemtt](https://github.com/BrettMayson/HEMTT)
- [rust](https://www.rust-lang.org)
- [cargo-make](https://docs.rs/cargo-make/latest/cli)
- [NodeJs](https://nodejs.org/en) (when using a vServer or similar)

## Steps
- Host the Backend on some type of Service like Heroku or Render or vServer (anything that supports Nodejs with Express)
- Copy EXAMPLE.env to .env and fill in the required information or
    Set SECRET_KEY, SERVER_DATA and Port PORT the Enviroment Variabes on that system
- Add the SECRET_KEY in the file src/lib.rs to the same
- Add Server URL (with port if needed) in the file src/lib.rs to the same
- Build the x64 Extension via `cargo make release`
- Build the Mod via `hemtt release`
- If you use multible servers you need to set the Index of the server to the extension in the config

## Special Thanks
- Brett for his awesome Tools with HEMTT and arma-rs
- Bohemia Interactive for Arma 3
