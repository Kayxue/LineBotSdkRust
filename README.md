# LINE Messaging API SDK for Rust

## Introduction

This is a library originating from [nanato12/line-bot-sdk-rust](https://github.com/nanato12/line-bot-sdk-rust), but with more complete api, newer versions of dependencies and more additional features.

The LINE Messaging API SDK for Rust makes it easy to develop bots using LINE Messaging API, and you can create a sample bot within minutes.

## Documentation

See the official API documentation for more information.

- English: <https://developers.line.biz/en/docs/messaging-api/overview/>
- Japanese: <https://developers.line.biz/ja/docs/messaging-api/overview/>

## Requirements

This library requires stable/beta Rust.

## Installation

### Cargo

You can install with command below

```sh
cargo add bot_sdk_line
```

Or add this line to `Cargo.toml`
```toml
bot_sdk_line = "<version>"
```

### Git

Add content below to your `Cargo.toml`

```toml
[dependencies.bot_sdk_line]
git = "https://github.com/Kayxue/LineBotSdkRust.git"
branch = "master"
```

## Web framework support

Extract `x-line-signature` from the request header.

### Use `rocket` framework

```toml
[dependencies.bot_sdk_line]
...
features = ["rocket_support"]
```

```rust
use bot_sdk_line::support::rocket::Signature;
use rocket::{http::Status, post};

#[post("/callback", data = "<body>")]
async fn world(signature: Signature, body: String) -> (Status, &'static str) {
    ...
}
```

### Use `actix_web` framework

```toml
[dependencies.bot_sdk_line]
...
features = ["actix_support"]
```

```rust
use actix_web::{post, web, Error, HttpResponse};
use bot_sdk_line::support::actix::Signature;

#[post("/callback")]
async fn callback(signature: Signature, bytes: web::Bytes) -> Result<HttpResponse, Error> {
    ...
}
```

### Use `axum` framework (Additional Support)

```toml
[dependencies.bot_sdk_line]
...
features = ["axum_support"]
```

```rust
use actix_web::{post, web, Error, HttpResponse};
use bot_sdk_line::support::actix::Signature;

#[post("/callback")]
async fn callback(signature: Signature, bytes: web::Bytes) -> Result<HttpResponse, Error> {
    ...
}
```

### Use `ntex` framework (Additional Support)

```toml
[dependencies.bot_sdk_line]
...
features = ["ntex_support"]
```

```rust
use ntex::{
    util::Bytes,
    web::{Responder, WebResponseError, post},
};
use bot_sdk_line::support::ntex::Signature;

#[post("/callback")]
async fn callback(
    signature: Signature,
    bytes: Bytes,
) -> Result<impl Responder, impl WebResponseError> {
    ...
}
```

### Use `xitca-web` framework (Additional Support)

```toml
[dependencies.bot_sdk_line]
...
features = ["xitca_support"]
```

```rust
use xitca_web::{bytes::Bytes, codegen::route, error::Error};
use bot_sdk_line::support::xitca::Signature;

#[route("/callback",method = post)]
async fn callback(signature: Signature<'_>, bytes: Bytes) -> Result<&'static str, Error> {
    ...
}
```

## Configuration

```rust
use bot_sdk_line::client::LINE;
use std::env;

fn main() {
    let access_token: &str =
        &env::var("LINE_CHANNEL_ACCESS_TOKEN").expect("Failed to get LINE_CHANNEL_ACCESS_TOKEN");

    let line = LINE::new(access_token.to_string());
}
```

## How to use

The LINE Messaging API uses the JSON data format.

For example, parse body (`&str`) into Result<CallbackRequest, serde_json::Error>.

```rust
let request: Result<CallbackRequest, serde_json::Error> = serde_json::from_str(body);
```

Or directly parse body from `bytes` body.

```rust
let request: Result<CallbackRequest, serde_json::Error> = serde_json::from_slice(&bytes);
```

Then:

```rust
match request {
    Err(err) => {
        // error handling
    },
    Ok(req) => {
        for e in req.events {
            // Processing for various events
        }
    }
}
```

## Bot Examples

* [LineBotSdkExample](https://github.com/Kayxue/LineBotSdkExample): Some examples of using this Rust library.
* [GitHubPushWebhookLineBotRust](https://github.com/Kayxue/GitHubPushWebhookLineBotRust): Line bot for sending GitHub push webhook to Line, but in Rust

## Contributing
Contributions are welcome! And please follow [Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct).
## License

```plain
Copyright 2025 KayXue

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

---

This project contains code under the Apache License 2.0:
lib/src/client/mod.rs
lib/src/parser/mod.rs
lib/src/parser/signature.rs
lib/src/support/actix.rs
lib/src/support/mod.rs
lib/src/support/rocket.rs

The following license applies to these files:

Apache License 2.0

Copyright (c) 2023 nanato12

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
```
