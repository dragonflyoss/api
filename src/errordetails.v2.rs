/// HTTP is error detail for HTTP protocol.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Http {
    /// HTTP Response Header.
    #[prost(map = "string, string", tag = "1")]
    pub header: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// HTTP Status Code.
    #[prost(int32, tag = "2")]
    pub status_code: i32,
}
