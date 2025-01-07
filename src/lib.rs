//! Generate JWTs signed using NKEYs for use with [NATS](https://nats.io)
//!
//! **NOTE** - This is still a work in progress and will be published to crates.io once it's ready.
//!
//! Supports generating JWTs for Account, User and Activation claims.
//!
//! ## Example
//!
//! ```
//! use nats_io_jwt::{KeyPair, Token, Account, User, Permission, SigningKeys};
//!
//! // You would probably load the operator's seed via a config and use KeyPair::from_seed
//! let operator_signing_key = KeyPair::new_operator();
//!
//! let account_keypair = KeyPair::new_account();
//! let account_signing_key = KeyPair::new_account();
//! let account: Account = Account::builder()
//!     .signing_keys(SigningKeys::from(&account_signing_key))
//!     .try_into()
//!     .expect("Account to be valid");
//! let account_token = Token::new(account_keypair.public_key())
//!     .name("My Account")
//!     .claims(account)
//!     .sign(&operator_signing_key);
//! println!("account_token: {}", account_token);
//!
//! let user_keypair = KeyPair::new_user();
//! let user: User = User::builder()
//!    .pub_(Permission::from("service.hello.world"))
//!    .sub(Permission::from("_INBOX."))
//!    .subs(10)
//!    .payload(1024 * 1024) // 1MiB
//!    .bearer_token(true)
//!    .try_into()
//!    .expect("Account to be valid");
//! let user_token = Token::new(user_keypair.public_key())
//!     .name("My User")
//!     .claims(user)
//!     .sign(&account_signing_key);
//! println!("user_token: {}", user_token);
//! ```
//!
//! ## License
//!
//! Some parts taken from `https://github.com/AircastDev/nats-jwt`
//!
//! Licensed under
//!
//! - MIT license
//!   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)
//!
mod nats_jwt_schema;

use data_encoding::{BASE32HEX_NOPAD, BASE64URL_NOPAD};
use std::time::SystemTime;

use sha2::{Digest, Sha256};

// flatten the module into this one
pub use nats_jwt_schema::*;

/// Re-export some `KeyPair` things from the nkeys crate.
pub use nkeys::{decode_seed, from_public_key, KeyPair, KeyPairType};

const JWT_HEADER: &str = r#"{"typ":"JWT","alg":"ed25519-nkey"}"#;

impl From<String> for SigningKeys {
    fn from(signing_public_key: String) -> Self {
        vec![SigningKeysItem::PublicKey(signing_public_key)].into()
    }
}

impl From<&str> for SigningKeys {
    fn from(signing_public_key: &str) -> Self {
        vec![SigningKeysItem::PublicKey(signing_public_key.into())].into()
    }
}

impl From<&KeyPair> for SigningKeys {
    fn from(signing_key: &KeyPair) -> Self {
        vec![SigningKeysItem::PublicKey(signing_key.public_key())].into()
    }
}

impl From<Vec<String>> for Permission {
    fn from(allow: Vec<String>) -> Self {
        Permission::try_from(Permission::builder().allow(Some(allow.into()))).unwrap()
    }
}

impl From<Vec<&str>> for Permission {
    fn from(allow: Vec<&str>) -> Self {
        let allow_vec: Vec<String> = allow.iter().map(|s| s.to_string()).collect();
        Permission::try_from(Permission::builder().allow(Some(allow_vec.into()))).unwrap()
    }
}

impl From<String> for Permission {
    fn from(allow: String) -> Self {
        Permission::try_from(Permission::builder().allow(Some(vec![allow].into()))).unwrap()
    }
}

impl From<&str> for Permission {
    fn from(allow: &str) -> Self {
        let allow_vec: Vec<String> = vec![allow.to_string()];
        Permission::try_from(Permission::builder().allow(Some(allow_vec.into()))).unwrap()
    }
}

/// JWT token builder.
#[derive(Debug, Clone)]
pub struct Token {
    subject: String,
    name: Option<String>,
    claims: Option<Claims>,
    expires: Option<i64>,
}

impl Token {
    pub fn new(subject: impl Into<String>) -> Self {
        Self {
            subject: subject.into(),
            name: None,
            claims: None,
            expires: None,
        }
    }

