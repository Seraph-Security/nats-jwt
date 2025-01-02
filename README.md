# NATS JWT

<!-- cargo-sync-readme start -->

Generate JWTs signed using NKEYs for use with [NATS](https://nats.io)

**NOTE** - This is still a work in progress and will be published to crates.io once it's ready.

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