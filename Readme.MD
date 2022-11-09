## TO RUN
- Run command `rustup target add wasm32-unknown-unknown` to compile target to wasm
- Install these tools:-
    -trunk: `cargo install trunk`
    -bindgen: `cargo install wasm-bindgen-cli`
    -cargo-make: `cargo install cargo-make`
    -node: https://nodejs.org/en/
- Run the server by command: `cargo make serve`


## TO ADD DEPENDENCIES
- Developer can install cargo-edit by running `cargo install cargo-edit`
- Dependencies can now be installed via `cargo add <crate name>`

## OPTIONAL
- Install NodeJs and run `npm install` to install node packages