    /// Set the friendly name for the token, can be anything, defaults to the token subject
    #[must_use]
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Set the Claims inside the Token.
    #[must_use]
    pub fn claims(mut self, claims: impl Into<Claims>) -> Self {
        self.claims = Some(claims.into());
        self
    }

    /// Set expiration
    #[must_use]
    pub fn expires(mut self, expires: i64) -> Self {
        self.expires = Some(expires);
        self
    }

    /// Sign the token with the given signing key, returning a JWT string.
    ///
    /// If this is a User token, this should be the Account signing key.
    /// If this is an Account token, this should be the Operator key
    ///
    /// # Panics
    ///
    /// - If system time is before UNIX epoch.
    /// - If the seconds from UNIX epoch cannot be represented in a i64.
    pub fn sign(self, signing_key: &KeyPair) -> String {
        let issued_at: i64 = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("system time is after the unix epoch")
            .as_secs()
            .try_into()
            .expect("seconds from UNIX epoch cannot be represented in a i64");
        let subject = self.subject.clone();
        let mut jwt: Jwt = Jwt::builder()
            .iat(issued_at)
            .iss(signing_key.public_key())
            .jti("")
            .sub(subject)
            .name(self.name)
            .nats(self.claims.expect("Claims to be set"))
            .exp(self.expires)
            .try_into()
            .expect("Jwt should be well formed");

        let claims_str = serde_json::to_string(&jwt).expect("claims serialisation cannot fail");
        let mut hasher = Sha256::new();
        hasher.update(claims_str);
        let claims_hash = hasher.finalize();
        jwt.jti = BASE32HEX_NOPAD.encode(claims_hash.as_slice());

        let claims_str = serde_json::to_string(&jwt).expect("claims serialisation cannot fail");

        let b64_header = BASE64URL_NOPAD.encode(JWT_HEADER.as_bytes());
        let b64_body = BASE64URL_NOPAD.encode(claims_str.as_bytes());
        let jwt_half = format!("{b64_header}.{b64_body}");
        let sig = signing_key.sign(jwt_half.as_bytes()).unwrap();
        let b64_sig = BASE64URL_NOPAD.encode(&sig);

        format!("{jwt_half}.{b64_sig}")
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_new_token() {
        let id_key = nkeys::KeyPair::new_account();
        let operator_key_pair = nkeys::KeyPair::new_operator();

        let limits: OperatorLimits = OperatorLimits::default();

        let account: Account = Account::builder()
            .limits(limits)
            .signing_keys(SigningKeys::from("key"))
            .try_into()
            .expect("Account to be valid");

        let jwt_str = Token::new(id_key.public_key())
            .name("test")
            .claims(account)
            .sign(&operator_key_pair);

        let claims_b64 = jwt_str.split('.').skip(1).next().unwrap().as_bytes();
        let claims_raw = BASE64URL_NOPAD.decode(claims_b64).unwrap();
        let jwt: Jwt = serde_json::from_slice(&claims_raw).unwrap();
        assert_eq!(jwt.iss, operator_key_pair.public_key());
        assert_eq!(jwt.sub, id_key.public_key());
        match &jwt.nats {
            Claims::Account(account) => {
                if let Some(limits) = &account.limits {
                    assert_eq!(limits.conn, -1);
                    assert_eq!(limits.consumer, -1);
                    assert_eq!(limits.data, -1);
                    assert_eq!(limits.disallow_bearer, false);
                    assert_eq!(limits.disk_max_stream_bytes, 0);
                    assert_eq!(limits.disk_storage, -1);
                    assert_eq!(limits.exports, -1);
                    assert_eq!(limits.imports, -1);
                    assert_eq!(limits.leaf, -1);
                    assert_eq!(limits.max_ack_pending, 0);
                    assert_eq!(limits.max_bytes_required, false);
                    assert_eq!(limits.mem_max_stream_bytes, 0);
                    assert_eq!(limits.mem_storage, -1);
                    assert_eq!(limits.payload, -1);
                    assert_eq!(limits.streams, -1);
                    assert_eq!(limits.subs, -1);
                    assert_eq!(limits.wildcards, true);
                } else {
                    panic!("Expected Default Limits");
                };
            }
            Claims::User(_) => panic!("Expected Account, was User"),
            Claims::Activation(_) => panic!("Expected Account, was Activation"),
        }

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
    }
}
