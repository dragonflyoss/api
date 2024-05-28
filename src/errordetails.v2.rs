/// Backend is error detail for Backend.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Backend {
    /// Backend error message.
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
    /// Backend HTTP response header.
    #[prost(map = "string, string", tag = "2")]
    pub header: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Backend HTTP status code.
    #[prost(int32, tag = "3")]
    pub status_code: i32,
}
