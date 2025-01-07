#![allow(unknown_lints)]
#![allow(clippy::all)]

/// Error types.
pub mod error {
    /// Error from a TryFrom or FromStr implementation.
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
///Represents the core configuration of an account, including imports, exports, limits, signing keys, default permissions, mappings, external auth, and more.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Represents the core configuration of an account, including imports, exports, limits, signing keys, default permissions, mappings, external auth, and more.",
///  "allOf": [
///    {
///      "$ref": "#/$defs/Info"
///    },
///    {
///      "$ref": "#/$defs/GenericFields"
///    },
///    {
///      "type": "object",
///      "properties": {
///        "authorization": {
///          "description": "Configuration for external authorization callouts.",
///          "$ref": "#/$defs/ExternalAuthorization"
///        },
///        "cluster_traffic": {
///          "description": "Indicates how cluster traffic is handled by this account. Can be 'system' or 'owner'.",
///          "$ref": "#/$defs/ClusterTraffic"
///        },
///        "default_permissions": {
///          "description": "Default permissions applied to users of this account if no user-specific permissions are provided.",
///          "$ref": "#/$defs/Permissions"
///        },
///        "exports": {
///          "description": "Subjects exported from this account.",
///          "$ref": "#/$defs/Exports"
///        },
///        "imports": {
///          "description": "Subjects imported from other accounts.",
///          "$ref": "#/$defs/Imports"
///        },
///        "limits": {
///          "description": "Limits that apply to this account, including operator-level, NATS, and JetStream constraints.",
///          "$ref": "#/$defs/OperatorLimits"
///        },
///        "mappings": {
///          "description": "Subject mappings that redirect or distribute messages from one subject to others.",
///          "$ref": "#/$defs/Mapping"
///        },
///        "revocations": {
///          "description": "A list of revoked user or account keys associated with this account.",
///          "$ref": "#/$defs/RevocationList"
///        },
///        "signing_keys": {
///          "description": "Keys authorized to sign user JWTs on behalf of this account, along with their scopes.",
///          "$ref": "#/$defs/SigningKeys"
///        },
///        "trace": {
///          "description": "Distributed message tracing configuration for this account.",
///          "$ref": "#/$defs/MsgTrace"
///        },
///        "type": {
///          "default": "account",
///          "const": "account",
///          "$ref": "#/$defs/ClaimsType"
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Account {
    ///Configuration for external authorization callouts.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub authorization: ::std::option::Option<ExternalAuthorization>,
    ///Indicates how cluster traffic is handled by this account. Can be 'system' or 'owner'.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cluster_traffic: ::std::option::Option<ClusterTraffic>,
    ///Default permissions applied to users of this account if no user-specific permissions are provided.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub default_permissions: ::std::option::Option<Permissions>,
    ///A more detailed description.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    ///Subjects exported from this account.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub exports: ::std::option::Option<Exports>,
    ///Subjects imported from other accounts.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub imports: ::std::option::Option<Imports>,
    ///URL to find extra info.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub info_url: ::std::option::Option<::std::string::String>,
    ///Limits that apply to this account, including operator-level, NATS, and JetStream constraints.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub limits: ::std::option::Option<OperatorLimits>,
    ///Subject mappings that redirect or distribute messages from one subject to others.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub mappings: ::std::option::Option<Mapping>,
    ///A list of revoked user or account keys associated with this account.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub revocations: ::std::option::Option<RevocationList>,
    ///Keys authorized to sign user JWTs on behalf of this account, along with their scopes.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub signing_keys: ::std::option::Option<SigningKeys>,
    ///Tags used to categorize or label this entity.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tags: ::std::option::Option<TagList>,
    ///Distributed message tracing configuration for this account.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub trace: ::std::option::Option<MsgTrace>,
    #[serde(rename = "type", default = "defaults::account_type")]
    pub type_: AccountType,
    ///The version of the claim.
    #[serde(default = "defaults::default_u64::<i64, 2>")]
    pub version: i64,
}
impl ::std::convert::From<&Account> for Account {
    fn from(value: &Account) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for Account {
    fn default() -> Self {
        Self {
            authorization: Default::default(),
            cluster_traffic: Default::default(),
            default_permissions: Default::default(),
            description: Default::default(),
            exports: Default::default(),
            imports: Default::default(),
            info_url: Default::default(),
            limits: Default::default(),
            mappings: Default::default(),
            revocations: Default::default(),
            signing_keys: Default::default(),
            tags: Default::default(),
            trace: Default::default(),
            type_: defaults::account_type(),
            version: defaults::default_u64::<i64, 2>(),
        }
    }
}
impl Account {
    pub fn builder() -> builder::Account {
        Default::default()
    }
}
///Represents all operator-imposed limits on an account, including NATS limits, account limits, and JetStream limits.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Represents all operator-imposed limits on an account, including NATS limits, account limits, and JetStream limits.",
///  "type": "object",
///  "properties": {
///    "conn": {
///      "description": "Max number of active connections",
///      "default": -1,
///      "type": "integer"
///    },
///    "disallow_bearer": {
///      "description": "User JWTs cannot be used as Bearer Tokens.",
///      "default": false,
///      "type": "boolean"
///    },
///    "exports": {
///      "description": "Max number of exports (-1 for no limit).",
///      "default": -1,
///      "type": "integer"
///    },
///    "imports": {
///      "description": "Max number of imports (-1 for no limit).",
///      "default": -1,
///      "type": "integer"
///    },
///    "leaf": {
///      "description": "Max number of leaf node connections (-1 for no limit).",
///      "default": -1,
///      "type": "integer"
///    },
///    "wildcards": {
///      "description": "Whether wildcard exports are allowed.",
///      "default": true,
///      "type": "boolean"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct AccountLimits {
    ///Max number of active connections
    #[serde(default = "defaults::default_i64::<i64, -1>")]
    pub conn: i64,
    ///User JWTs cannot be used as Bearer Tokens.
    #[serde(default)]
    pub disallow_bearer: bool,
    ///Max number of exports (-1 for no limit).
    #[serde(default = "defaults::default_i64::<i64, -1>")]
    pub exports: i64,
    ///Max number of imports (-1 for no limit).
    #[serde(default = "defaults::default_i64::<i64, -1>")]
    pub imports: i64,
    ///Max number of leaf node connections (-1 for no limit).
    #[serde(default = "defaults::default_i64::<i64, -1>")]
    pub leaf: i64,
    ///Whether wildcard exports are allowed.
    #[serde(default = "defaults::default_bool::<true>")]
    pub wildcards: bool,
}
impl ::std::convert::From<&AccountLimits> for AccountLimits {
    fn from(value: &AccountLimits) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for AccountLimits {
    fn default() -> Self {
        Self {
            conn: defaults::default_i64::<i64, -1>(),
            disallow_bearer: Default::default(),
            exports: defaults::default_i64::<i64, -1>(),
            imports: defaults::default_i64::<i64, -1>(),
            leaf: defaults::default_i64::<i64, -1>(),
            wildcards: defaults::default_bool::<true>(),
        }
    }
}
impl AccountLimits {
    pub fn builder() -> builder::AccountLimits {
        Default::default()
    }
}
///AccountType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "enum": [
///    "account"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum AccountType {
    #[serde(rename = "account")]
    Account,
}
impl ::std::convert::From<&Self> for AccountType {
    fn from(value: &AccountType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for AccountType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Account => write!(f, "account"),
        }
    }
}
impl ::std::str::FromStr for AccountType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "account" => Ok(Self::Account),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AccountType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for AccountType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AccountType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for AccountType {
    fn default() -> Self {
        AccountType::Account
    }
}
///Defines the custom parts of an activation claim, including the imported subject and its type (stream/service).
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Defines the custom parts of an activation claim, including the imported subject and its type (stream/service).",
///  "allOf": [
///    {
///      "$ref": "#/$defs/GenericFields"
///    },
///    {
///      "type": "object",
///      "properties": {
///        "issuer_account": {
///          "description": "The account public key that issued this activation (if signing key used).",
///          "type": "string"
///        },
///        "kind": {
///          "description": "The type of import the activation is for, either 'stream' or 'service'.",
///          "$ref": "#/$defs/ExportType"
///        },
///        "subject": {
///          "description": "The subject to which the activation applies.",
///          "$ref": "#/$defs/Subject"
///        },
///        "type": {
///          "default": "activation",
///          "const": "activation",
///          "$ref": "#/$defs/ClaimsType"
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Activation {
    ///The account public key that issued this activation (if signing key used).
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub issuer_account: ::std::option::Option<::std::string::String>,
    ///The type of import the activation is for, either 'stream' or 'service'.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub kind: ::std::option::Option<ExportType>,
    ///The subject to which the activation applies.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subject: ::std::option::Option<Subject>,
    ///Tags used to categorize or label this entity.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tags: ::std::option::Option<TagList>,
    #[serde(rename = "type", default = "defaults::activation_type")]
    pub type_: ActivationType,
    ///The version of the claim.
    #[serde(default = "defaults::default_u64::<i64, 2>")]
    pub version: i64,
}
impl ::std::convert::From<&Activation> for Activation {
    fn from(value: &Activation) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for Activation {
    fn default() -> Self {
        Self {
            issuer_account: Default::default(),
            kind: Default::default(),
            subject: Default::default(),
            tags: Default::default(),
            type_: defaults::activation_type(),
            version: defaults::default_u64::<i64, 2>(),
        }
    }
}
impl Activation {
    pub fn builder() -> builder::Activation {
        Default::default()
    }
}
///ActivationType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "enum": [
///    "activation"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ActivationType {
    #[serde(rename = "activation")]
    Activation,
}
impl ::std::convert::From<&Self> for ActivationType {
    fn from(value: &ActivationType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for ActivationType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Activation => write!(f, "activation"),
        }
    }
}
impl ::std::str::FromStr for ActivationType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "activation" => Ok(Self::Activation),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ActivationType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ActivationType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ActivationType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for ActivationType {
    fn default() -> Self {
        ActivationType::Activation
    }
}
///A list of CIDR-notation addresses for restricting access based on IP ranges.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "A list of CIDR-notation addresses for restricting access based on IP ranges.",
///  "type": "array",
///  "items": {
///    "description": "A CIDR (Classless Inter-Domain Routing) notation string (e.g., '192.168.0.0/16').",
///    "type": "string"
///  },
///  "uniqueItems": true
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct CidrList(pub Vec<::std::string::String>);
impl ::std::ops::Deref for CidrList {
    type Target = Vec<::std::string::String>;
    fn deref(&self) -> &Vec<::std::string::String> {
        &self.0
    }
}
impl ::std::convert::From<CidrList> for Vec<::std::string::String> {
    fn from(value: CidrList) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CidrList> for CidrList {
    fn from(value: &CidrList) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<Vec<::std::string::String>> for CidrList {
    fn from(value: Vec<::std::string::String>) -> Self {
        Self(value)
    }
}
///Configuration and permission Claims.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Configuration and permission Claims.",
///  "type": "object",
///  "oneOf": [
///    {
///      "description": "User-specific configuration and permissions.",
///      "$ref": "#/$defs/User"
///    },
///    {
///      "description": "Account-specific configuration and permissions.",
///      "$ref": "#/$defs/Account"
///    },
///    {
///      "description": "Activation-specific data.",
///      "$ref": "#/$defs/Activation"
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum Claims {
    User(User),
    Account(Account),
    Activation(Activation),
}
impl ::std::convert::From<&Self> for Claims {
    fn from(value: &Claims) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<User> for Claims {
    fn from(value: User) -> Self {
        Self::User(value)
    }
}
impl ::std::convert::From<Account> for Claims {
    fn from(value: Account) -> Self {
        Self::Account(value)
    }
}
impl ::std::convert::From<Activation> for Claims {
    fn from(value: Activation) -> Self {
        Self::Activation(value)
    }
}
///The type of the claims.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The type of the claims.",
///  "enum": [
///    "account",
///    "activation",
///    "user"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ClaimsType {
    #[serde(rename = "account")]
    Account,
    #[serde(rename = "activation")]
    Activation,
    #[serde(rename = "user")]
    User,
}
impl ::std::convert::From<&Self> for ClaimsType {
    fn from(value: &ClaimsType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for ClaimsType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Account => write!(f, "account"),
            Self::Activation => write!(f, "activation"),
            Self::User => write!(f, "user"),
        }
    }
}
impl ::std::str::FromStr for ClaimsType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "account" => Ok(Self::Account),
            "activation" => Ok(Self::Activation),
            "user" => Ok(Self::User),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ClaimsType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ClaimsType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ClaimsType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Represents how cluster traffic should be handled. Allowed values are empty (no special handling), 'system' (use system accounts), or 'owner' (use owner accounts).
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Represents how cluster traffic should be handled. Allowed values are empty (no special handling), 'system' (use system accounts), or 'owner' (use owner accounts).",
///  "type": "string",
///  "enum": [
///    "",
///    "system",
///    "owner"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ClusterTraffic {
    #[serde(rename = "")]
    X,
    #[serde(rename = "system")]
    System,
    #[serde(rename = "owner")]
    Owner,
}
impl ::std::convert::From<&Self> for ClusterTraffic {
    fn from(value: &ClusterTraffic) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for ClusterTraffic {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::X => write!(f, ""),
            Self::System => write!(f, "system"),
            Self::Owner => write!(f, "owner"),
        }
    }
}
impl ::std::str::FromStr for ClusterTraffic {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "" => Ok(Self::X),
            "system" => Ok(Self::System),
            "owner" => Ok(Self::Owner),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ClusterTraffic {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ClusterTraffic {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ClusterTraffic {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Represents a connection type used by a user, indicating how the client connects to the server.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Represents a connection type used by a user, indicating how the client connects to the server.",
///  "type": "string",
///  "enum": [
///    "STANDARD",
///    "WEBSOCKET",
///    "LEAFNODE",
///    "LEAFNODE_WS",
///    "MQTT",
///    "MQTT_WS",
///    "IN_PROCESS"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ConnectionType {
    #[serde(rename = "STANDARD")]
    Standard,
    #[serde(rename = "WEBSOCKET")]
    Websocket,
    #[serde(rename = "LEAFNODE")]
    Leafnode,
    #[serde(rename = "LEAFNODE_WS")]
    LeafnodeWs,
    #[serde(rename = "MQTT")]
    Mqtt,
    #[serde(rename = "MQTT_WS")]
    MqttWs,
    #[serde(rename = "IN_PROCESS")]
    InProcess,
}
impl ::std::convert::From<&Self> for ConnectionType {
    fn from(value: &ConnectionType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for ConnectionType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Standard => write!(f, "STANDARD"),
            Self::Websocket => write!(f, "WEBSOCKET"),
            Self::Leafnode => write!(f, "LEAFNODE"),
            Self::LeafnodeWs => write!(f, "LEAFNODE_WS"),
            Self::Mqtt => write!(f, "MQTT"),
            Self::MqttWs => write!(f, "MQTT_WS"),
            Self::InProcess => write!(f, "IN_PROCESS"),
        }
    }
}
impl ::std::str::FromStr for ConnectionType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "STANDARD" => Ok(Self::Standard),
            "WEBSOCKET" => Ok(Self::Websocket),
            "LEAFNODE" => Ok(Self::Leafnode),
            "LEAFNODE_WS" => Ok(Self::LeafnodeWs),
            "MQTT" => Ok(Self::Mqtt),
            "MQTT_WS" => Ok(Self::MqttWs),
            "IN_PROCESS" => Ok(Self::InProcess),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ConnectionType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ConnectionType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ConnectionType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Represents an export that allows other accounts to import subjects from this account, possibly requiring tokens.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Represents an export that allows other accounts to import subjects from this account, possibly requiring tokens.",
///  "allOf": [
///    {
///      "$ref": "#/$defs/Info"
///    },
///    {
///      "type": "object",
///      "properties": {
///        "account_token_position": {
///          "description": "If set, references the position of an account token in a wildcard subject (public exports only).",
///          "type": "integer"
///        },
///        "advertise": {
///          "description": "Indicates if this export should be advertised to other accounts.",
///          "type": "boolean"
///        },
///        "allow_trace": {
///          "description": "Allows message tracing if this is a service export.",
///          "type": "boolean"
///        },
///        "name": {
///          "description": "A human-readable name for this export.",
///          "type": "string"
///        },
///        "response_threshold": {
///          "description": "The response threshold in nanoseconds, valid only for services.",
///          "type": "integer"
///        },
///        "response_type": {
///          "description": "The type of response pattern for a service export.",
///          "$ref": "#/$defs/ResponseType"
///        },
///        "revocations": {
///          "description": "A list of revocations for previously issued activations.",
///          "$ref": "#/$defs/RevocationList"
///        },
///        "service_latency": {
///          "description": "Configures latency tracking for services.",
///          "$ref": "#/$defs/ServiceLatency"
///        },
///        "subject": {
///          "description": "The subject being exported.",
///          "$ref": "#/$defs/Subject"
///        },
///        "token_req": {
///          "description": "Indicates if an activation token is required for imports.",
///          "default": false,
///          "type": "boolean"
///        },
///        "type": {
///          "description": "The type of export, either 'stream' or 'service'.",
///          "$ref": "#/$defs/ExportType"
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Export {
    ///If set, references the position of an account token in a wildcard subject (public exports only).
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub account_token_position: ::std::option::Option<i64>,
    ///Indicates if this export should be advertised to other accounts.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub advertise: ::std::option::Option<bool>,
    ///Allows message tracing if this is a service export.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub allow_trace: ::std::option::Option<bool>,
    ///A more detailed description.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    ///URL to find extra info.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub info_url: ::std::option::Option<::std::string::String>,
    ///A human-readable name for this export.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    ///The response threshold in nanoseconds, valid only for services.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub response_threshold: ::std::option::Option<i64>,
    ///The type of response pattern for a service export.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub response_type: ::std::option::Option<ResponseType>,
    ///A list of revocations for previously issued activations.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub revocations: ::std::option::Option<RevocationList>,
    ///Configures latency tracking for services.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub service_latency: ::std::option::Option<ServiceLatency>,
    ///The subject being exported.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subject: ::std::option::Option<Subject>,
    ///Indicates if an activation token is required for imports.
    #[serde(default)]
    pub token_req: bool,
    ///The type of export, either 'stream' or 'service'.
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<ExportType>,
}
impl ::std::convert::From<&Export> for Export {
    fn from(value: &Export) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for Export {
    fn default() -> Self {
        Self {
            account_token_position: Default::default(),
            advertise: Default::default(),
            allow_trace: Default::default(),
            description: Default::default(),
            info_url: Default::default(),
            name: Default::default(),
            response_threshold: Default::default(),
            response_type: Default::default(),
            revocations: Default::default(),
            service_latency: Default::default(),
            subject: Default::default(),
            token_req: Default::default(),
            type_: Default::default(),
        }
    }
}
impl Export {
    pub fn builder() -> builder::Export {
        Default::default()
    }
}
///Represents the type of an export, determining if the export is a stream or a service.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Represents the type of an export, determining if the export is a stream or a service.",
///  "type": "string",
///  "enum": [
///    "stream",
///    "service",
///    "unknown"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ExportType {
    #[serde(rename = "stream")]
    Stream,
    #[serde(rename = "service")]
    Service,
    #[serde(rename = "unknown")]
    Unknown,
}
impl ::std::convert::From<&Self> for ExportType {
    fn from(value: &ExportType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for ExportType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Stream => write!(f, "stream"),
            Self::Service => write!(f, "service"),
            Self::Unknown => write!(f, "unknown"),
        }
    }
}
impl ::std::str::FromStr for ExportType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "stream" => Ok(Self::Stream),
            "service" => Ok(Self::Service),
            "unknown" => Ok(Self::Unknown),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ExportType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ExportType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ExportType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///A list of exports defining subjects that this account makes available to other accounts.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "A list of exports defining subjects that this account makes available to other accounts.",
///  "type": "array",
///  "items": {
///    "$ref": "#/$defs/Export"
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct Exports(pub ::std::vec::Vec<Export>);
impl ::std::ops::Deref for Exports {
    type Target = ::std::vec::Vec<Export>;
    fn deref(&self) -> &::std::vec::Vec<Export> {
        &self.0
    }
}
impl ::std::convert::From<Exports> for ::std::vec::Vec<Export> {
    fn from(value: Exports) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Exports> for Exports {
    fn from(value: &Exports) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::vec::Vec<Export>> for Exports {
    fn from(value: ::std::vec::Vec<Export>) -> Self {
        Self(value)
    }
}
///Configures external authorization for accounts, specifying which users and accounts are involved and optional encryption keys.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Configures external authorization for accounts, specifying which users and accounts are involved and optional encryption keys.",
///  "type": "object",
///  "properties": {
///    "allowed_accounts": {
///      "description": "The list of account public keys that the external authorization service may bind users to, or '*' for any account.",
///      "$ref": "#/$defs/StringList"
///    },
///    "auth_users": {
///      "description": "The list of user public keys allowed to bypass external authorization checks.",
///      "$ref": "#/$defs/StringList"
///    },
///    "xkey": {
///      "description": "An optional public x25519 key for encrypting authorization requests.",
///      "type": "string"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ExternalAuthorization {
    ///The list of account public keys that the external authorization service may bind users to, or '*' for any account.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub allowed_accounts: ::std::option::Option<StringList>,
    ///The list of user public keys allowed to bypass external authorization checks.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub auth_users: ::std::option::Option<StringList>,
    ///An optional public x25519 key for encrypting authorization requests.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub xkey: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&ExternalAuthorization> for ExternalAuthorization {
    fn from(value: &ExternalAuthorization) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for ExternalAuthorization {
    fn default() -> Self {
        Self {
            allowed_accounts: Default::default(),
            auth_users: Default::default(),
            xkey: Default::default(),
        }
    }
}
impl ExternalAuthorization {
    pub fn builder() -> builder::ExternalAuthorization {
        Default::default()
    }
}
///Generic fields shared by multiple kinds of claims.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Generic fields shared by multiple kinds of claims.",
///  "type": "object",
///  "properties": {
///    "tags": {
///      "description": "Tags used to categorize or label this entity.",
///      "$ref": "#/$defs/TagList"
///    },
///    "version": {
///      "description": "The version of the claim.",
///      "default": 2,
///      "type": "integer"
///    }
///  },
///  "$comment": "NOTE: type field omitted as it's const depending on the implementing entity."
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct GenericFields {
    ///Tags used to categorize or label this entity.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tags: ::std::option::Option<TagList>,
    ///The version of the claim.
    #[serde(default = "defaults::default_u64::<i64, 2>")]
    pub version: i64,
}
impl ::std::convert::From<&GenericFields> for GenericFields {
    fn from(value: &GenericFields) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for GenericFields {
    fn default() -> Self {
        Self {
            tags: Default::default(),
            version: defaults::default_u64::<i64, 2>(),
        }
    }
}
impl GenericFields {
    pub fn builder() -> builder::GenericFields {
        Default::default()
    }
}
///Represents a mapping (import) from another account's subject space into this account.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Represents a mapping (import) from another account's subject space into this account.",
///  "type": "object",
///  "properties": {
///    "account": {
///      "description": "The public account key from which this subject is imported.",
///      "type": "string"
///    },
///    "allow_trace": {
///      "description": "Indicates if message tracing is allowed on a stream import.",
///      "type": "boolean"
///    },
///    "local_subject": {
///      "description": "The local subject name to map the imported subject to, potentially using wildcard references.",
///      "$ref": "#/$defs/RenamingSubject"
///    },
///    "name": {
///      "description": "A human-readable name for the import.",
///      "type": "string"
///    },
///    "share": {
///      "description": "Indicates if the service import supports request latency tracking (for services only).",
///      "type": "boolean"
///    },
///    "subject": {
///      "description": "The subject being imported from the exporting account.",
///      "$ref": "#/$defs/Subject"
///    },
///    "to": {
///      "description": "Deprecated: the local subject mapped to the imported subject. Use local_subject instead.",
///      "$ref": "#/$defs/Subject"
///    },
///    "token": {
///      "description": "An activation token enabling the import. May be optional.",
///      "type": "string"
///    },
///    "type": {
///      "description": "The type of import, either 'stream' or 'service'.",
///      "$ref": "#/$defs/ExportType"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Import {
    ///The public account key from which this subject is imported.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub account: ::std::option::Option<::std::string::String>,
    ///Indicates if message tracing is allowed on a stream import.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub allow_trace: ::std::option::Option<bool>,
    ///The local subject name to map the imported subject to, potentially using wildcard references.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub local_subject: ::std::option::Option<RenamingSubject>,
    ///A human-readable name for the import.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    ///Indicates if the service import supports request latency tracking (for services only).
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub share: ::std::option::Option<bool>,
    ///The subject being imported from the exporting account.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subject: ::std::option::Option<Subject>,
    ///Deprecated: the local subject mapped to the imported subject. Use local_subject instead.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub to: ::std::option::Option<Subject>,
    ///An activation token enabling the import. May be optional.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub token: ::std::option::Option<::std::string::String>,
    ///The type of import, either 'stream' or 'service'.
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<ExportType>,
}
impl ::std::convert::From<&Import> for Import {
    fn from(value: &Import) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for Import {
    fn default() -> Self {
        Self {
            account: Default::default(),
            allow_trace: Default::default(),
            local_subject: Default::default(),
            name: Default::default(),
            share: Default::default(),
            subject: Default::default(),
            to: Default::default(),
            token: Default::default(),
            type_: Default::default(),
        }
    }
}
impl Import {
    pub fn builder() -> builder::Import {
        Default::default()
    }
}
///A list of imports, each defining an external subject mapped into this account.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "A list of imports, each defining an external subject mapped into this account.",
///  "type": "array",
///  "items": {
///    "$ref": "#/$defs/Import"
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct Imports(pub ::std::vec::Vec<Import>);
impl ::std::ops::Deref for Imports {
    type Target = ::std::vec::Vec<Import>;
    fn deref(&self) -> &::std::vec::Vec<Import> {
        &self.0
    }
}
impl ::std::convert::From<Imports> for ::std::vec::Vec<Import> {
    fn from(value: Imports) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Imports> for Imports {
    fn from(value: &Imports) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::vec::Vec<Import>> for Imports {
    fn from(value: ::std::vec::Vec<Import>) -> Self {
        Self(value)
    }
}
///Generic extra Info.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Generic extra Info.",
///  "type": "object",
///  "properties": {
///    "description": {
///      "description": "A more detailed description.",
///      "type": "string"
///    },
///    "info_url": {
///      "description": "URL to find extra info.",
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Info {
    ///A more detailed description.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    ///URL to find extra info.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub info_url: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&Info> for Info {
    fn from(value: &Info) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for Info {
    fn default() -> Self {
        Self {
            description: Default::default(),
            info_url: Default::default(),
        }
    }
}
impl Info {
    pub fn builder() -> builder::Info {
        Default::default()
    }
}
///Defines JetStream resource limits for memory, disk, streams, consumers, and other constraints.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Defines JetStream resource limits for memory, disk, streams, consumers, and other constraints.",
///  "type": "object",
///  "properties": {
///    "consumer": {
///      "description": "Maximum number of consumers allowed (-1 indicates no limit).",
///      "default": -1,
///      "type": "integer"
///    },
///    "disk_max_stream_bytes": {
///      "description": "Maximum bytes per disk-backed stream (0 means unlimited).",
///      "default": 0,
///      "type": "integer"
///    },
///    "disk_storage": {
///      "description": "Maximum disk storage in bytes for all streams (0 means disabled).",
///      "default": -1,
///      "type": "integer"
///    },
///    "max_ack_pending": {
///      "description": "Maximum ack pending limit for a stream. If negative, treated as zero.",
///      "default": 0,
///      "type": "integer"
///    },
///    "max_bytes_required": {
///      "description": "If true, all streams must have max_bytes set.",
///      "default": false,
///      "type": "boolean"
///    },
///    "mem_max_stream_bytes": {
///      "description": "Maximum bytes per memory-backed stream (0 means unlimited).",
///      "default": 0,
///      "type": "integer"
///    },
///    "mem_storage": {
///      "description": "Maximum memory storage in bytes for all streams (0 means disabled).",
///      "default": -1,
///      "type": "integer"
///    },
///    "streams": {
///      "description": "Maximum number of streams allowed (-1 indicates no limit).",
///      "default": -1,
///      "type": "integer"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct JetStreamLimits {
    ///Maximum number of consumers allowed (-1 indicates no limit).
    #[serde(default = "defaults::default_i64::<i64, -1>")]
    pub consumer: i64,
    ///Maximum bytes per disk-backed stream (0 means unlimited).
    #[serde(default)]
    pub disk_max_stream_bytes: i64,
    ///Maximum disk storage in bytes for all streams (0 means disabled).
    #[serde(default = "defaults::default_i64::<i64, -1>")]
    pub disk_storage: i64,
    ///Maximum ack pending limit for a stream. If negative, treated as zero.
    #[serde(default)]
    pub max_ack_pending: i64,
    ///If true, all streams must have max_bytes set.
    #[serde(default)]
    pub max_bytes_required: bool,
    ///Maximum bytes per memory-backed stream (0 means unlimited).
    #[serde(default)]
    pub mem_max_stream_bytes: i64,
    ///Maximum memory storage in bytes for all streams (0 means disabled).
    #[serde(default = "defaults::default_i64::<i64, -1>")]
    pub mem_storage: i64,
    ///Maximum number of streams allowed (-1 indicates no limit).
    #[serde(default = "defaults::default_i64::<i64, -1>")]
    pub streams: i64,
}
impl ::std::convert::From<&JetStreamLimits> for JetStreamLimits {
    fn from(value: &JetStreamLimits) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for JetStreamLimits {
    fn default() -> Self {
        Self {
            consumer: defaults::default_i64::<i64, -1>(),
            disk_max_stream_bytes: Default::default(),
            disk_storage: defaults::default_i64::<i64, -1>(),
            max_ack_pending: Default::default(),
            max_bytes_required: Default::default(),
            mem_max_stream_bytes: Default::default(),
            mem_storage: defaults::default_i64::<i64, -1>(),
            streams: defaults::default_i64::<i64, -1>(),
        }
    }
}
impl JetStreamLimits {
    pub fn builder() -> builder::JetStreamLimits {
        Default::default()
    }
}
///A map of tier names to JetStreamLimits, allowing tiered resource allocations for accounts.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "A map of tier names to JetStreamLimits, allowing tiered resource allocations for accounts.",
///  "type": "object",
///  "additionalProperties": {
///    "$ref": "#/$defs/JetStreamLimits"
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct JetStreamTieredLimits(
    pub ::std::collections::HashMap<::std::string::String, JetStreamLimits>,
);
impl ::std::ops::Deref for JetStreamTieredLimits {
    type Target = ::std::collections::HashMap<::std::string::String, JetStreamLimits>;
    fn deref(&self) -> &::std::collections::HashMap<::std::string::String, JetStreamLimits> {
        &self.0
    }
}
impl ::std::convert::From<JetStreamTieredLimits>
    for ::std::collections::HashMap<::std::string::String, JetStreamLimits>
{
    fn from(value: JetStreamTieredLimits) -> Self {
        value.0
    }
}
impl ::std::convert::From<&JetStreamTieredLimits> for JetStreamTieredLimits {
    fn from(value: &JetStreamTieredLimits) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::collections::HashMap<::std::string::String, JetStreamLimits>>
    for JetStreamTieredLimits
{
    fn from(value: ::std::collections::HashMap<::std::string::String, JetStreamLimits>) -> Self {
        Self(value)
    }
}
///The Claims portion of a NATS JWT authorization token.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "$id": "https://github.com/Seraph-Security/nats-jwt/blob/main/src/nats_jwt_schema.json",
///  "title": "Jwt",
///  "description": "The Claims portion of a NATS JWT authorization token.",
///  "type": "object",
///  "required": [
///    "iat",
///    "iss",
///    "jti",
///    "nats",
///    "sub"
///  ],
///  "properties": {
///    "aud": {
///      "description": "The intended audience of the JWT.",
///      "type": "string"
///    },
///    "exp": {
///      "description": "The expiration time (in Unix seconds) after which the JWT is invalid.",
///      "type": "integer"
///    },
///    "iat": {
///      "description": "The time (in Unix seconds) at which the JWT was issued.",
///      "type": "integer"
///    },
///    "iss": {
///      "description": "The issuer of the JWT, typically a public key.",
///      "type": "string"
///    },
///    "jti": {
///      "description": "A unique identifier for the JWT.",
///      "type": "string"
///    },
///    "name": {
///      "description": "A human-readable name for the entity described by the JWT.",
///      "type": "string"
///    },
///    "nats": {
///      "description": "NATS specific portion of the Claims.",
///      "$ref": "#/$defs/Claims"
///    },
///    "nbf": {
///      "description": "The 'not before' time (in Unix seconds) before which the JWT must not be accepted.",
///      "type": "integer"
///    },
///    "sub": {
///      "description": "The subject of the JWT, typically the public key of the entity this JWT describes.",
///      "type": "string"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Jwt {
    ///The intended audience of the JWT.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub aud: ::std::option::Option<::std::string::String>,
    ///The expiration time (in Unix seconds) after which the JWT is invalid.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub exp: ::std::option::Option<i64>,
    ///The time (in Unix seconds) at which the JWT was issued.
    pub iat: i64,
    ///The issuer of the JWT, typically a public key.
    pub iss: ::std::string::String,
    ///A unique identifier for the JWT.
    pub jti: ::std::string::String,
    ///A human-readable name for the entity described by the JWT.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    ///NATS specific portion of the Claims.
    pub nats: Claims,
    ///The 'not before' time (in Unix seconds) before which the JWT must not be accepted.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub nbf: ::std::option::Option<i64>,
    ///The subject of the JWT, typically the public key of the entity this JWT describes.
    pub sub: ::std::string::String,
}
impl ::std::convert::From<&Jwt> for Jwt {
    fn from(value: &Jwt) -> Self {
        value.clone()
    }
}
impl Jwt {
    pub fn builder() -> builder::Jwt {
        Default::default()
    }
}
///The limits for Users including generic NatsLimits and User specific UserLimits
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The limits for Users including generic NatsLimits and User specific UserLimits",
///  "allOf": [
///    {
///      "$ref": "#/$defs/UserLimits"
///    },
///    {
///      "$ref": "#/$defs/NatsLimits"
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Limits {
    ///Max data bytes (-1 for no limit).
    #[serde(default = "defaults::default_i64::<i64, -1>")]
    pub data: i64,
    ///Max message payload size in bytes (-1 for no limit).
    #[serde(default = "defaults::default_i64::<i64, -1>")]
    pub payload: i64,
    ///List of acceptable origin IPs for user.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub src: ::std::option::Option<CidrList>,
    ///Max number of subscriptions (-1 for no limit).
    #[serde(default = "defaults::default_i64::<i64, -1>")]
    pub subs: i64,
    ///Times when connection is allowed.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub times: ::std::vec::Vec<TimeRange>,
    ///Timezone location for the times list.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub times_location: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&Limits> for Limits {
    fn from(value: &Limits) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for Limits {
    fn default() -> Self {
        Self {
            data: defaults::default_i64::<i64, -1>(),
            payload: defaults::default_i64::<i64, -1>(),
            src: Default::default(),
            subs: defaults::default_i64::<i64, -1>(),
            times: Default::default(),
            times_location: Default::default(),
        }
    }
}
impl Limits {
    pub fn builder() -> builder::Limits {
        Default::default()
    }
}
///A mapping from a subject to one or more weighted mappings, allowing splitting or routing messages.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "A mapping from a subject to one or more weighted mappings, allowing splitting or routing messages.",
///  "type": "object",
///  "patternProperties": {
///    ".*": {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/WeightedMapping"
///      }
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct Mapping(pub ::std::collections::HashMap<MappingKey, ::std::vec::Vec<WeightedMapping>>);
impl ::std::ops::Deref for Mapping {
    type Target = ::std::collections::HashMap<MappingKey, ::std::vec::Vec<WeightedMapping>>;
    fn deref(&self) -> &::std::collections::HashMap<MappingKey, ::std::vec::Vec<WeightedMapping>> {
        &self.0
    }
}
impl ::std::convert::From<Mapping>
    for ::std::collections::HashMap<MappingKey, ::std::vec::Vec<WeightedMapping>>
{
    fn from(value: Mapping) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Mapping> for Mapping {
    fn from(value: &Mapping) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::collections::HashMap<MappingKey, ::std::vec::Vec<WeightedMapping>>>
    for Mapping
{
    fn from(
        value: ::std::collections::HashMap<MappingKey, ::std::vec::Vec<WeightedMapping>>,
    ) -> Self {
        Self(value)
    }
}
///MappingKey
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "pattern": ".*"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct MappingKey(::std::string::String);
impl ::std::ops::Deref for MappingKey {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<MappingKey> for ::std::string::String {
    fn from(value: MappingKey) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MappingKey> for MappingKey {
    fn from(value: &MappingKey) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for MappingKey {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if regress::Regex::new(".*").unwrap().find(value).is_none() {
            return Err("doesn't match pattern \".*\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for MappingKey {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for MappingKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for MappingKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for MappingKey {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///Configures distributed message tracing for an account, specifying the destination subject and sampling rate.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Configures distributed message tracing for an account, specifying the destination subject and sampling rate.",
///  "type": "object",
///  "properties": {
///    "dest": {
///      "description": "The subject to which message traces are sent when tracing is triggered.",
///      "$ref": "#/$defs/Subject"
///    },
///    "sampling": {
///      "description": "Probability sampling rate (1-100). Defaults to 100 if not set.",
///      "default": 100,
///      "type": "integer",
///      "maximum": 100.0,
///      "minimum": 1.0
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct MsgTrace {
    ///The subject to which message traces are sent when tracing is triggered.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub dest: ::std::option::Option<Subject>,
    ///Probability sampling rate (1-100). Defaults to 100 if not set.
    #[serde(default = "defaults::default_u64::<i64, 100>")]
    pub sampling: i64,
}
impl ::std::convert::From<&MsgTrace> for MsgTrace {
    fn from(value: &MsgTrace) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for MsgTrace {
    fn default() -> Self {
        Self {
            dest: Default::default(),
            sampling: defaults::default_u64::<i64, 100>(),
        }
    }
}
impl MsgTrace {
    pub fn builder() -> builder::MsgTrace {
        Default::default()
    }
}
///Represents NATS server limits.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Represents NATS server limits.",
///  "type": "object",
///  "properties": {
///    "data": {
///      "description": "Max data bytes (-1 for no limit).",
///      "default": -1,
///      "type": "integer"
///    },
///    "payload": {
///      "description": "Max message payload size in bytes (-1 for no limit).",
///      "default": -1,
///      "type": "integer"
///    },
///    "subs": {
///      "description": "Max number of subscriptions (-1 for no limit).",
///      "default": -1,
///      "type": "integer"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct NatsLimits {
    ///Max data bytes (-1 for no limit).
    #[serde(default = "defaults::default_i64::<i64, -1>")]
    pub data: i64,
    ///Max message payload size in bytes (-1 for no limit).
    #[serde(default = "defaults::default_i64::<i64, -1>")]
    pub payload: i64,
    ///Max number of subscriptions (-1 for no limit).
    #[serde(default = "defaults::default_i64::<i64, -1>")]
    pub subs: i64,
}
impl ::std::convert::From<&NatsLimits> for NatsLimits {
    fn from(value: &NatsLimits) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for NatsLimits {
    fn default() -> Self {
        Self {
            data: defaults::default_i64::<i64, -1>(),
            payload: defaults::default_i64::<i64, -1>(),
            subs: defaults::default_i64::<i64, -1>(),
        }
    }
}
impl NatsLimits {
    pub fn builder() -> builder::NatsLimits {
        Default::default()
    }
}
///Represents all operator-imposed limits on an account, including NATS limits, account limits, and JetStream limits.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Represents all operator-imposed limits on an account, including NATS limits, account limits, and JetStream limits.",
///  "allOf": [
///    {
///      "$ref": "#/$defs/NatsLimits"
///    },
///    {
///      "$ref": "#/$defs/AccountLimits"
///    },
///    {
///      "$ref": "#/$defs/JetStreamLimits"
///    },
///    {
///      "type": "object",
///      "properties": {
///        "tiered_limits": {
///          "description": "Optional tiered JetStream limits.",
///          "$ref": "#/$defs/JetStreamTieredLimits"
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct OperatorLimits {
    ///Max number of active connections
    #[serde(default = "defaults::default_i64::<i64, -1>")]
    pub conn: i64,
    ///Maximum number of consumers allowed (-1 indicates no limit).
    #[serde(default = "defaults::default_i64::<i64, -1>")]
    pub consumer: i64,
    ///Max data bytes (-1 for no limit).
    #[serde(default = "defaults::default_i64::<i64, -1>")]
    pub data: i64,
    ///User JWTs cannot be used as Bearer Tokens.
    #[serde(default)]
    pub disallow_bearer: bool,
    ///Maximum bytes per disk-backed stream (0 means unlimited).
    #[serde(default)]
    pub disk_max_stream_bytes: i64,
    ///Maximum disk storage in bytes for all streams (0 means disabled).
    #[serde(default = "defaults::default_i64::<i64, -1>")]
    pub disk_storage: i64,
    ///Max number of exports (-1 for no limit).
    #[serde(default = "defaults::default_i64::<i64, -1>")]
    pub exports: i64,
    ///Max number of imports (-1 for no limit).
    #[serde(default = "defaults::default_i64::<i64, -1>")]
    pub imports: i64,
    ///Max number of leaf node connections (-1 for no limit).
    #[serde(default = "defaults::default_i64::<i64, -1>")]
    pub leaf: i64,
    ///Maximum ack pending limit for a stream. If negative, treated as zero.
    #[serde(default)]
    pub max_ack_pending: i64,
    ///If true, all streams must have max_bytes set.
    #[serde(default)]
    pub max_bytes_required: bool,
    ///Maximum bytes per memory-backed stream (0 means unlimited).
    #[serde(default)]
    pub mem_max_stream_bytes: i64,
    ///Maximum memory storage in bytes for all streams (0 means disabled).
    #[serde(default = "defaults::default_i64::<i64, -1>")]
    pub mem_storage: i64,
    ///Max message payload size in bytes (-1 for no limit).
    #[serde(default = "defaults::default_i64::<i64, -1>")]
    pub payload: i64,
    ///Maximum number of streams allowed (-1 indicates no limit).
    #[serde(default = "defaults::default_i64::<i64, -1>")]
    pub streams: i64,
    ///Max number of subscriptions (-1 for no limit).
    #[serde(default = "defaults::default_i64::<i64, -1>")]
    pub subs: i64,
    ///Optional tiered JetStream limits.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tiered_limits: ::std::option::Option<JetStreamTieredLimits>,
    ///Whether wildcard exports are allowed.
    #[serde(default = "defaults::default_bool::<true>")]
    pub wildcards: bool,
}
impl ::std::convert::From<&OperatorLimits> for OperatorLimits {
    fn from(value: &OperatorLimits) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for OperatorLimits {
    fn default() -> Self {
        Self {
            conn: defaults::default_i64::<i64, -1>(),
            consumer: defaults::default_i64::<i64, -1>(),
            data: defaults::default_i64::<i64, -1>(),
            disallow_bearer: Default::default(),
            disk_max_stream_bytes: Default::default(),
            disk_storage: defaults::default_i64::<i64, -1>(),
            exports: defaults::default_i64::<i64, -1>(),
            imports: defaults::default_i64::<i64, -1>(),
            leaf: defaults::default_i64::<i64, -1>(),
            max_ack_pending: Default::default(),
            max_bytes_required: Default::default(),
            mem_max_stream_bytes: Default::default(),
            mem_storage: defaults::default_i64::<i64, -1>(),
            payload: defaults::default_i64::<i64, -1>(),
            streams: defaults::default_i64::<i64, -1>(),
            subs: defaults::default_i64::<i64, -1>(),
            tiered_limits: Default::default(),
            wildcards: defaults::default_bool::<true>(),
        }
    }
}
impl OperatorLimits {
    pub fn builder() -> builder::OperatorLimits {
        Default::default()
    }
}
///Defines per-subject publish or subscribe permissions with allow and deny lists.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Defines per-subject publish or subscribe permissions with allow and deny lists.",
///  "type": "object",
///  "properties": {
///    "allow": {
///      "description": "A list of subjects that are explicitly allowed.",
///      "$ref": "#/$defs/StringList"
///    },
///    "deny": {
///      "description": "A list of subjects that are explicitly denied.",
///      "$ref": "#/$defs/StringList"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Permission {
    ///A list of subjects that are explicitly allowed.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub allow: ::std::option::Option<StringList>,
    ///A list of subjects that are explicitly denied.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub deny: ::std::option::Option<StringList>,
}
impl ::std::convert::From<&Permission> for Permission {
    fn from(value: &Permission) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for Permission {
    fn default() -> Self {
        Self {
            allow: Default::default(),
            deny: Default::default(),
        }
    }
}
impl Permission {
    pub fn builder() -> builder::Permission {
        Default::default()
    }
}
///Represents a set of permissions controlling what subjects can be published or subscribed to, and optional response permissions.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Represents a set of permissions controlling what subjects can be published or subscribed to, and optional response permissions.",
///  "type": "object",
///  "properties": {
///    "pub": {
///      "description": "Publication permissions.",
///      "$ref": "#/$defs/Permission"
///    },
///    "resp": {
///      "description": "Response permissions for allowing temporary response subjects.",
///      "$ref": "#/$defs/ResponsePermission"
///    },
///    "sub": {
///      "description": "Subscription permissions.",
///      "$ref": "#/$defs/Permission"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Permissions {
    ///Publication permissions.
    #[serde(
        rename = "pub",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub pub_: ::std::option::Option<Permission>,
    ///Response permissions for allowing temporary response subjects.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub resp: ::std::option::Option<ResponsePermission>,
    ///Subscription permissions.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub sub: ::std::option::Option<Permission>,
}
impl ::std::convert::From<&Permissions> for Permissions {
    fn from(value: &Permissions) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for Permissions {
    fn default() -> Self {
        Self {
            pub_: Default::default(),
            resp: Default::default(),
            sub: Default::default(),
        }
    }
}
impl Permissions {
    pub fn builder() -> builder::Permissions {
        Default::default()
    }
}
///Represents a subject used for renaming imported subjects. Can contain $<number> references to wildcard tokens in the original subject.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Represents a subject used for renaming imported subjects. Can contain $<number> references to wildcard tokens in the original subject.",
///  "type": "string"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize, ::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
#[serde(transparent)]
pub struct RenamingSubject(pub ::std::string::String);
impl ::std::ops::Deref for RenamingSubject {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RenamingSubject> for ::std::string::String {
    fn from(value: RenamingSubject) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RenamingSubject> for RenamingSubject {
    fn from(value: &RenamingSubject) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::string::String> for RenamingSubject {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for RenamingSubject {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for RenamingSubject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
///Specifies how response permissions are handled for a user or account, limiting the number of responses and their TTL.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Specifies how response permissions are handled for a user or account, limiting the number of responses and their TTL.",
///  "type": "object",
///  "properties": {
///    "max": {
///      "description": "The maximum number of responses allowed.",
///      "type": "integer"
///    },
///    "ttl": {
///      "description": "The time-to-live for responses, in nanoseconds.",
///      "type": "integer"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ResponsePermission {
    ///The maximum number of responses allowed.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub max: ::std::option::Option<i64>,
    ///The time-to-live for responses, in nanoseconds.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ttl: ::std::option::Option<i64>,
}
impl ::std::convert::From<&ResponsePermission> for ResponsePermission {
    fn from(value: &ResponsePermission) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for ResponsePermission {
    fn default() -> Self {
        Self {
            max: Default::default(),
            ttl: Default::default(),
        }
    }
}
impl ResponsePermission {
    pub fn builder() -> builder::ResponsePermission {
        Default::default()
    }
}
///Represents the response pattern for a service export: either a 'Singleton' (one response), 'Stream' (multiple responses), or 'Chunked' (one response in chunks).
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Represents the response pattern for a service export: either a 'Singleton' (one response), 'Stream' (multiple responses), or 'Chunked' (one response in chunks).",
///  "type": "string",
///  "enum": [
///    "Singleton",
///    "Stream",
///    "Chunked"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResponseType {
    Singleton,
    Stream,
    Chunked,
}
impl ::std::convert::From<&Self> for ResponseType {
    fn from(value: &ResponseType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for ResponseType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Singleton => write!(f, "Singleton"),
            Self::Stream => write!(f, "Stream"),
            Self::Chunked => write!(f, "Chunked"),
        }
    }
}
impl ::std::str::FromStr for ResponseType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "Singleton" => Ok(Self::Singleton),
            "Stream" => Ok(Self::Stream),
            "Chunked" => Ok(Self::Chunked),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ResponseType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ResponseType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///A mapping of public keys or '*' to revocation timestamps (Unix time). Entries here revoke previously issued JWTs.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "A mapping of public keys or '*' to revocation timestamps (Unix time). Entries here revoke previously issued JWTs.",
///  "type": "object",
///  "patternProperties": {
///    ".*": {
///      "description": "Revocation timestamp for this key or '*'.",
///      "type": "integer"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct RevocationList(pub ::std::collections::HashMap<RevocationListKey, i64>);
impl ::std::ops::Deref for RevocationList {
    type Target = ::std::collections::HashMap<RevocationListKey, i64>;
    fn deref(&self) -> &::std::collections::HashMap<RevocationListKey, i64> {
        &self.0
    }
}
impl ::std::convert::From<RevocationList> for ::std::collections::HashMap<RevocationListKey, i64> {
    fn from(value: RevocationList) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RevocationList> for RevocationList {
    fn from(value: &RevocationList) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::collections::HashMap<RevocationListKey, i64>> for RevocationList {
    fn from(value: ::std::collections::HashMap<RevocationListKey, i64>) -> Self {
        Self(value)
    }
}
///RevocationListKey
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "pattern": ".*"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct RevocationListKey(::std::string::String);
impl ::std::ops::Deref for RevocationListKey {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RevocationListKey> for ::std::string::String {
    fn from(value: RevocationListKey) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RevocationListKey> for RevocationListKey {
    fn from(value: &RevocationListKey) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RevocationListKey {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if regress::Regex::new(".*").unwrap().find(value).is_none() {
            return Err("doesn't match pattern \".*\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for RevocationListKey {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RevocationListKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RevocationListKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RevocationListKey {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///Represents the sampling rate for collecting latency metrics. May be 'headers' or an integer between 1 and 100.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Represents the sampling rate for collecting latency metrics. May be 'headers' or an integer between 1 and 100.",
///  "oneOf": [
///    {
///      "description": "Indicates headers-based sampling ('headers'). If set to 'headers', the NATS server uses request headers for sampling decisions.",
///      "type": "string",
///      "enum": [
///        "headers"
///      ]
///    },
///    {
///      "type": "integer",
///      "maximum": 100.0,
///      "minimum": 1.0
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum SamplingRate {
    Variant0(SamplingRateVariant0),
    Variant1(i64),
}
impl ::std::convert::From<&Self> for SamplingRate {
    fn from(value: &SamplingRate) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<SamplingRateVariant0> for SamplingRate {
    fn from(value: SamplingRateVariant0) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<i64> for SamplingRate {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///Indicates headers-based sampling ('headers'). If set to 'headers', the NATS server uses request headers for sampling decisions.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Indicates headers-based sampling ('headers'). If set to 'headers', the NATS server uses request headers for sampling decisions.",
///  "type": "string",
///  "enum": [
///    "headers"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum SamplingRateVariant0 {
    #[serde(rename = "headers")]
    Headers,
}
impl ::std::convert::From<&Self> for SamplingRateVariant0 {
    fn from(value: &SamplingRateVariant0) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for SamplingRateVariant0 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Headers => write!(f, "headers"),
        }
    }
}
impl ::std::str::FromStr for SamplingRateVariant0 {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "headers" => Ok(Self::Headers),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SamplingRateVariant0 {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for SamplingRateVariant0 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SamplingRateVariant0 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Represents the kind of scope for a signing key, currently supporting a 'user_scope'.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Represents the kind of scope for a signing key, currently supporting a 'user_scope'.",
///  "type": "string",
///  "enum": [
///    "user_scope"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ScopeType {
    #[serde(rename = "user_scope")]
    UserScope,
}
impl ::std::convert::From<&Self> for ScopeType {
    fn from(value: &ScopeType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for ScopeType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::UserScope => write!(f, "user_scope"),
        }
    }
}
impl ::std::str::FromStr for ScopeType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "user_scope" => Ok(Self::UserScope),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ScopeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ScopeType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ScopeType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Configures latency sampling and results reporting for exported services.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Configures latency sampling and results reporting for exported services.",
///  "type": "object",
///  "properties": {
///    "results": {
///      "description": "Subject to which latency results are published. Cannot contain wildcards.",
///      "$ref": "#/$defs/Subject"
///    },
///    "sampling": {
///      "description": "Sampling rate for latency measurements, either 'headers' or a percentage (1-100).",
///      "$ref": "#/$defs/SamplingRate"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ServiceLatency {
    ///Subject to which latency results are published. Cannot contain wildcards.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub results: ::std::option::Option<Subject>,
    ///Sampling rate for latency measurements, either 'headers' or a percentage (1-100).
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub sampling: ::std::option::Option<SamplingRate>,
}
impl ::std::convert::From<&ServiceLatency> for ServiceLatency {
    fn from(value: &ServiceLatency) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for ServiceLatency {
    fn default() -> Self {
        Self {
            results: Default::default(),
            sampling: Default::default(),
        }
    }
}
impl ServiceLatency {
    pub fn builder() -> builder::ServiceLatency {
        Default::default()
    }
}
///Represents a list of signing keys that may be simply strings (for signing keys) or objects (for scoped keys like user_scope).
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Represents a list of signing keys that may be simply strings (for signing keys) or objects (for scoped keys like user_scope).",
///  "type": "array",
///  "items": {
///    "oneOf": [
///      {
///        "title": "PublicKey",
///        "description": "A public account key string representing a non-scoped signing key.",
///        "type": "string"
///      },
///      {
///        "description": "A scoped signing key object.",
///        "$ref": "#/$defs/UserScope"
///      }
///    ]
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct SigningKeys(pub ::std::vec::Vec<SigningKeysItem>);
impl ::std::ops::Deref for SigningKeys {
    type Target = ::std::vec::Vec<SigningKeysItem>;
    fn deref(&self) -> &::std::vec::Vec<SigningKeysItem> {
        &self.0
    }
}
impl ::std::convert::From<SigningKeys> for ::std::vec::Vec<SigningKeysItem> {
    fn from(value: SigningKeys) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SigningKeys> for SigningKeys {
    fn from(value: &SigningKeys) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::vec::Vec<SigningKeysItem>> for SigningKeys {
    fn from(value: ::std::vec::Vec<SigningKeysItem>) -> Self {
        Self(value)
    }
}
///SigningKeysItem
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "title": "PublicKey",
///      "description": "A public account key string representing a non-scoped signing key.",
///      "type": "string"
///    },
///    {
///      "description": "A scoped signing key object.",
///      "$ref": "#/$defs/UserScope"
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum SigningKeysItem {
    PublicKey(::std::string::String),
    UserScope(UserScope),
}
impl ::std::convert::From<&Self> for SigningKeysItem {
    fn from(value: &SigningKeysItem) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<UserScope> for SigningKeysItem {
    fn from(value: UserScope) -> Self {
        Self::UserScope(value)
    }
}
///A list of arbitrary strings.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "A list of arbitrary strings.",
///  "type": "array",
///  "items": {
///    "type": "string"
///  },
///  "uniqueItems": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct StringList(pub ::std::vec::Vec<::std::string::String>);
impl ::std::ops::Deref for StringList {
    type Target = ::std::vec::Vec<::std::string::String>;
    fn deref(&self) -> &::std::vec::Vec<::std::string::String> {
        &self.0
    }
}
impl ::std::convert::From<StringList> for ::std::vec::Vec<::std::string::String> {
    fn from(value: StringList) -> Self {
        value.0
    }
}
impl ::std::convert::From<&StringList> for StringList {
    fn from(value: &StringList) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::vec::Vec<::std::string::String>> for StringList {
    fn from(value: ::std::vec::Vec<::std::string::String>) -> Self {
        Self(value)
    }
}
///Represents a NATS subject, a hierarchical string separated by dots, with optional wildcards ('*' or '>').
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Represents a NATS subject, a hierarchical string separated by dots, with optional wildcards ('*' or '>').",
///  "type": "string"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize, ::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
#[serde(transparent)]
pub struct Subject(pub ::std::string::String);
impl ::std::ops::Deref for Subject {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Subject> for ::std::string::String {
    fn from(value: Subject) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Subject> for Subject {
    fn from(value: &Subject) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::string::String> for Subject {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for Subject {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for Subject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
///A list of tags (strings) that are unique and lowercase. Used to categorize or label entities.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "A list of tags (strings) that are unique and lowercase. Used to categorize or label entities.",
///  "type": "array",
///  "items": {
///    "type": "string"
///  },
///  "uniqueItems": true
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct TagList(pub Vec<::std::string::String>);
impl ::std::ops::Deref for TagList {
    type Target = Vec<::std::string::String>;
    fn deref(&self) -> &Vec<::std::string::String> {
        &self.0
    }
}
impl ::std::convert::From<TagList> for Vec<::std::string::String> {
    fn from(value: TagList) -> Self {
        value.0
    }
}
impl ::std::convert::From<&TagList> for TagList {
    fn from(value: &TagList) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<Vec<::std::string::String>> for TagList {
    fn from(value: Vec<::std::string::String>) -> Self {
        Self(value)
    }
}
///Represents a daily time range with a start and end time in HH:MM:SS format.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Represents a daily time range with a start and end time in HH:MM:SS format.",
///  "type": "object",
///  "required": [
///    "end",
///    "start"
///  ],
///  "properties": {
///    "end": {
///      "description": "The end time in HH:MM:SS format.",
///      "type": "string"
///    },
///    "start": {
///      "description": "The start time in HH:MM:SS format.",
///      "type": "string"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TimeRange {
    ///The end time in HH:MM:SS format.
    pub end: ::std::string::String,
    ///The start time in HH:MM:SS format.
    pub start: ::std::string::String,
}
impl ::std::convert::From<&TimeRange> for TimeRange {
    fn from(value: &TimeRange) -> Self {
        value.clone()
    }
}
impl TimeRange {
    pub fn builder() -> builder::TimeRange {
        Default::default()
    }
}
///Represents user-specific configuration, including permissions, limits, issuer account, and optional bearer tokens.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Represents user-specific configuration, including permissions, limits, issuer account, and optional bearer tokens.",
///  "allOf": [
///    {
///      "$ref": "#/$defs/GenericFields"
///    },
///    {
///      "$ref": "#/$defs/UserPermissionLimits"
///    },
///    {
///      "type": "object",
///      "properties": {
///        "issuer_account": {
///          "description": "The account identity public key if JWT signed by a signing key.",
///          "type": "string"
///        },
///        "type": {
///          "default": "user",
///          "const": "user",
///          "$ref": "#/$defs/ClaimsType"
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct User {
    ///The list of connection types allowed for this user.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub allowed_connection_types: ::std::vec::Vec<ConnectionType>,
    ///If true, this user can be authenticated using a bearer token.
    #[serde(default = "defaults::default_bool::<true>")]
    pub bearer_token: bool,
    ///Max data bytes (-1 for no limit).
    #[serde(default = "defaults::default_i64::<i64, -1>")]
    pub data: i64,
    ///The account identity public key if JWT signed by a signing key.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub issuer_account: ::std::option::Option<::std::string::String>,
    ///Max message payload size in bytes (-1 for no limit).
    #[serde(default = "defaults::default_i64::<i64, -1>")]
    pub payload: i64,
    ///Publication permissions.
    #[serde(
        rename = "pub",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub pub_: ::std::option::Option<Permission>,
    ///Response permissions for allowing temporary response subjects.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub resp: ::std::option::Option<ResponsePermission>,
    ///List of acceptable origin IPs for user.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub src: ::std::option::Option<CidrList>,
    ///Subscription permissions.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub sub: ::std::option::Option<Permission>,
    ///Max number of subscriptions (-1 for no limit).
    #[serde(default = "defaults::default_i64::<i64, -1>")]
    pub subs: i64,
    ///Tags used to categorize or label this entity.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tags: ::std::option::Option<TagList>,
    ///Times when connection is allowed.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub times: ::std::vec::Vec<TimeRange>,
    ///Timezone location for the times list.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub times_location: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type", default = "defaults::user_type")]
    pub type_: UserType,
    ///The version of the claim.
    #[serde(default = "defaults::default_u64::<i64, 2>")]
    pub version: i64,
}
impl ::std::convert::From<&User> for User {
    fn from(value: &User) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for User {
    fn default() -> Self {
        Self {
            allowed_connection_types: Default::default(),
            bearer_token: defaults::default_bool::<true>(),
            data: defaults::default_i64::<i64, -1>(),
            issuer_account: Default::default(),
            payload: defaults::default_i64::<i64, -1>(),
            pub_: Default::default(),
            resp: Default::default(),
            src: Default::default(),
            sub: Default::default(),
            subs: defaults::default_i64::<i64, -1>(),
            tags: Default::default(),
            times: Default::default(),
            times_location: Default::default(),
            type_: defaults::user_type(),
            version: defaults::default_u64::<i64, 2>(),
        }
    }
}
impl User {
    pub fn builder() -> builder::User {
        Default::default()
    }
}
///UserLimits are the limits specific to Users.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "UserLimits are the limits specific to Users.",
///  "type": "object",
///  "properties": {
///    "src": {
///      "description": "List of acceptable origin IPs for user.",
///      "$ref": "#/$defs/CIDRList"
///    },
///    "times": {
///      "description": "Times when connection is allowed.",
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/TimeRange"
///      }
///    },
///    "times_location": {
///      "description": "Timezone location for the times list.",
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct UserLimits {
    ///List of acceptable origin IPs for user.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub src: ::std::option::Option<CidrList>,
    ///Times when connection is allowed.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub times: ::std::vec::Vec<TimeRange>,
    ///Timezone location for the times list.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub times_location: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&UserLimits> for UserLimits {
    fn from(value: &UserLimits) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for UserLimits {
    fn default() -> Self {
        Self {
            src: Default::default(),
            times: Default::default(),
            times_location: Default::default(),
        }
    }
}
impl UserLimits {
    pub fn builder() -> builder::UserLimits {
        Default::default()
    }
}
///Represents the combined permissions and limits assigned to a user, including bearer token and allowed connection types.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Represents the combined permissions and limits assigned to a user, including bearer token and allowed connection types.",
///  "allOf": [
///    {
///      "$ref": "#/$defs/Permissions"
///    },
///    {
///      "$ref": "#/$defs/Limits"
///    },
///    {
///      "type": "object",
///      "properties": {
///        "allowed_connection_types": {
///          "description": "The list of connection types allowed for this user.",
///          "type": "array",
///          "items": {
///            "$ref": "#/$defs/ConnectionType"
///          }
///        },
///        "bearer_token": {
///          "description": "If true, this user can be authenticated using a bearer token.",
///          "default": true,
///          "type": "boolean"
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct UserPermissionLimits {
    ///The list of connection types allowed for this user.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub allowed_connection_types: ::std::vec::Vec<ConnectionType>,
    ///If true, this user can be authenticated using a bearer token.
    #[serde(default = "defaults::default_bool::<true>")]
    pub bearer_token: bool,
    ///Max data bytes (-1 for no limit).
    #[serde(default = "defaults::default_i64::<i64, -1>")]
    pub data: i64,
    ///Max message payload size in bytes (-1 for no limit).
    #[serde(default = "defaults::default_i64::<i64, -1>")]
    pub payload: i64,
    ///Publication permissions.
    #[serde(
        rename = "pub",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub pub_: ::std::option::Option<Permission>,
    ///Response permissions for allowing temporary response subjects.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub resp: ::std::option::Option<ResponsePermission>,
    ///List of acceptable origin IPs for user.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub src: ::std::option::Option<CidrList>,
    ///Subscription permissions.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub sub: ::std::option::Option<Permission>,
    ///Max number of subscriptions (-1 for no limit).
    #[serde(default = "defaults::default_i64::<i64, -1>")]
    pub subs: i64,
    ///Times when connection is allowed.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub times: ::std::vec::Vec<TimeRange>,
    ///Timezone location for the times list.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub times_location: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&UserPermissionLimits> for UserPermissionLimits {
    fn from(value: &UserPermissionLimits) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for UserPermissionLimits {
    fn default() -> Self {
        Self {
            allowed_connection_types: Default::default(),
            bearer_token: defaults::default_bool::<true>(),
            data: defaults::default_i64::<i64, -1>(),
            payload: defaults::default_i64::<i64, -1>(),
            pub_: Default::default(),
            resp: Default::default(),
            src: Default::default(),
            sub: Default::default(),
            subs: defaults::default_i64::<i64, -1>(),
            times: Default::default(),
            times_location: Default::default(),
        }
    }
}
impl UserPermissionLimits {
    pub fn builder() -> builder::UserPermissionLimits {
        Default::default()
    }
}
///Represents a scoped signing key that can only sign user JWTs with certain predefined permission templates.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Represents a scoped signing key that can only sign user JWTs with certain predefined permission templates.",
///  "type": "object",
///  "properties": {
///    "description": {
///      "description": "A human-readable description of this scope.",
///      "type": "string"
///    },
///    "key": {
///      "description": "The public account key that acts as a scoped signer.",
///      "type": "string"
///    },
///    "kind": {
///      "description": "The kind of scope, currently always 'user_scope'.",
///      "default": "user_scope",
///      "$ref": "#/$defs/ScopeType"
///    },
///    "role": {
///      "description": "A user-defined role name to label this scope.",
///      "type": "string"
///    },
///    "template": {
///      "description": "A template of user permission and limits that must not be overridden by signed JWTs.",
///      "$ref": "#/$defs/UserPermissionLimits"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct UserScope {
    ///A human-readable description of this scope.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    ///The public account key that acts as a scoped signer.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub key: ::std::option::Option<::std::string::String>,
    ///The kind of scope, currently always 'user_scope'.
    #[serde(default = "defaults::user_scope_kind")]
    pub kind: ScopeType,
    ///A user-defined role name to label this scope.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub role: ::std::option::Option<::std::string::String>,
    ///A template of user permission and limits that must not be overridden by signed JWTs.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub template: ::std::option::Option<UserPermissionLimits>,
}
impl ::std::convert::From<&UserScope> for UserScope {
    fn from(value: &UserScope) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for UserScope {
    fn default() -> Self {
        Self {
            description: Default::default(),
            key: Default::default(),
            kind: defaults::user_scope_kind(),
            role: Default::default(),
            template: Default::default(),
        }
    }
}
impl UserScope {
    pub fn builder() -> builder::UserScope {
        Default::default()
    }
}
///UserType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "enum": [
///    "user"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum UserType {
    #[serde(rename = "user")]
    User,
}
impl ::std::convert::From<&Self> for UserType {
    fn from(value: &UserType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for UserType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::User => write!(f, "user"),
        }
    }
}
impl ::std::str::FromStr for UserType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "user" => Ok(Self::User),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for UserType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for UserType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UserType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for UserType {
    fn default() -> Self {
        UserType::User
    }
}
///Defines a single weighted mapping target with a subject, optional weight, and optional cluster.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Defines a single weighted mapping target with a subject, optional weight, and optional cluster.",
///  "type": "object",
///  "properties": {
///    "cluster": {
///      "description": "Optional cluster identifier that groups weighted mappings together.",
///      "type": "string"
///    },
///    "subject": {
///      "description": "The subject to which messages are mapped.",
///      "$ref": "#/$defs/Subject"
///    },
///    "weight": {
///      "description": "The relative weight of this mapping, defaults to 100 if not set.",
///      "default": 100,
///      "type": "integer",
///      "minimum": 0.0
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct WeightedMapping {
    ///Optional cluster identifier that groups weighted mappings together.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cluster: ::std::option::Option<::std::string::String>,
    ///The subject to which messages are mapped.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subject: ::std::option::Option<Subject>,
    ///The relative weight of this mapping, defaults to 100 if not set.
    #[serde(default = "defaults::default_u64::<u64, 100>")]
    pub weight: u64,
}
impl ::std::convert::From<&WeightedMapping> for WeightedMapping {
    fn from(value: &WeightedMapping) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for WeightedMapping {
    fn default() -> Self {
        Self {
            cluster: Default::default(),
            subject: Default::default(),
            weight: defaults::default_u64::<u64, 100>(),
        }
    }
}
impl WeightedMapping {
    pub fn builder() -> builder::WeightedMapping {
        Default::default()
    }
}
/// Types for composing complex structures.
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct Account {
        authorization: ::std::result::Result<
            ::std::option::Option<super::ExternalAuthorization>,
            ::std::string::String,
        >,
        cluster_traffic: ::std::result::Result<
            ::std::option::Option<super::ClusterTraffic>,
            ::std::string::String,
        >,
        default_permissions:
            ::std::result::Result<::std::option::Option<super::Permissions>, ::std::string::String>,
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        exports:
            ::std::result::Result<::std::option::Option<super::Exports>, ::std::string::String>,
        imports:
            ::std::result::Result<::std::option::Option<super::Imports>, ::std::string::String>,
        info_url: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        limits: ::std::result::Result<
            ::std::option::Option<super::OperatorLimits>,
            ::std::string::String,
        >,
        mappings:
            ::std::result::Result<::std::option::Option<super::Mapping>, ::std::string::String>,
        revocations: ::std::result::Result<
            ::std::option::Option<super::RevocationList>,
            ::std::string::String,
        >,
        signing_keys:
            ::std::result::Result<::std::option::Option<super::SigningKeys>, ::std::string::String>,
        tags: ::std::result::Result<::std::option::Option<super::TagList>, ::std::string::String>,
        trace: ::std::result::Result<::std::option::Option<super::MsgTrace>, ::std::string::String>,
        type_: ::std::result::Result<super::AccountType, ::std::string::String>,
        version: ::std::result::Result<i64, ::std::string::String>,
    }
    impl ::std::default::Default for Account {
        fn default() -> Self {
            Self {
                authorization: Ok(Default::default()),
                cluster_traffic: Ok(Default::default()),
                default_permissions: Ok(Default::default()),
                description: Ok(Default::default()),
                exports: Ok(Default::default()),
                imports: Ok(Default::default()),
                info_url: Ok(Default::default()),
                limits: Ok(Default::default()),
                mappings: Ok(Default::default()),
                revocations: Ok(Default::default()),
                signing_keys: Ok(Default::default()),
                tags: Ok(Default::default()),
                trace: Ok(Default::default()),
                type_: Ok(super::defaults::account_type()),
                version: Ok(super::defaults::default_u64::<i64, 2>()),
            }
        }
    }
    impl Account {
        pub fn authorization<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ExternalAuthorization>>,
            T::Error: ::std::fmt::Display,
        {
            self.authorization = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for authorization: {}", e));
            self
        }
        pub fn cluster_traffic<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ClusterTraffic>>,
            T::Error: ::std::fmt::Display,
        {
            self.cluster_traffic = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cluster_traffic: {}", e));
            self
        }
        pub fn default_permissions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Permissions>>,
            T::Error: ::std::fmt::Display,
        {
            self.default_permissions = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for default_permissions: {}",
                    e
                )
            });
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn exports<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Exports>>,
            T::Error: ::std::fmt::Display,
        {
            self.exports = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for exports: {}", e));
            self
        }
        pub fn imports<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Imports>>,
            T::Error: ::std::fmt::Display,
        {
            self.imports = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for imports: {}", e));
            self
        }
        pub fn info_url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.info_url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for info_url: {}", e));
            self
        }
        pub fn limits<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::OperatorLimits>>,
            T::Error: ::std::fmt::Display,
        {
            self.limits = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for limits: {}", e));
            self
        }
        pub fn mappings<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Mapping>>,
            T::Error: ::std::fmt::Display,
        {
            self.mappings = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mappings: {}", e));
            self
        }
        pub fn revocations<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::RevocationList>>,
            T::Error: ::std::fmt::Display,
        {
            self.revocations = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for revocations: {}", e));
            self
        }
        pub fn signing_keys<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SigningKeys>>,
            T::Error: ::std::fmt::Display,
        {
            self.signing_keys = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for signing_keys: {}", e));
            self
        }
        pub fn tags<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TagList>>,
            T::Error: ::std::fmt::Display,
        {
            self.tags = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tags: {}", e));
            self
        }
        pub fn trace<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::MsgTrace>>,
            T::Error: ::std::fmt::Display,
        {
            self.trace = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for trace: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AccountType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
        pub fn version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.version = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for version: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Account> for super::Account {
        type Error = super::error::ConversionError;
        fn try_from(value: Account) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                authorization: value.authorization?,
                cluster_traffic: value.cluster_traffic?,
                default_permissions: value.default_permissions?,
                description: value.description?,
                exports: value.exports?,
                imports: value.imports?,
                info_url: value.info_url?,
                limits: value.limits?,
                mappings: value.mappings?,
                revocations: value.revocations?,
                signing_keys: value.signing_keys?,
                tags: value.tags?,
                trace: value.trace?,
                type_: value.type_?,
                version: value.version?,
            })
        }
    }
    impl ::std::convert::From<super::Account> for Account {
        fn from(value: super::Account) -> Self {
            Self {
                authorization: Ok(value.authorization),
                cluster_traffic: Ok(value.cluster_traffic),
                default_permissions: Ok(value.default_permissions),
                description: Ok(value.description),
                exports: Ok(value.exports),
                imports: Ok(value.imports),
                info_url: Ok(value.info_url),
                limits: Ok(value.limits),
                mappings: Ok(value.mappings),
                revocations: Ok(value.revocations),
                signing_keys: Ok(value.signing_keys),
                tags: Ok(value.tags),
                trace: Ok(value.trace),
                type_: Ok(value.type_),
                version: Ok(value.version),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AccountLimits {
        conn: ::std::result::Result<i64, ::std::string::String>,
        disallow_bearer: ::std::result::Result<bool, ::std::string::String>,
        exports: ::std::result::Result<i64, ::std::string::String>,
        imports: ::std::result::Result<i64, ::std::string::String>,
        leaf: ::std::result::Result<i64, ::std::string::String>,
        wildcards: ::std::result::Result<bool, ::std::string::String>,
    }
    impl ::std::default::Default for AccountLimits {
        fn default() -> Self {
            Self {
                conn: Ok(super::defaults::default_i64::<i64, -1>()),
                disallow_bearer: Ok(Default::default()),
                exports: Ok(super::defaults::default_i64::<i64, -1>()),
                imports: Ok(super::defaults::default_i64::<i64, -1>()),
                leaf: Ok(super::defaults::default_i64::<i64, -1>()),
                wildcards: Ok(super::defaults::default_bool::<true>()),
            }
        }
    }
    impl AccountLimits {
        pub fn conn<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.conn = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for conn: {}", e));
            self
        }
        pub fn disallow_bearer<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.disallow_bearer = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for disallow_bearer: {}", e));
            self
        }
        pub fn exports<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.exports = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for exports: {}", e));
            self
        }
        pub fn imports<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.imports = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for imports: {}", e));
            self
        }
        pub fn leaf<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.leaf = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for leaf: {}", e));
            self
        }
        pub fn wildcards<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.wildcards = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for wildcards: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<AccountLimits> for super::AccountLimits {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AccountLimits,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                conn: value.conn?,
                disallow_bearer: value.disallow_bearer?,
                exports: value.exports?,
                imports: value.imports?,
                leaf: value.leaf?,
                wildcards: value.wildcards?,
            })
        }
    }
    impl ::std::convert::From<super::AccountLimits> for AccountLimits {
        fn from(value: super::AccountLimits) -> Self {
            Self {
                conn: Ok(value.conn),
                disallow_bearer: Ok(value.disallow_bearer),
                exports: Ok(value.exports),
                imports: Ok(value.imports),
                leaf: Ok(value.leaf),
                wildcards: Ok(value.wildcards),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Activation {
        issuer_account: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        kind:
            ::std::result::Result<::std::option::Option<super::ExportType>, ::std::string::String>,
        subject:
            ::std::result::Result<::std::option::Option<super::Subject>, ::std::string::String>,
        tags: ::std::result::Result<::std::option::Option<super::TagList>, ::std::string::String>,
        type_: ::std::result::Result<super::ActivationType, ::std::string::String>,
        version: ::std::result::Result<i64, ::std::string::String>,
    }
    impl ::std::default::Default for Activation {
        fn default() -> Self {
            Self {
                issuer_account: Ok(Default::default()),
                kind: Ok(Default::default()),
                subject: Ok(Default::default()),
                tags: Ok(Default::default()),
                type_: Ok(super::defaults::activation_type()),
                version: Ok(super::defaults::default_u64::<i64, 2>()),
            }
        }
    }
    impl Activation {
        pub fn issuer_account<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.issuer_account = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for issuer_account: {}", e));
            self
        }
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ExportType>>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
            self
        }
        pub fn subject<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Subject>>,
            T::Error: ::std::fmt::Display,
        {
            self.subject = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subject: {}", e));
            self
        }
        pub fn tags<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TagList>>,
            T::Error: ::std::fmt::Display,
        {
            self.tags = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tags: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ActivationType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
        pub fn version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.version = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for version: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Activation> for super::Activation {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Activation,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                issuer_account: value.issuer_account?,
                kind: value.kind?,
                subject: value.subject?,
                tags: value.tags?,
                type_: value.type_?,
                version: value.version?,
            })
        }
    }
    impl ::std::convert::From<super::Activation> for Activation {
        fn from(value: super::Activation) -> Self {
            Self {
                issuer_account: Ok(value.issuer_account),
                kind: Ok(value.kind),
                subject: Ok(value.subject),
                tags: Ok(value.tags),
                type_: Ok(value.type_),
                version: Ok(value.version),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Export {
        account_token_position:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        advertise: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        allow_trace: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        info_url: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        response_threshold:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        response_type: ::std::result::Result<
            ::std::option::Option<super::ResponseType>,
            ::std::string::String,
        >,
        revocations: ::std::result::Result<
            ::std::option::Option<super::RevocationList>,
            ::std::string::String,
        >,
        service_latency: ::std::result::Result<
            ::std::option::Option<super::ServiceLatency>,
            ::std::string::String,
        >,
        subject:
            ::std::result::Result<::std::option::Option<super::Subject>, ::std::string::String>,
        token_req: ::std::result::Result<bool, ::std::string::String>,
        type_:
            ::std::result::Result<::std::option::Option<super::ExportType>, ::std::string::String>,
    }
    impl ::std::default::Default for Export {
        fn default() -> Self {
            Self {
                account_token_position: Ok(Default::default()),
                advertise: Ok(Default::default()),
                allow_trace: Ok(Default::default()),
                description: Ok(Default::default()),
                info_url: Ok(Default::default()),
                name: Ok(Default::default()),
                response_threshold: Ok(Default::default()),
                response_type: Ok(Default::default()),
                revocations: Ok(Default::default()),
                service_latency: Ok(Default::default()),
                subject: Ok(Default::default()),
                token_req: Ok(Default::default()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl Export {
        pub fn account_token_position<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.account_token_position = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for account_token_position: {}",
                    e
                )
            });
            self
        }
        pub fn advertise<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.advertise = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for advertise: {}", e));
            self
        }
        pub fn allow_trace<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.allow_trace = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for allow_trace: {}", e));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn info_url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.info_url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for info_url: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn response_threshold<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.response_threshold = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for response_threshold: {}",
                    e
                )
            });
            self
        }
        pub fn response_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ResponseType>>,
            T::Error: ::std::fmt::Display,
        {
            self.response_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for response_type: {}", e));
            self
        }
        pub fn revocations<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::RevocationList>>,
            T::Error: ::std::fmt::Display,
        {
            self.revocations = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for revocations: {}", e));
            self
        }
        pub fn service_latency<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ServiceLatency>>,
            T::Error: ::std::fmt::Display,
        {
            self.service_latency = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for service_latency: {}", e));
            self
        }
        pub fn subject<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Subject>>,
            T::Error: ::std::fmt::Display,
        {
            self.subject = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subject: {}", e));
            self
        }
        pub fn token_req<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.token_req = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for token_req: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ExportType>>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Export> for super::Export {
        type Error = super::error::ConversionError;
        fn try_from(value: Export) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                account_token_position: value.account_token_position?,
                advertise: value.advertise?,
                allow_trace: value.allow_trace?,
                description: value.description?,
                info_url: value.info_url?,
                name: value.name?,
                response_threshold: value.response_threshold?,
                response_type: value.response_type?,
                revocations: value.revocations?,
                service_latency: value.service_latency?,
                subject: value.subject?,
                token_req: value.token_req?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::Export> for Export {
        fn from(value: super::Export) -> Self {
            Self {
                account_token_position: Ok(value.account_token_position),
                advertise: Ok(value.advertise),
                allow_trace: Ok(value.allow_trace),
                description: Ok(value.description),
                info_url: Ok(value.info_url),
                name: Ok(value.name),
                response_threshold: Ok(value.response_threshold),
                response_type: Ok(value.response_type),
                revocations: Ok(value.revocations),
                service_latency: Ok(value.service_latency),
                subject: Ok(value.subject),
                token_req: Ok(value.token_req),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ExternalAuthorization {
        allowed_accounts:
            ::std::result::Result<::std::option::Option<super::StringList>, ::std::string::String>,
        auth_users:
            ::std::result::Result<::std::option::Option<super::StringList>, ::std::string::String>,
        xkey: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ExternalAuthorization {
        fn default() -> Self {
            Self {
                allowed_accounts: Ok(Default::default()),
                auth_users: Ok(Default::default()),
                xkey: Ok(Default::default()),
            }
        }
    }
    impl ExternalAuthorization {
        pub fn allowed_accounts<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::StringList>>,
            T::Error: ::std::fmt::Display,
        {
            self.allowed_accounts = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for allowed_accounts: {}",
                    e
                )
            });
            self
        }
        pub fn auth_users<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::StringList>>,
            T::Error: ::std::fmt::Display,
        {
            self.auth_users = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for auth_users: {}", e));
            self
        }
        pub fn xkey<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.xkey = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for xkey: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ExternalAuthorization> for super::ExternalAuthorization {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ExternalAuthorization,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                allowed_accounts: value.allowed_accounts?,
                auth_users: value.auth_users?,
                xkey: value.xkey?,
            })
        }
    }
    impl ::std::convert::From<super::ExternalAuthorization> for ExternalAuthorization {
        fn from(value: super::ExternalAuthorization) -> Self {
            Self {
                allowed_accounts: Ok(value.allowed_accounts),
                auth_users: Ok(value.auth_users),
                xkey: Ok(value.xkey),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GenericFields {
        tags: ::std::result::Result<::std::option::Option<super::TagList>, ::std::string::String>,
        version: ::std::result::Result<i64, ::std::string::String>,
    }
    impl ::std::default::Default for GenericFields {
        fn default() -> Self {
            Self {
                tags: Ok(Default::default()),
                version: Ok(super::defaults::default_u64::<i64, 2>()),
            }
        }
    }
    impl GenericFields {
        pub fn tags<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TagList>>,
            T::Error: ::std::fmt::Display,
        {
            self.tags = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tags: {}", e));
            self
        }
        pub fn version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.version = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for version: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<GenericFields> for super::GenericFields {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GenericFields,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                tags: value.tags?,
                version: value.version?,
            })
        }
    }
    impl ::std::convert::From<super::GenericFields> for GenericFields {
        fn from(value: super::GenericFields) -> Self {
            Self {
                tags: Ok(value.tags),
                version: Ok(value.version),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Import {
        account: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        allow_trace: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        local_subject: ::std::result::Result<
            ::std::option::Option<super::RenamingSubject>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        share: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        subject:
            ::std::result::Result<::std::option::Option<super::Subject>, ::std::string::String>,
        to: ::std::result::Result<::std::option::Option<super::Subject>, ::std::string::String>,
        token: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_:
            ::std::result::Result<::std::option::Option<super::ExportType>, ::std::string::String>,
    }
    impl ::std::default::Default for Import {
        fn default() -> Self {
            Self {
                account: Ok(Default::default()),
                allow_trace: Ok(Default::default()),
                local_subject: Ok(Default::default()),
                name: Ok(Default::default()),
                share: Ok(Default::default()),
                subject: Ok(Default::default()),
                to: Ok(Default::default()),
                token: Ok(Default::default()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl Import {
        pub fn account<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.account = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for account: {}", e));
            self
        }
        pub fn allow_trace<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.allow_trace = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for allow_trace: {}", e));
            self
        }
        pub fn local_subject<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::RenamingSubject>>,
            T::Error: ::std::fmt::Display,
        {
            self.local_subject = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for local_subject: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn share<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.share = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for share: {}", e));
            self
        }
        pub fn subject<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Subject>>,
            T::Error: ::std::fmt::Display,
        {
            self.subject = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subject: {}", e));
            self
        }
        pub fn to<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Subject>>,
            T::Error: ::std::fmt::Display,
        {
            self.to = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for to: {}", e));
            self
        }
        pub fn token<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.token = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for token: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ExportType>>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Import> for super::Import {
        type Error = super::error::ConversionError;
        fn try_from(value: Import) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                account: value.account?,
                allow_trace: value.allow_trace?,
                local_subject: value.local_subject?,
                name: value.name?,
                share: value.share?,
                subject: value.subject?,
                to: value.to?,
                token: value.token?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::Import> for Import {
        fn from(value: super::Import) -> Self {
            Self {
                account: Ok(value.account),
                allow_trace: Ok(value.allow_trace),
                local_subject: Ok(value.local_subject),
                name: Ok(value.name),
                share: Ok(value.share),
                subject: Ok(value.subject),
                to: Ok(value.to),
                token: Ok(value.token),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Info {
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        info_url: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Info {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
                info_url: Ok(Default::default()),
            }
        }
    }
    impl Info {
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn info_url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.info_url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for info_url: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Info> for super::Info {
        type Error = super::error::ConversionError;
        fn try_from(value: Info) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                info_url: value.info_url?,
            })
        }
    }
    impl ::std::convert::From<super::Info> for Info {
        fn from(value: super::Info) -> Self {
            Self {
                description: Ok(value.description),
                info_url: Ok(value.info_url),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct JetStreamLimits {
        consumer: ::std::result::Result<i64, ::std::string::String>,
        disk_max_stream_bytes: ::std::result::Result<i64, ::std::string::String>,
        disk_storage: ::std::result::Result<i64, ::std::string::String>,
        max_ack_pending: ::std::result::Result<i64, ::std::string::String>,
        max_bytes_required: ::std::result::Result<bool, ::std::string::String>,
        mem_max_stream_bytes: ::std::result::Result<i64, ::std::string::String>,
        mem_storage: ::std::result::Result<i64, ::std::string::String>,
        streams: ::std::result::Result<i64, ::std::string::String>,
    }
    impl ::std::default::Default for JetStreamLimits {
        fn default() -> Self {
            Self {
                consumer: Ok(super::defaults::default_i64::<i64, -1>()),
                disk_max_stream_bytes: Ok(Default::default()),
                disk_storage: Ok(super::defaults::default_i64::<i64, -1>()),
                max_ack_pending: Ok(Default::default()),
                max_bytes_required: Ok(Default::default()),
                mem_max_stream_bytes: Ok(Default::default()),
                mem_storage: Ok(super::defaults::default_i64::<i64, -1>()),
                streams: Ok(super::defaults::default_i64::<i64, -1>()),
            }
        }
    }
    impl JetStreamLimits {
        pub fn consumer<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.consumer = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for consumer: {}", e));
            self
        }
        pub fn disk_max_stream_bytes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.disk_max_stream_bytes = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for disk_max_stream_bytes: {}",
                    e
                )
            });
            self
        }
        pub fn disk_storage<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.disk_storage = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for disk_storage: {}", e));
            self
        }
        pub fn max_ack_pending<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.max_ack_pending = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for max_ack_pending: {}", e));
            self
        }
        pub fn max_bytes_required<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.max_bytes_required = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for max_bytes_required: {}",
                    e
                )
            });
            self
        }
        pub fn mem_max_stream_bytes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.mem_max_stream_bytes = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for mem_max_stream_bytes: {}",
                    e
                )
            });
            self
        }
        pub fn mem_storage<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.mem_storage = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mem_storage: {}", e));
            self
        }
        pub fn streams<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.streams = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for streams: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<JetStreamLimits> for super::JetStreamLimits {
        type Error = super::error::ConversionError;
        fn try_from(
            value: JetStreamLimits,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                consumer: value.consumer?,
                disk_max_stream_bytes: value.disk_max_stream_bytes?,
                disk_storage: value.disk_storage?,
                max_ack_pending: value.max_ack_pending?,
                max_bytes_required: value.max_bytes_required?,
                mem_max_stream_bytes: value.mem_max_stream_bytes?,
                mem_storage: value.mem_storage?,
                streams: value.streams?,
            })
        }
    }
    impl ::std::convert::From<super::JetStreamLimits> for JetStreamLimits {
        fn from(value: super::JetStreamLimits) -> Self {
            Self {
                consumer: Ok(value.consumer),
                disk_max_stream_bytes: Ok(value.disk_max_stream_bytes),
                disk_storage: Ok(value.disk_storage),
                max_ack_pending: Ok(value.max_ack_pending),
                max_bytes_required: Ok(value.max_bytes_required),
                mem_max_stream_bytes: Ok(value.mem_max_stream_bytes),
                mem_storage: Ok(value.mem_storage),
                streams: Ok(value.streams),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Jwt {
        aud: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        exp: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        iat: ::std::result::Result<i64, ::std::string::String>,
        iss: ::std::result::Result<::std::string::String, ::std::string::String>,
        jti: ::std::result::Result<::std::string::String, ::std::string::String>,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        nats: ::std::result::Result<super::Claims, ::std::string::String>,
        nbf: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        sub: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for Jwt {
        fn default() -> Self {
            Self {
                aud: Ok(Default::default()),
                exp: Ok(Default::default()),
                iat: Err("no value supplied for iat".to_string()),
                iss: Err("no value supplied for iss".to_string()),
                jti: Err("no value supplied for jti".to_string()),
                name: Ok(Default::default()),
                nats: Err("no value supplied for nats".to_string()),
                nbf: Ok(Default::default()),
                sub: Err("no value supplied for sub".to_string()),
            }
        }
    }
    impl Jwt {
        pub fn aud<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.aud = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for aud: {}", e));
            self
        }
        pub fn exp<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.exp = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for exp: {}", e));
            self
        }
        pub fn iat<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.iat = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for iat: {}", e));
            self
        }
        pub fn iss<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.iss = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for iss: {}", e));
            self
        }
        pub fn jti<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.jti = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for jti: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn nats<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Claims>,
            T::Error: ::std::fmt::Display,
        {
            self.nats = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for nats: {}", e));
            self
        }
        pub fn nbf<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.nbf = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for nbf: {}", e));
            self
        }
        pub fn sub<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.sub = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sub: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Jwt> for super::Jwt {
        type Error = super::error::ConversionError;
        fn try_from(value: Jwt) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                aud: value.aud?,
                exp: value.exp?,
                iat: value.iat?,
                iss: value.iss?,
                jti: value.jti?,
                name: value.name?,
                nats: value.nats?,
                nbf: value.nbf?,
                sub: value.sub?,
            })
        }
    }
    impl ::std::convert::From<super::Jwt> for Jwt {
        fn from(value: super::Jwt) -> Self {
            Self {
                aud: Ok(value.aud),
                exp: Ok(value.exp),
                iat: Ok(value.iat),
                iss: Ok(value.iss),
                jti: Ok(value.jti),
                name: Ok(value.name),
                nats: Ok(value.nats),
                nbf: Ok(value.nbf),
                sub: Ok(value.sub),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Limits {
        data: ::std::result::Result<i64, ::std::string::String>,
        payload: ::std::result::Result<i64, ::std::string::String>,
        src: ::std::result::Result<::std::option::Option<super::CidrList>, ::std::string::String>,
        subs: ::std::result::Result<i64, ::std::string::String>,
        times: ::std::result::Result<::std::vec::Vec<super::TimeRange>, ::std::string::String>,
        times_location: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Limits {
        fn default() -> Self {
            Self {
                data: Ok(super::defaults::default_i64::<i64, -1>()),
                payload: Ok(super::defaults::default_i64::<i64, -1>()),
                src: Ok(Default::default()),
                subs: Ok(super::defaults::default_i64::<i64, -1>()),
                times: Ok(Default::default()),
                times_location: Ok(Default::default()),
            }
        }
    }
    impl Limits {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {}", e));
            self
        }
        pub fn payload<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.payload = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for payload: {}", e));
            self
        }
        pub fn src<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CidrList>>,
            T::Error: ::std::fmt::Display,
        {
            self.src = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for src: {}", e));
            self
        }
        pub fn subs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.subs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subs: {}", e));
            self
        }
        pub fn times<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::TimeRange>>,
            T::Error: ::std::fmt::Display,
        {
            self.times = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for times: {}", e));
            self
        }
        pub fn times_location<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.times_location = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for times_location: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Limits> for super::Limits {
        type Error = super::error::ConversionError;
        fn try_from(value: Limits) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                data: value.data?,
                payload: value.payload?,
                src: value.src?,
                subs: value.subs?,
                times: value.times?,
                times_location: value.times_location?,
            })
        }
    }
    impl ::std::convert::From<super::Limits> for Limits {
        fn from(value: super::Limits) -> Self {
            Self {
                data: Ok(value.data),
                payload: Ok(value.payload),
                src: Ok(value.src),
                subs: Ok(value.subs),
                times: Ok(value.times),
                times_location: Ok(value.times_location),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MsgTrace {
        dest: ::std::result::Result<::std::option::Option<super::Subject>, ::std::string::String>,
        sampling: ::std::result::Result<i64, ::std::string::String>,
    }
    impl ::std::default::Default for MsgTrace {
        fn default() -> Self {
            Self {
                dest: Ok(Default::default()),
                sampling: Ok(super::defaults::default_u64::<i64, 100>()),
            }
        }
    }
    impl MsgTrace {
        pub fn dest<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Subject>>,
            T::Error: ::std::fmt::Display,
        {
            self.dest = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for dest: {}", e));
            self
        }
        pub fn sampling<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.sampling = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sampling: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<MsgTrace> for super::MsgTrace {
        type Error = super::error::ConversionError;
        fn try_from(value: MsgTrace) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                dest: value.dest?,
                sampling: value.sampling?,
            })
        }
    }
    impl ::std::convert::From<super::MsgTrace> for MsgTrace {
        fn from(value: super::MsgTrace) -> Self {
            Self {
                dest: Ok(value.dest),
                sampling: Ok(value.sampling),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct NatsLimits {
        data: ::std::result::Result<i64, ::std::string::String>,
        payload: ::std::result::Result<i64, ::std::string::String>,
        subs: ::std::result::Result<i64, ::std::string::String>,
    }
    impl ::std::default::Default for NatsLimits {
        fn default() -> Self {
            Self {
                data: Ok(super::defaults::default_i64::<i64, -1>()),
                payload: Ok(super::defaults::default_i64::<i64, -1>()),
                subs: Ok(super::defaults::default_i64::<i64, -1>()),
            }
        }
    }
    impl NatsLimits {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {}", e));
            self
        }
        pub fn payload<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.payload = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for payload: {}", e));
            self
        }
        pub fn subs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.subs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subs: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<NatsLimits> for super::NatsLimits {
        type Error = super::error::ConversionError;
        fn try_from(
            value: NatsLimits,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                data: value.data?,
                payload: value.payload?,
                subs: value.subs?,
            })
        }
    }
    impl ::std::convert::From<super::NatsLimits> for NatsLimits {
        fn from(value: super::NatsLimits) -> Self {
            Self {
                data: Ok(value.data),
                payload: Ok(value.payload),
                subs: Ok(value.subs),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OperatorLimits {
        conn: ::std::result::Result<i64, ::std::string::String>,
        consumer: ::std::result::Result<i64, ::std::string::String>,
        data: ::std::result::Result<i64, ::std::string::String>,
        disallow_bearer: ::std::result::Result<bool, ::std::string::String>,
        disk_max_stream_bytes: ::std::result::Result<i64, ::std::string::String>,
        disk_storage: ::std::result::Result<i64, ::std::string::String>,
        exports: ::std::result::Result<i64, ::std::string::String>,
        imports: ::std::result::Result<i64, ::std::string::String>,
        leaf: ::std::result::Result<i64, ::std::string::String>,
        max_ack_pending: ::std::result::Result<i64, ::std::string::String>,
        max_bytes_required: ::std::result::Result<bool, ::std::string::String>,
        mem_max_stream_bytes: ::std::result::Result<i64, ::std::string::String>,
        mem_storage: ::std::result::Result<i64, ::std::string::String>,
        payload: ::std::result::Result<i64, ::std::string::String>,
        streams: ::std::result::Result<i64, ::std::string::String>,
        subs: ::std::result::Result<i64, ::std::string::String>,
        tiered_limits: ::std::result::Result<
            ::std::option::Option<super::JetStreamTieredLimits>,
            ::std::string::String,
        >,
        wildcards: ::std::result::Result<bool, ::std::string::String>,
    }
    impl ::std::default::Default for OperatorLimits {
        fn default() -> Self {
            Self {
                conn: Ok(super::defaults::default_i64::<i64, -1>()),
                consumer: Ok(super::defaults::default_i64::<i64, -1>()),
                data: Ok(super::defaults::default_i64::<i64, -1>()),
                disallow_bearer: Ok(Default::default()),
                disk_max_stream_bytes: Ok(Default::default()),
                disk_storage: Ok(super::defaults::default_i64::<i64, -1>()),
                exports: Ok(super::defaults::default_i64::<i64, -1>()),
                imports: Ok(super::defaults::default_i64::<i64, -1>()),
                leaf: Ok(super::defaults::default_i64::<i64, -1>()),
                max_ack_pending: Ok(Default::default()),
                max_bytes_required: Ok(Default::default()),
                mem_max_stream_bytes: Ok(Default::default()),
                mem_storage: Ok(super::defaults::default_i64::<i64, -1>()),
                payload: Ok(super::defaults::default_i64::<i64, -1>()),
                streams: Ok(super::defaults::default_i64::<i64, -1>()),
                subs: Ok(super::defaults::default_i64::<i64, -1>()),
                tiered_limits: Ok(Default::default()),
                wildcards: Ok(super::defaults::default_bool::<true>()),
            }
        }
    }
    impl OperatorLimits {
        pub fn conn<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.conn = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for conn: {}", e));
            self
        }
        pub fn consumer<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.consumer = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for consumer: {}", e));
            self
        }
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {}", e));
            self
        }
        pub fn disallow_bearer<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.disallow_bearer = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for disallow_bearer: {}", e));
            self
        }
        pub fn disk_max_stream_bytes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.disk_max_stream_bytes = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for disk_max_stream_bytes: {}",
                    e
                )
            });
            self
        }
        pub fn disk_storage<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.disk_storage = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for disk_storage: {}", e));
            self
        }
        pub fn exports<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.exports = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for exports: {}", e));
            self
        }
        pub fn imports<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.imports = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for imports: {}", e));
            self
        }
        pub fn leaf<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.leaf = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for leaf: {}", e));
            self
        }
        pub fn max_ack_pending<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.max_ack_pending = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for max_ack_pending: {}", e));
            self
        }
        pub fn max_bytes_required<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.max_bytes_required = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for max_bytes_required: {}",
                    e
                )
            });
            self
        }
        pub fn mem_max_stream_bytes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.mem_max_stream_bytes = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for mem_max_stream_bytes: {}",
                    e
                )
            });
            self
        }
        pub fn mem_storage<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.mem_storage = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mem_storage: {}", e));
            self
        }
        pub fn payload<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.payload = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for payload: {}", e));
            self
        }
        pub fn streams<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.streams = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for streams: {}", e));
            self
        }
        pub fn subs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.subs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subs: {}", e));
            self
        }
        pub fn tiered_limits<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::JetStreamTieredLimits>>,
            T::Error: ::std::fmt::Display,
        {
            self.tiered_limits = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tiered_limits: {}", e));
            self
        }
        pub fn wildcards<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.wildcards = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for wildcards: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<OperatorLimits> for super::OperatorLimits {
        type Error = super::error::ConversionError;
        fn try_from(
            value: OperatorLimits,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                conn: value.conn?,
                consumer: value.consumer?,
                data: value.data?,
                disallow_bearer: value.disallow_bearer?,
                disk_max_stream_bytes: value.disk_max_stream_bytes?,
                disk_storage: value.disk_storage?,
                exports: value.exports?,
                imports: value.imports?,
                leaf: value.leaf?,
                max_ack_pending: value.max_ack_pending?,
                max_bytes_required: value.max_bytes_required?,
                mem_max_stream_bytes: value.mem_max_stream_bytes?,
                mem_storage: value.mem_storage?,
                payload: value.payload?,
                streams: value.streams?,
                subs: value.subs?,
                tiered_limits: value.tiered_limits?,
                wildcards: value.wildcards?,
            })
        }
    }
    impl ::std::convert::From<super::OperatorLimits> for OperatorLimits {
        fn from(value: super::OperatorLimits) -> Self {
            Self {
                conn: Ok(value.conn),
                consumer: Ok(value.consumer),
                data: Ok(value.data),
                disallow_bearer: Ok(value.disallow_bearer),
                disk_max_stream_bytes: Ok(value.disk_max_stream_bytes),
                disk_storage: Ok(value.disk_storage),
                exports: Ok(value.exports),
                imports: Ok(value.imports),
                leaf: Ok(value.leaf),
                max_ack_pending: Ok(value.max_ack_pending),
                max_bytes_required: Ok(value.max_bytes_required),
                mem_max_stream_bytes: Ok(value.mem_max_stream_bytes),
                mem_storage: Ok(value.mem_storage),
                payload: Ok(value.payload),
                streams: Ok(value.streams),
                subs: Ok(value.subs),
                tiered_limits: Ok(value.tiered_limits),
                wildcards: Ok(value.wildcards),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Permission {
        allow:
            ::std::result::Result<::std::option::Option<super::StringList>, ::std::string::String>,
        deny:
            ::std::result::Result<::std::option::Option<super::StringList>, ::std::string::String>,
    }
    impl ::std::default::Default for Permission {
        fn default() -> Self {
            Self {
                allow: Ok(Default::default()),
                deny: Ok(Default::default()),
            }
        }
    }
    impl Permission {
        pub fn allow<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::StringList>>,
            T::Error: ::std::fmt::Display,
        {
            self.allow = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for allow: {}", e));
            self
        }
        pub fn deny<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::StringList>>,
            T::Error: ::std::fmt::Display,
        {
            self.deny = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for deny: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Permission> for super::Permission {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Permission,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                allow: value.allow?,
                deny: value.deny?,
            })
        }
    }
    impl ::std::convert::From<super::Permission> for Permission {
        fn from(value: super::Permission) -> Self {
            Self {
                allow: Ok(value.allow),
                deny: Ok(value.deny),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Permissions {
        pub_:
            ::std::result::Result<::std::option::Option<super::Permission>, ::std::string::String>,
        resp: ::std::result::Result<
            ::std::option::Option<super::ResponsePermission>,
            ::std::string::String,
        >,
        sub: ::std::result::Result<::std::option::Option<super::Permission>, ::std::string::String>,
    }
    impl ::std::default::Default for Permissions {
        fn default() -> Self {
            Self {
                pub_: Ok(Default::default()),
                resp: Ok(Default::default()),
                sub: Ok(Default::default()),
            }
        }
    }
    impl Permissions {
        pub fn pub_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Permission>>,
            T::Error: ::std::fmt::Display,
        {
            self.pub_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pub_: {}", e));
            self
        }
        pub fn resp<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ResponsePermission>>,
            T::Error: ::std::fmt::Display,
        {
            self.resp = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for resp: {}", e));
            self
        }
        pub fn sub<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Permission>>,
            T::Error: ::std::fmt::Display,
        {
            self.sub = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sub: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Permissions> for super::Permissions {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Permissions,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                pub_: value.pub_?,
                resp: value.resp?,
                sub: value.sub?,
            })
        }
    }
    impl ::std::convert::From<super::Permissions> for Permissions {
        fn from(value: super::Permissions) -> Self {
            Self {
                pub_: Ok(value.pub_),
                resp: Ok(value.resp),
                sub: Ok(value.sub),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ResponsePermission {
        max: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        ttl: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
    }
    impl ::std::default::Default for ResponsePermission {
        fn default() -> Self {
            Self {
                max: Ok(Default::default()),
                ttl: Ok(Default::default()),
            }
        }
    }
    impl ResponsePermission {
        pub fn max<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.max = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for max: {}", e));
            self
        }
        pub fn ttl<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.ttl = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ttl: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ResponsePermission> for super::ResponsePermission {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ResponsePermission,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                max: value.max?,
                ttl: value.ttl?,
            })
        }
    }
    impl ::std::convert::From<super::ResponsePermission> for ResponsePermission {
        fn from(value: super::ResponsePermission) -> Self {
            Self {
                max: Ok(value.max),
                ttl: Ok(value.ttl),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ServiceLatency {
        results:
            ::std::result::Result<::std::option::Option<super::Subject>, ::std::string::String>,
        sampling: ::std::result::Result<
            ::std::option::Option<super::SamplingRate>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ServiceLatency {
        fn default() -> Self {
            Self {
                results: Ok(Default::default()),
                sampling: Ok(Default::default()),
            }
        }
    }
    impl ServiceLatency {
        pub fn results<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Subject>>,
            T::Error: ::std::fmt::Display,
        {
            self.results = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for results: {}", e));
            self
        }
        pub fn sampling<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SamplingRate>>,
            T::Error: ::std::fmt::Display,
        {
            self.sampling = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sampling: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ServiceLatency> for super::ServiceLatency {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ServiceLatency,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                results: value.results?,
                sampling: value.sampling?,
            })
        }
    }
    impl ::std::convert::From<super::ServiceLatency> for ServiceLatency {
        fn from(value: super::ServiceLatency) -> Self {
            Self {
                results: Ok(value.results),
                sampling: Ok(value.sampling),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TimeRange {
        end: ::std::result::Result<::std::string::String, ::std::string::String>,
        start: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for TimeRange {
        fn default() -> Self {
            Self {
                end: Err("no value supplied for end".to_string()),
                start: Err("no value supplied for start".to_string()),
            }
        }
    }
    impl TimeRange {
        pub fn end<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.end = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for end: {}", e));
            self
        }
        pub fn start<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.start = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for start: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<TimeRange> for super::TimeRange {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TimeRange,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                end: value.end?,
                start: value.start?,
            })
        }
    }
    impl ::std::convert::From<super::TimeRange> for TimeRange {
        fn from(value: super::TimeRange) -> Self {
            Self {
                end: Ok(value.end),
                start: Ok(value.start),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct User {
        allowed_connection_types:
            ::std::result::Result<::std::vec::Vec<super::ConnectionType>, ::std::string::String>,
        bearer_token: ::std::result::Result<bool, ::std::string::String>,
        data: ::std::result::Result<i64, ::std::string::String>,
        issuer_account: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        payload: ::std::result::Result<i64, ::std::string::String>,
        pub_:
            ::std::result::Result<::std::option::Option<super::Permission>, ::std::string::String>,
        resp: ::std::result::Result<
            ::std::option::Option<super::ResponsePermission>,
            ::std::string::String,
        >,
        src: ::std::result::Result<::std::option::Option<super::CidrList>, ::std::string::String>,
        sub: ::std::result::Result<::std::option::Option<super::Permission>, ::std::string::String>,
        subs: ::std::result::Result<i64, ::std::string::String>,
        tags: ::std::result::Result<::std::option::Option<super::TagList>, ::std::string::String>,
        times: ::std::result::Result<::std::vec::Vec<super::TimeRange>, ::std::string::String>,
        times_location: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<super::UserType, ::std::string::String>,
        version: ::std::result::Result<i64, ::std::string::String>,
    }
    impl ::std::default::Default for User {
        fn default() -> Self {
            Self {
                allowed_connection_types: Ok(Default::default()),
                bearer_token: Ok(super::defaults::default_bool::<true>()),
                data: Ok(super::defaults::default_i64::<i64, -1>()),
                issuer_account: Ok(Default::default()),
                payload: Ok(super::defaults::default_i64::<i64, -1>()),
                pub_: Ok(Default::default()),
                resp: Ok(Default::default()),
                src: Ok(Default::default()),
                sub: Ok(Default::default()),
                subs: Ok(super::defaults::default_i64::<i64, -1>()),
                tags: Ok(Default::default()),
                times: Ok(Default::default()),
                times_location: Ok(Default::default()),
                type_: Ok(super::defaults::user_type()),
                version: Ok(super::defaults::default_u64::<i64, 2>()),
            }
        }
    }
    impl User {
        pub fn allowed_connection_types<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ConnectionType>>,
            T::Error: ::std::fmt::Display,
        {
            self.allowed_connection_types = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for allowed_connection_types: {}",
                    e
                )
            });
            self
        }
        pub fn bearer_token<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.bearer_token = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bearer_token: {}", e));
            self
        }
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {}", e));
            self
        }
        pub fn issuer_account<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.issuer_account = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for issuer_account: {}", e));
            self
        }
        pub fn payload<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.payload = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for payload: {}", e));
            self
        }
        pub fn pub_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Permission>>,
            T::Error: ::std::fmt::Display,
        {
            self.pub_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pub_: {}", e));
            self
        }
        pub fn resp<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ResponsePermission>>,
            T::Error: ::std::fmt::Display,
        {
            self.resp = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for resp: {}", e));
            self
        }
        pub fn src<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CidrList>>,
            T::Error: ::std::fmt::Display,
        {
            self.src = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for src: {}", e));
            self
        }
        pub fn sub<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Permission>>,
            T::Error: ::std::fmt::Display,
        {
            self.sub = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sub: {}", e));
            self
        }
        pub fn subs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.subs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subs: {}", e));
            self
        }
        pub fn tags<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TagList>>,
            T::Error: ::std::fmt::Display,
        {
            self.tags = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tags: {}", e));
            self
        }
        pub fn times<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::TimeRange>>,
            T::Error: ::std::fmt::Display,
        {
            self.times = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for times: {}", e));
            self
        }
        pub fn times_location<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.times_location = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for times_location: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UserType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
        pub fn version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.version = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for version: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<User> for super::User {
        type Error = super::error::ConversionError;
        fn try_from(value: User) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                allowed_connection_types: value.allowed_connection_types?,
                bearer_token: value.bearer_token?,
                data: value.data?,
                issuer_account: value.issuer_account?,
                payload: value.payload?,
                pub_: value.pub_?,
                resp: value.resp?,
                src: value.src?,
                sub: value.sub?,
                subs: value.subs?,
                tags: value.tags?,
                times: value.times?,
                times_location: value.times_location?,
                type_: value.type_?,
                version: value.version?,
            })
        }
    }
    impl ::std::convert::From<super::User> for User {
        fn from(value: super::User) -> Self {
            Self {
                allowed_connection_types: Ok(value.allowed_connection_types),
                bearer_token: Ok(value.bearer_token),
                data: Ok(value.data),
                issuer_account: Ok(value.issuer_account),
                payload: Ok(value.payload),
                pub_: Ok(value.pub_),
                resp: Ok(value.resp),
                src: Ok(value.src),
                sub: Ok(value.sub),
                subs: Ok(value.subs),
                tags: Ok(value.tags),
                times: Ok(value.times),
                times_location: Ok(value.times_location),
                type_: Ok(value.type_),
                version: Ok(value.version),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UserLimits {
        src: ::std::result::Result<::std::option::Option<super::CidrList>, ::std::string::String>,
        times: ::std::result::Result<::std::vec::Vec<super::TimeRange>, ::std::string::String>,
        times_location: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for UserLimits {
        fn default() -> Self {
            Self {
                src: Ok(Default::default()),
                times: Ok(Default::default()),
                times_location: Ok(Default::default()),
            }
        }
    }
    impl UserLimits {
        pub fn src<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CidrList>>,
            T::Error: ::std::fmt::Display,
        {
            self.src = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for src: {}", e));
            self
        }
        pub fn times<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::TimeRange>>,
            T::Error: ::std::fmt::Display,
        {
            self.times = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for times: {}", e));
            self
        }
        pub fn times_location<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.times_location = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for times_location: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<UserLimits> for super::UserLimits {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UserLimits,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                src: value.src?,
                times: value.times?,
                times_location: value.times_location?,
            })
        }
    }
    impl ::std::convert::From<super::UserLimits> for UserLimits {
        fn from(value: super::UserLimits) -> Self {
            Self {
                src: Ok(value.src),
                times: Ok(value.times),
                times_location: Ok(value.times_location),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UserPermissionLimits {
        allowed_connection_types:
            ::std::result::Result<::std::vec::Vec<super::ConnectionType>, ::std::string::String>,
        bearer_token: ::std::result::Result<bool, ::std::string::String>,
        data: ::std::result::Result<i64, ::std::string::String>,
        payload: ::std::result::Result<i64, ::std::string::String>,
        pub_:
            ::std::result::Result<::std::option::Option<super::Permission>, ::std::string::String>,
        resp: ::std::result::Result<
            ::std::option::Option<super::ResponsePermission>,
            ::std::string::String,
        >,
        src: ::std::result::Result<::std::option::Option<super::CidrList>, ::std::string::String>,
        sub: ::std::result::Result<::std::option::Option<super::Permission>, ::std::string::String>,
        subs: ::std::result::Result<i64, ::std::string::String>,
        times: ::std::result::Result<::std::vec::Vec<super::TimeRange>, ::std::string::String>,
        times_location: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for UserPermissionLimits {
        fn default() -> Self {
            Self {
                allowed_connection_types: Ok(Default::default()),
                bearer_token: Ok(super::defaults::default_bool::<true>()),
                data: Ok(super::defaults::default_i64::<i64, -1>()),
                payload: Ok(super::defaults::default_i64::<i64, -1>()),
                pub_: Ok(Default::default()),
                resp: Ok(Default::default()),
                src: Ok(Default::default()),
                sub: Ok(Default::default()),
                subs: Ok(super::defaults::default_i64::<i64, -1>()),
                times: Ok(Default::default()),
                times_location: Ok(Default::default()),
            }
        }
    }
    impl UserPermissionLimits {
        pub fn allowed_connection_types<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ConnectionType>>,
            T::Error: ::std::fmt::Display,
        {
            self.allowed_connection_types = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for allowed_connection_types: {}",
                    e
                )
            });
            self
        }
        pub fn bearer_token<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.bearer_token = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bearer_token: {}", e));
            self
        }
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {}", e));
            self
        }
        pub fn payload<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.payload = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for payload: {}", e));
            self
        }
        pub fn pub_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Permission>>,
            T::Error: ::std::fmt::Display,
        {
            self.pub_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pub_: {}", e));
            self
        }
        pub fn resp<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ResponsePermission>>,
            T::Error: ::std::fmt::Display,
        {
            self.resp = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for resp: {}", e));
            self
        }
        pub fn src<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CidrList>>,
            T::Error: ::std::fmt::Display,
        {
            self.src = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for src: {}", e));
            self
        }
        pub fn sub<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Permission>>,
            T::Error: ::std::fmt::Display,
        {
            self.sub = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sub: {}", e));
            self
        }
        pub fn subs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.subs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subs: {}", e));
            self
        }
        pub fn times<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::TimeRange>>,
            T::Error: ::std::fmt::Display,
        {
            self.times = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for times: {}", e));
            self
        }
        pub fn times_location<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.times_location = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for times_location: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<UserPermissionLimits> for super::UserPermissionLimits {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UserPermissionLimits,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                allowed_connection_types: value.allowed_connection_types?,
                bearer_token: value.bearer_token?,
                data: value.data?,
                payload: value.payload?,
                pub_: value.pub_?,
                resp: value.resp?,
                src: value.src?,
                sub: value.sub?,
                subs: value.subs?,
                times: value.times?,
                times_location: value.times_location?,
            })
        }
    }
    impl ::std::convert::From<super::UserPermissionLimits> for UserPermissionLimits {
        fn from(value: super::UserPermissionLimits) -> Self {
            Self {
                allowed_connection_types: Ok(value.allowed_connection_types),
                bearer_token: Ok(value.bearer_token),
                data: Ok(value.data),
                payload: Ok(value.payload),
                pub_: Ok(value.pub_),
                resp: Ok(value.resp),
                src: Ok(value.src),
                sub: Ok(value.sub),
                subs: Ok(value.subs),
                times: Ok(value.times),
                times_location: Ok(value.times_location),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UserScope {
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        key: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        kind: ::std::result::Result<super::ScopeType, ::std::string::String>,
        role: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        template: ::std::result::Result<
            ::std::option::Option<super::UserPermissionLimits>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for UserScope {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
                key: Ok(Default::default()),
                kind: Ok(super::defaults::user_scope_kind()),
                role: Ok(Default::default()),
                template: Ok(Default::default()),
            }
        }
    }
    impl UserScope {
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn key<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.key = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for key: {}", e));
            self
        }
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ScopeType>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
            self
        }
        pub fn role<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.role = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for role: {}", e));
            self
        }
        pub fn template<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::UserPermissionLimits>>,
            T::Error: ::std::fmt::Display,
        {
            self.template = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for template: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<UserScope> for super::UserScope {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UserScope,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                key: value.key?,
                kind: value.kind?,
                role: value.role?,
                template: value.template?,
            })
        }
    }
    impl ::std::convert::From<super::UserScope> for UserScope {
        fn from(value: super::UserScope) -> Self {
            Self {
                description: Ok(value.description),
                key: Ok(value.key),
                kind: Ok(value.kind),
                role: Ok(value.role),
                template: Ok(value.template),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct WeightedMapping {
        cluster: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        subject:
            ::std::result::Result<::std::option::Option<super::Subject>, ::std::string::String>,
        weight: ::std::result::Result<u64, ::std::string::String>,
    }
    impl ::std::default::Default for WeightedMapping {
        fn default() -> Self {
            Self {
                cluster: Ok(Default::default()),
                subject: Ok(Default::default()),
                weight: Ok(super::defaults::default_u64::<u64, 100>()),
            }
        }
    }
    impl WeightedMapping {
        pub fn cluster<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.cluster = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cluster: {}", e));
            self
        }
        pub fn subject<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Subject>>,
            T::Error: ::std::fmt::Display,
        {
            self.subject = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subject: {}", e));
            self
        }
        pub fn weight<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<u64>,
            T::Error: ::std::fmt::Display,
        {
            self.weight = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for weight: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<WeightedMapping> for super::WeightedMapping {
        type Error = super::error::ConversionError;
        fn try_from(
            value: WeightedMapping,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                cluster: value.cluster?,
                subject: value.subject?,
                weight: value.weight?,
            })
        }
    }
    impl ::std::convert::From<super::WeightedMapping> for WeightedMapping {
        fn from(value: super::WeightedMapping) -> Self {
            Self {
                cluster: Ok(value.cluster),
                subject: Ok(value.subject),
                weight: Ok(value.weight),
            }
        }
    }
}
/// Generation of default values for serde.
pub mod defaults {
    pub(super) fn default_bool<const V: bool>() -> bool {
        V
    }
    pub(super) fn default_i64<T, const V: i64>() -> T
    where
        T: std::convert::TryFrom<i64>,
        <T as std::convert::TryFrom<i64>>::Error: std::fmt::Debug,
    {
        T::try_from(V).unwrap()
    }
    pub(super) fn default_u64<T, const V: u64>() -> T
    where
        T: std::convert::TryFrom<u64>,
        <T as std::convert::TryFrom<u64>>::Error: std::fmt::Debug,
    {
        T::try_from(V).unwrap()
    }
    pub(super) fn account_type() -> super::AccountType {
        super::AccountType::Account
    }
    pub(super) fn activation_type() -> super::ActivationType {
        super::ActivationType::Activation
    }
    pub(super) fn user_type() -> super::UserType {
        super::UserType::User
    }
    pub(super) fn user_scope_kind() -> super::ScopeType {
        super::ScopeType::UserScope
    }
}
