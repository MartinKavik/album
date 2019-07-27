# album
Photo album manager

## Frameworks
* Seed : https://seed-rs.org/

## Other packages
* wasm-bindgen : Interacting between JS and Rust
* dotenv : To use variables from .env file
* futures : Asynchronous tasks

## Installation

### Install Rustup + Cargo
https://www.rust-lang.org/learn/get-started

### Install wasm-pack
https://rustwasm.github.io/wasm-pack/installer/

### Install Node Dependencies
```Shell
npm i 
```

## Configuration (Dotenv)

Create in root folder the file `.env` containing : 
```
API_URL=http://your_api_url.com
```

## Re-build Rust code
```Shell
wasm-pack build
```

## Run
```Shell
npm start
```
