/// DownloadPeerBackToSourceFailed is error detail of downloading peer back-to-source.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DownloadPeerBackToSourceFailed {
    /// The description of the error.
    #[prost(string, tag = "1")]
    pub description: ::prost::alloc::string::String,
}
/// DownloadPieceBackToSourceFailed is error detail of downloading piece back-to-source.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DownloadPieceBackToSourceFailed {
    /// Temporary recoverable error of source.
    #[prost(bool, tag = "1")]
    pub temporary: bool,
    /// Source response metadata, eg: HTTP Status Code, HTTP Status, HTTP Header
    #[prost(message, optional, tag = "2")]
    pub metadata: ::core::option::Option<super::common::ExtendAttribute>,
    /// The number of piece.
    #[prost(uint32, tag = "3")]
    pub piece_number: u32,
    /// The description of the error.
    #[prost(string, tag = "4")]
    pub description: ::prost::alloc::string::String,
}
/// DownloadPieceFailed is error detail of downloading piece.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DownloadPieceFailed {
    /// Temporary recoverable error of parent peer.
    #[prost(bool, tag = "1")]
    pub temporary: bool,
    /// Source response metadata, eg: HTTP Status Code, HTTP Status, HTTP Header
    #[prost(message, optional, tag = "2")]
    pub metadata: ::core::option::Option<super::common::ExtendAttribute>,
    /// Piece is information of piece.
    #[prost(string, tag = "3")]
    pub parent_id: ::prost::alloc::string::String,
    /// The number of piece.
    #[prost(uint32, tag = "4")]
    pub piece_number: u32,
    /// The description of the error.
    #[prost(string, tag = "5")]
    pub description: ::prost::alloc::string::String,
}
/// SchedulePeerForbidden is error detail of forbidden.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SchedulePeerForbidden {
    /// The description of the error.
    #[prost(string, tag = "1")]
    pub description: ::prost::alloc::string::String,
}
/// SchedulePeerFailed is error detail of scheduling.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SchedulePeerFailed {
    /// The description of the error.
    #[prost(string, tag = "1")]
    pub description: ::prost::alloc::string::String,
}
/// SyncPiecesFailed is error detail of syncing pieces.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SyncPiecesFailed {
    /// Temporary recoverable error of parent peer.
    #[prost(bool, tag = "1")]
    pub temporary: bool,
    /// Parent peer id.
    #[prost(string, tag = "2")]
    pub parent_id: ::prost::alloc::string::String,
    /// The description of the error.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
}
/// StatMetadataFailed is error detail of stat metadata.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatMetadataFailed {
    /// The description of the error.
    #[prost(string, tag = "1")]
    pub description: ::prost::alloc::string::String,
}
