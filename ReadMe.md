# Connector
Connector is a small mod that is suppose to fetch server IPs from a Server so that you can change automatically the server you connect to via the Main Menus Spotlight

## How to use
Connector is a mod that needs to be customised for every community, you need to follow the following steps

## Required Tools
- hemtt (https://github.com/BrettMayson/HEMTT)
- rust (https://www.rust-lang.org/)
- cargo-make (https://docs.rs/cargo-make/latest/cli/)

## Steps
- Host the Backend on some type of Service like Heroku or Render or Server (anything that supports Nodejs with Express)
- Set SECRET_KEY and SERVER_DATA in the Enviroment Variabes on that system
- Add the SECRET_KEY in the file src/lib.rs to the same
- Add Server URL in the file src/lib.rs to the same
- Build the x64 Extension via `cargo make release`
- Build the Mod via `hemtt release`

## Special Thanks
- Brett for his awesome Tools with HEMTT and arma-rs
- Bohemia Interactive for Arma 3