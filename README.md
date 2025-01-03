# NATS IO JWT

[![License MIT](https://img.shields.io/badge/License-MIT-blue.svg)](http://opensource.org/licenses/MIT)
[![mirror](https://img.shields.io/badge/mirror-github-blue)](https://github.com/nats-io/jwt)
[![crates.io](https://img.shields.io/crates/v/nats-io-jwt.svg)](https://crates.io/crates/nats-io-jwt)
[![docs.rs](https://docs.rs/nats-io-jwt/badge.svg)](https://docs.rs/nats-io-jwt)

This crate is based off of a JSON schema that was initially generated from
a JSON schema that was generated from v2.7.3 of the golang nats-io jwt library
at [nats-io/jwt](https://github.com/nats-io/jwt). At the point when this
crate was built, this was the most up-to-date supported library in use for
nats.io. The idea was to generate a language agnostic representation of the JWT
API for nats.io and then generate Rust code from this schema using `typify`.
Finally a thin wrapper found in `src/lib.rs` was built to provide an interface
to the generated code.

**NOTE** - This is still a work in progress while at the 0.0.x version

<!-- cargo-sync-readme start -->

Generate JWTs signed using NKEYs for use with [NATS](https://nats.io)

Supports generating JWTs for Account, User and Activation claims.

## Example

```rust
use nats_jwt::{KeyPair, Token, Account, User, Permission, SigningKeys};

// You would probably load the operator's seed via a config and use KeyPair::from_seed
let operator_signing_key = KeyPair::new_operator();

let account_keypair = KeyPair::new_account();
let account_signing_key = KeyPair::new_account();
let account: Account = Account::builder()
    .signing_keys(SigningKeys::from(&account_signing_key))
    .try_into()
    .expect("Account to be valid");
let account_token = Token::new(account_keypair.public_key())
    .name("My Account")
    .claims(account)
    .sign(&operator_signing_key);
println!("account_token: {}", account_token);

let user_keypair = KeyPair::new_user();
let user: User = User::builder()
   .pub_(Permission::from("service.hello.world"))
   .sub(Permission::from("_INBOX."))
   .subs(10)
   .payload(1024 * 1024) // 1MiB
   .bearer_token(true)
   .try_into()
   .expect("Account to be valid");
let user_token = Token::new(user_keypair.public_key())
    .name("My User")
    .claims(user)
    .sign(&account_signing_key);
println!("user_token: {}", user_token);
```

## License

Some parts taken from `https://github.com/AircastDev/nats-jwt`

Licensed under

- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)


<!-- cargo-sync-readme end -->
