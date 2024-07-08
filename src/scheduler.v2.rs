/// RegisterPeerRequest represents peer registered request of AnnouncePeerRequest.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterPeerRequest {
    /// Download information.
    #[prost(message, optional, tag = "1")]
    pub download: ::core::option::Option<super::super::common::v2::Download>,
}
/// DownloadPeerStartedRequest represents peer download started request of AnnouncePeerRequest.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DownloadPeerStartedRequest {}
/// DownloadPeerBackToSourceStartedRequest represents peer download back-to-source started request of AnnouncePeerRequest.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DownloadPeerBackToSourceStartedRequest {
    /// The description of the back-to-source reason.
    #[prost(string, optional, tag = "1")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
}
/// ReschedulePeerRequest represents reschedule request of AnnouncePeerRequest.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReschedulePeerRequest {
    /// Candidate parent ids.
    #[prost(message, repeated, tag = "1")]
    pub candidate_parents: ::prost::alloc::vec::Vec<super::super::common::v2::Peer>,
    /// The description of the reschedule reason.
    #[prost(string, optional, tag = "2")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
}
/// DownloadPeerFinishedRequest represents peer download finished request of AnnouncePeerRequest.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DownloadPeerFinishedRequest {
    /// Total content length.
    #[prost(uint64, tag = "1")]
    pub content_length: u64,
    /// Total piece count.
    #[prost(uint32, tag = "2")]
    pub piece_count: u32,
}
/// DownloadPeerBackToSourceFinishedRequest represents peer download back-to-source finished request of AnnouncePeerRequest.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DownloadPeerBackToSourceFinishedRequest {
    /// Total content length.
    #[prost(uint64, tag = "1")]
    pub content_length: u64,
    /// Total piece count.
    #[prost(uint32, tag = "2")]
    pub piece_count: u32,
}
/// DownloadPeerFailedRequest represents peer download failed request of AnnouncePeerRequest.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DownloadPeerFailedRequest {
    /// The description of the download failed.
    #[prost(string, optional, tag = "1")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
}
/// DownloadPeerBackToSourceFailedRequest represents peer download back-to-source failed request of AnnouncePeerRequest.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DownloadPeerBackToSourceFailedRequest {
    /// The description of the download back-to-source failed.
    #[prost(string, optional, tag = "1")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
}
/// DownloadPieceFinishedRequest represents piece download finished request of AnnouncePeerRequest.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DownloadPieceFinishedRequest {
    /// Piece info.
    #[prost(message, optional, tag = "1")]
    pub piece: ::core::option::Option<super::super::common::v2::Piece>,
}
/// DownloadPieceBackToSourceFinishedRequest represents piece download back-to-source finished request of AnnouncePeerRequest.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DownloadPieceBackToSourceFinishedRequest {
    /// Piece info.
    #[prost(message, optional, tag = "1")]
    pub piece: ::core::option::Option<super::super::common::v2::Piece>,
}
/// DownloadPieceFailedRequest downloads piece failed request of AnnouncePeerRequest.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DownloadPieceFailedRequest {
    /// Piece number.
    #[prost(uint32, optional, tag = "1")]
    pub piece_number: ::core::option::Option<u32>,
    /// Parent id.
    #[prost(string, tag = "2")]
    pub parent_id: ::prost::alloc::string::String,
    /// Temporary indicates whether the error is temporary.
    #[prost(bool, tag = "3")]
    pub temporary: bool,
}
/// DownloadPieceBackToSourceFailedRequest downloads piece back-to-source failed request of AnnouncePeerRequest.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DownloadPieceBackToSourceFailedRequest {
    /// Piece number.
    #[prost(uint32, optional, tag = "1")]
    pub piece_number: ::core::option::Option<u32>,
    #[prost(
        oneof = "download_piece_back_to_source_failed_request::Response",
        tags = "2"
    )]
    pub response: ::core::option::Option<
        download_piece_back_to_source_failed_request::Response,
    >,
}
/// Nested message and enum types in `DownloadPieceBackToSourceFailedRequest`.
pub mod download_piece_back_to_source_failed_request {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag = "2")]
        Backend(super::super::super::errordetails::v2::Backend),
    }
}
/// AnnouncePeerRequest represents request of AnnouncePeer.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnouncePeerRequest {
    /// Host id.
    #[prost(string, tag = "1")]
    pub host_id: ::prost::alloc::string::String,
    /// Task id.
    #[prost(string, tag = "2")]
    pub task_id: ::prost::alloc::string::String,
    /// Peer id.
    #[prost(string, tag = "3")]
    pub peer_id: ::prost::alloc::string::String,
    #[prost(
        oneof = "announce_peer_request::Request",
        tags = "4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15"
    )]
    pub request: ::core::option::Option<announce_peer_request::Request>,
}
/// Nested message and enum types in `AnnouncePeerRequest`.
pub mod announce_peer_request {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Request {
        #[prost(message, tag = "4")]
        RegisterPeerRequest(super::RegisterPeerRequest),
        #[prost(message, tag = "5")]
        DownloadPeerStartedRequest(super::DownloadPeerStartedRequest),
        #[prost(message, tag = "6")]
        DownloadPeerBackToSourceStartedRequest(
            super::DownloadPeerBackToSourceStartedRequest,
        ),
        #[prost(message, tag = "7")]
        ReschedulePeerRequest(super::ReschedulePeerRequest),
        #[prost(message, tag = "8")]
        DownloadPeerFinishedRequest(super::DownloadPeerFinishedRequest),
        #[prost(message, tag = "9")]
        DownloadPeerBackToSourceFinishedRequest(
            super::DownloadPeerBackToSourceFinishedRequest,
        ),
        #[prost(message, tag = "10")]
        DownloadPeerFailedRequest(super::DownloadPeerFailedRequest),
        #[prost(message, tag = "11")]
        DownloadPeerBackToSourceFailedRequest(
            super::DownloadPeerBackToSourceFailedRequest,
        ),
        #[prost(message, tag = "12")]
        DownloadPieceFinishedRequest(super::DownloadPieceFinishedRequest),
        #[prost(message, tag = "13")]
        DownloadPieceBackToSourceFinishedRequest(
            super::DownloadPieceBackToSourceFinishedRequest,
        ),
        #[prost(message, tag = "14")]
        DownloadPieceFailedRequest(super::DownloadPieceFailedRequest),
        #[prost(message, tag = "15")]
        DownloadPieceBackToSourceFailedRequest(
            super::DownloadPieceBackToSourceFailedRequest,
        ),
    }
}
/// EmptyTaskResponse represents empty task response of AnnouncePeerResponse.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmptyTaskResponse {}
/// NormalTaskResponse represents normal task response of AnnouncePeerResponse.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NormalTaskResponse {
    /// Candidate parents.
    #[prost(message, repeated, tag = "1")]
    pub candidate_parents: ::prost::alloc::vec::Vec<super::super::common::v2::Peer>,
}
/// NeedBackToSourceResponse represents need back-to-source response of AnnouncePeerResponse.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NeedBackToSourceResponse {
    /// The description of the back-to-source reason.
    #[prost(string, optional, tag = "1")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
}
/// AnnouncePeerResponse represents response of AnnouncePeer.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnouncePeerResponse {
    #[prost(oneof = "announce_peer_response::Response", tags = "1, 2, 3")]
    pub response: ::core::option::Option<announce_peer_response::Response>,
}
/// Nested message and enum types in `AnnouncePeerResponse`.
pub mod announce_peer_response {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag = "1")]
        EmptyTaskResponse(super::EmptyTaskResponse),
        #[prost(message, tag = "2")]
        NormalTaskResponse(super::NormalTaskResponse),
        #[prost(message, tag = "3")]
        NeedBackToSourceResponse(super::NeedBackToSourceResponse),
    }
}
/// StatPeerRequest represents request of StatPeer.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatPeerRequest {
    /// Host id.
    #[prost(string, tag = "1")]
    pub host_id: ::prost::alloc::string::String,
    /// Task id.
    #[prost(string, tag = "2")]
    pub task_id: ::prost::alloc::string::String,
    /// Peer id.
    #[prost(string, tag = "3")]
    pub peer_id: ::prost::alloc::string::String,
}
/// DeletePeerRequest represents request of DeletePeer.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePeerRequest {
    /// Host id.
    #[prost(string, tag = "1")]
    pub host_id: ::prost::alloc::string::String,
    /// Task id.
    #[prost(string, tag = "2")]
    pub task_id: ::prost::alloc::string::String,
    /// Peer id.
    #[prost(string, tag = "3")]
    pub peer_id: ::prost::alloc::string::String,
}
/// StatTaskRequest represents request of StatTask.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatTaskRequest {
    /// Task id.
    #[prost(string, tag = "1")]
    pub task_id: ::prost::alloc::string::String,
}
/// DeleteTaskRequest represents request of DeleteTask.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTaskRequest {
    /// Host id.
    #[prost(string, tag = "1")]
    pub host_id: ::prost::alloc::string::String,
    /// Task id.
    #[prost(string, tag = "2")]
    pub task_id: ::prost::alloc::string::String,
}
/// AnnounceHostRequest represents request of AnnounceHost.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnounceHostRequest {
    /// Host information.
    #[prost(message, optional, tag = "1")]
    pub host: ::core::option::Option<super::super::common::v2::Host>,
    /// The interval between dfdaemon announces to scheduler.
    #[prost(message, optional, tag = "2")]
    pub interval: ::core::option::Option<::prost_wkt_types::Duration>,
}
/// DeleteHostRequest represents request of DeleteHost.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteHostRequest {
    /// Host id.
    #[prost(string, tag = "1")]
    pub host_id: ::prost::alloc::string::String,
}
/// ProbeStartedRequest represents started request of SyncProbesRequest.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProbeStartedRequest {}
/// Probe information.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Probe {
    /// Destination host metadata.
    #[prost(message, optional, tag = "1")]
    pub host: ::core::option::Option<super::super::common::v2::Host>,
    /// RTT is the round-trip time sent via this pinger.
    #[prost(message, optional, tag = "2")]
    pub rtt: ::core::option::Option<::prost_wkt_types::Duration>,
    /// Probe create time.
    #[prost(message, optional, tag = "3")]
    pub created_at: ::core::option::Option<::prost_wkt_types::Timestamp>,
}
/// ProbeFinishedRequest represents finished request of SyncProbesRequest.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProbeFinishedRequest {
    /// Probes information.
    #[prost(message, repeated, tag = "1")]
    pub probes: ::prost::alloc::vec::Vec<Probe>,
}
/// FailedProbe information.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FailedProbe {
    /// Destination host metadata.
    #[prost(message, optional, tag = "1")]
    pub host: ::core::option::Option<super::super::common::v2::Host>,
    /// The description of probing failed.
    #[prost(string, optional, tag = "2")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
}
/// ProbeFailedRequest represents failed request of SyncProbesRequest.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProbeFailedRequest {
    /// Failed probes information.
    #[prost(message, repeated, tag = "1")]
    pub probes: ::prost::alloc::vec::Vec<FailedProbe>,
}
/// SyncProbesRequest represents request of SyncProbes.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SyncProbesRequest {
    /// Source host metadata.
    #[prost(message, optional, tag = "1")]
    pub host: ::core::option::Option<super::super::common::v2::Host>,
    #[prost(oneof = "sync_probes_request::Request", tags = "2, 3, 4")]
    pub request: ::core::option::Option<sync_probes_request::Request>,
}
/// Nested message and enum types in `SyncProbesRequest`.
pub mod sync_probes_request {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Request {
        #[prost(message, tag = "2")]
        ProbeStartedRequest(super::ProbeStartedRequest),
        #[prost(message, tag = "3")]
        ProbeFinishedRequest(super::ProbeFinishedRequest),
        #[prost(message, tag = "4")]
        ProbeFailedRequest(super::ProbeFailedRequest),
    }
}
/// SyncProbesResponse represents response of SyncProbes.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SyncProbesResponse {
    /// Hosts needs to be probed.
    #[prost(message, repeated, tag = "1")]
    pub hosts: ::prost::alloc::vec::Vec<super::super::common::v2::Host>,
}
/// RegisterCachePeerRequest represents cache peer registered request of AnnounceCachePeerRequest.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterCachePeerRequest {
    /// Host id.
    #[prost(string, tag = "1")]
    pub host_id: ::prost::alloc::string::String,
    /// Task id.
    #[prost(string, tag = "2")]
    pub task_id: ::prost::alloc::string::String,
    /// Tag is used to distinguish different cache tasks.
    #[prost(string, optional, tag = "3")]
    pub tag: ::core::option::Option<::prost::alloc::string::String>,
    /// Application of task.
    #[prost(string, optional, tag = "4")]
    pub application: ::core::option::Option<::prost::alloc::string::String>,
    /// Task piece length.
    #[prost(uint64, tag = "5")]
    pub piece_length: u64,
    /// File path to be exported.
    #[prost(string, tag = "6")]
    pub output_path: ::prost::alloc::string::String,
    /// Download timeout.
    #[prost(message, optional, tag = "7")]
    pub timeout: ::core::option::Option<::prost_wkt_types::Duration>,
}
/// DownloadCachePeerStartedRequest represents cache peer download started request of AnnounceCachePeerRequest.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DownloadCachePeerStartedRequest {}
/// RescheduleCachePeerRequest represents reschedule request of AnnounceCachePeerRequest.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RescheduleCachePeerRequest {
    /// Candidate parent ids.
    #[prost(message, repeated, tag = "1")]
    pub candidate_parents: ::prost::alloc::vec::Vec<super::super::common::v2::CachePeer>,
    /// The description of the reschedule reason.
    #[prost(string, optional, tag = "2")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
}
/// DownloadCachePeerFinishedRequest represents cache peer download finished request of AnnounceCachePeerRequest.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DownloadCachePeerFinishedRequest {
    /// Total piece count.
    #[prost(uint32, tag = "1")]
    pub piece_count: u32,
}
/// DownloadCachePeerFailedRequest represents cache peer download failed request of AnnounceCachePeerRequest.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DownloadCachePeerFailedRequest {
    /// The description of the download failed.
    #[prost(string, optional, tag = "1")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
}
/// AnnounceCachePeerRequest represents request of AnnounceCachePeer.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnounceCachePeerRequest {
    /// Host id.
    #[prost(string, tag = "1")]
    pub host_id: ::prost::alloc::string::String,
    /// Task id.
    #[prost(string, tag = "2")]
    pub task_id: ::prost::alloc::string::String,
    /// Peer id.
    #[prost(string, tag = "3")]
    pub peer_id: ::prost::alloc::string::String,
    #[prost(
        oneof = "announce_cache_peer_request::Request",
        tags = "4, 5, 6, 7, 8, 9, 10"
    )]
    pub request: ::core::option::Option<announce_cache_peer_request::Request>,
}
/// Nested message and enum types in `AnnounceCachePeerRequest`.
pub mod announce_cache_peer_request {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Request {
        #[prost(message, tag = "4")]
        RegisterCachePeerRequest(super::RegisterCachePeerRequest),
        #[prost(message, tag = "5")]
        DownloadCachePeerStartedRequest(super::DownloadCachePeerStartedRequest),
        #[prost(message, tag = "6")]
        RescheduleCachePeerRequest(super::RescheduleCachePeerRequest),
        #[prost(message, tag = "7")]
        DownloadCachePeerFinishedRequest(super::DownloadCachePeerFinishedRequest),
        #[prost(message, tag = "8")]
        DownloadCachePeerFailedRequest(super::DownloadCachePeerFailedRequest),
        #[prost(message, tag = "9")]
        DownloadPieceFinishedRequest(super::DownloadPieceFinishedRequest),
        #[prost(message, tag = "10")]
        DownloadPieceFailedRequest(super::DownloadPieceFailedRequest),
    }
}
/// EmptyCacheTaskResponse represents empty cache task response of AnnounceCachePeerResponse.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmptyCacheTaskResponse {}
/// NormalCacheTaskResponse represents normal cache task response of AnnounceCachePeerResponse.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NormalCacheTaskResponse {
    /// Candidate parents.
    #[prost(message, repeated, tag = "1")]
    pub candidate_cache_parents: ::prost::alloc::vec::Vec<
        super::super::common::v2::CachePeer,
    >,
}
/// AnnounceCachePeerResponse represents response of AnnounceCachePeer.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnounceCachePeerResponse {
    #[prost(oneof = "announce_cache_peer_response::Response", tags = "1, 2")]
    pub response: ::core::option::Option<announce_cache_peer_response::Response>,
}
/// Nested message and enum types in `AnnounceCachePeerResponse`.
pub mod announce_cache_peer_response {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag = "1")]
        EmptyTaskResponse(super::EmptyCacheTaskResponse),
        #[prost(message, tag = "2")]
        NormalTaskResponse(super::NormalCacheTaskResponse),
    }
}
/// StatCachePeerRequest represents request of StatCachePeer.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatCachePeerRequest {
    /// Host id.
    #[prost(string, tag = "1")]
    pub host_id: ::prost::alloc::string::String,
    /// Task id.
    #[prost(string, tag = "2")]
    pub task_id: ::prost::alloc::string::String,
    /// Peer id.
    #[prost(string, tag = "3")]
    pub peer_id: ::prost::alloc::string::String,
}
/// DeleteCachePeerRequest represents request of DeleteCachePeer.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteCachePeerRequest {
    /// Host id.
    #[prost(string, tag = "1")]
    pub host_id: ::prost::alloc::string::String,
    /// Task id.
    #[prost(string, tag = "2")]
    pub task_id: ::prost::alloc::string::String,
    /// Peer id.
    #[prost(string, tag = "3")]
    pub peer_id: ::prost::alloc::string::String,
}
/// UploadCacheTaskStartedRequest represents upload cache task started request of UploadCacheTaskStarted.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadCacheTaskStartedRequest {
    /// Host id.
    #[prost(string, tag = "1")]
    pub host_id: ::prost::alloc::string::String,
    /// Task id.
    #[prost(string, tag = "2")]
    pub task_id: ::prost::alloc::string::String,
    /// Peer id.
    #[prost(string, tag = "3")]
    pub peer_id: ::prost::alloc::string::String,
    /// Replica count of the persistent cache task.
    #[prost(uint64, tag = "4")]
    pub persistent_replica_count: u64,
    /// Tag is used to distinguish different cache tasks.
    #[prost(string, optional, tag = "5")]
    pub tag: ::core::option::Option<::prost::alloc::string::String>,
    /// Application of task.
    #[prost(string, optional, tag = "6")]
    pub application: ::core::option::Option<::prost::alloc::string::String>,
    /// Task piece length.
    #[prost(uint64, tag = "7")]
    pub piece_length: u64,
    /// TTL of the cache task.
    #[prost(message, optional, tag = "8")]
    pub ttl: ::core::option::Option<::prost_wkt_types::Duration>,
    /// Upload timeout.
    #[prost(message, optional, tag = "9")]
    pub timeout: ::core::option::Option<::prost_wkt_types::Duration>,
}
/// UploadCacheTaskFinishedRequest represents upload cache task finished request of UploadCacheTaskFinished.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadCacheTaskFinishedRequest {
    /// Host id.
    #[prost(string, tag = "1")]
    pub host_id: ::prost::alloc::string::String,
    /// Task id.
    #[prost(string, tag = "2")]
    pub task_id: ::prost::alloc::string::String,
    /// Peer id.
    #[prost(string, tag = "3")]
    pub peer_id: ::prost::alloc::string::String,
}
/// UploadCacheTaskFailedRequest represents upload cache task failed request of UploadCacheTaskFailed.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadCacheTaskFailedRequest {
    /// Host id.
    #[prost(string, tag = "1")]
    pub host_id: ::prost::alloc::string::String,
    /// Task id.
    #[prost(string, tag = "2")]
    pub task_id: ::prost::alloc::string::String,
    /// Peer id.
    #[prost(string, tag = "3")]
    pub peer_id: ::prost::alloc::string::String,
    /// The description of the upload failed.
    #[prost(string, optional, tag = "4")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
}
/// StatCacheTaskRequest represents request of StatCacheTask.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatCacheTaskRequest {
    /// Task id.
    #[prost(string, tag = "1")]
    pub task_id: ::prost::alloc::string::String,
}
/// DeleteCacheTaskRequest represents request of DeleteCacheTask.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteCacheTaskRequest {
    /// Host id.
    #[prost(string, tag = "1")]
    pub host_id: ::prost::alloc::string::String,
    /// Task id.
    #[prost(string, tag = "2")]
    pub task_id: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod scheduler_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Scheduler RPC Service.
    #[derive(Debug, Clone)]
    pub struct SchedulerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SchedulerClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> SchedulerClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> SchedulerClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            SchedulerClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// AnnouncePeer announces peer to scheduler.
        pub async fn announce_peer(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::AnnouncePeerRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::AnnouncePeerResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/scheduler.v2.Scheduler/AnnouncePeer",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("scheduler.v2.Scheduler", "AnnouncePeer"));
            self.inner.streaming(req, path, codec).await
        }
        /// Checks information of peer.
        pub async fn stat_peer(
            &mut self,
            request: impl tonic::IntoRequest<super::StatPeerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v2::Peer>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/scheduler.v2.Scheduler/StatPeer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("scheduler.v2.Scheduler", "StatPeer"));
            self.inner.unary(req, path, codec).await
        }
        /// DeletePeer releases peer in scheduler.
        pub async fn delete_peer(
            &mut self,
            request: impl tonic::IntoRequest<super::DeletePeerRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/scheduler.v2.Scheduler/DeletePeer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("scheduler.v2.Scheduler", "DeletePeer"));
            self.inner.unary(req, path, codec).await
        }
        /// Checks information of task.
        pub async fn stat_task(
            &mut self,
            request: impl tonic::IntoRequest<super::StatTaskRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v2::Task>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/scheduler.v2.Scheduler/StatTask",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("scheduler.v2.Scheduler", "StatTask"));
            self.inner.unary(req, path, codec).await
        }
        /// DeleteTask releases task in scheduler.
        pub async fn delete_task(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteTaskRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/scheduler.v2.Scheduler/DeleteTask",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("scheduler.v2.Scheduler", "DeleteTask"));
            self.inner.unary(req, path, codec).await
        }
        /// AnnounceHost announces host to scheduler.
        pub async fn announce_host(
            &mut self,
            request: impl tonic::IntoRequest<super::AnnounceHostRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/scheduler.v2.Scheduler/AnnounceHost",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("scheduler.v2.Scheduler", "AnnounceHost"));
            self.inner.unary(req, path, codec).await
        }
        /// DeleteHost releases host in scheduler.
        pub async fn delete_host(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteHostRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/scheduler.v2.Scheduler/DeleteHost",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("scheduler.v2.Scheduler", "DeleteHost"));
            self.inner.unary(req, path, codec).await
        }
        /// SyncProbes sync probes of the host.
        pub async fn sync_probes(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::SyncProbesRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::SyncProbesResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/scheduler.v2.Scheduler/SyncProbes",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("scheduler.v2.Scheduler", "SyncProbes"));
            self.inner.streaming(req, path, codec).await
        }
        /// AnnounceCachePeer announces cache peer to scheduler.
        pub async fn announce_cache_peer(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::AnnounceCachePeerRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::AnnounceCachePeerResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/scheduler.v2.Scheduler/AnnounceCachePeer",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("scheduler.v2.Scheduler", "AnnounceCachePeer"));
            self.inner.streaming(req, path, codec).await
        }
        /// Checks information of cache peer.
        pub async fn stat_cache_peer(
            &mut self,
            request: impl tonic::IntoRequest<super::StatCachePeerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v2::CachePeer>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/scheduler.v2.Scheduler/StatCachePeer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("scheduler.v2.Scheduler", "StatCachePeer"));
            self.inner.unary(req, path, codec).await
        }
        /// DeleteCachePeer releases cache peer in scheduler.
        pub async fn delete_cache_peer(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteCachePeerRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/scheduler.v2.Scheduler/DeleteCachePeer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("scheduler.v2.Scheduler", "DeleteCachePeer"));
            self.inner.unary(req, path, codec).await
        }
        /// UploadCacheTaskStarted uploads cache task started to scheduler.
        pub async fn upload_cache_task_started(
            &mut self,
            request: impl tonic::IntoRequest<super::UploadCacheTaskStartedRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/scheduler.v2.Scheduler/UploadCacheTaskStarted",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("scheduler.v2.Scheduler", "UploadCacheTaskStarted"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// UploadCacheTaskFinished uploads cache task finished to scheduler.
        pub async fn upload_cache_task_finished(
            &mut self,
            request: impl tonic::IntoRequest<super::UploadCacheTaskFinishedRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v2::CacheTask>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/scheduler.v2.Scheduler/UploadCacheTaskFinished",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("scheduler.v2.Scheduler", "UploadCacheTaskFinished"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// UploadCacheTaskFailed uploads cache task failed to scheduler.
        pub async fn upload_cache_task_failed(
            &mut self,
            request: impl tonic::IntoRequest<super::UploadCacheTaskFailedRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/scheduler.v2.Scheduler/UploadCacheTaskFailed",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("scheduler.v2.Scheduler", "UploadCacheTaskFailed"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Checks information of cache task.
        pub async fn stat_cache_task(
            &mut self,
            request: impl tonic::IntoRequest<super::StatCacheTaskRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v2::CacheTask>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/scheduler.v2.Scheduler/StatCacheTask",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("scheduler.v2.Scheduler", "StatCacheTask"));
            self.inner.unary(req, path, codec).await
        }
        /// DeleteCacheTask releases cache task in scheduler.
        pub async fn delete_cache_task(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteCacheTaskRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/scheduler.v2.Scheduler/DeleteCacheTask",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("scheduler.v2.Scheduler", "DeleteCacheTask"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod scheduler_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with SchedulerServer.
    #[async_trait]
    pub trait Scheduler: Send + Sync + 'static {
        /// Server streaming response type for the AnnouncePeer method.
        type AnnouncePeerStream: futures_core::Stream<
                Item = std::result::Result<super::AnnouncePeerResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// AnnouncePeer announces peer to scheduler.
        async fn announce_peer(
            &self,
            request: tonic::Request<tonic::Streaming<super::AnnouncePeerRequest>>,
        ) -> std::result::Result<
            tonic::Response<Self::AnnouncePeerStream>,
            tonic::Status,
        >;
        /// Checks information of peer.
        async fn stat_peer(
            &self,
            request: tonic::Request<super::StatPeerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v2::Peer>,
            tonic::Status,
        >;
        /// DeletePeer releases peer in scheduler.
        async fn delete_peer(
            &self,
            request: tonic::Request<super::DeletePeerRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /// Checks information of task.
        async fn stat_task(
            &self,
            request: tonic::Request<super::StatTaskRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v2::Task>,
            tonic::Status,
        >;
        /// DeleteTask releases task in scheduler.
        async fn delete_task(
            &self,
            request: tonic::Request<super::DeleteTaskRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /// AnnounceHost announces host to scheduler.
        async fn announce_host(
            &self,
            request: tonic::Request<super::AnnounceHostRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /// DeleteHost releases host in scheduler.
        async fn delete_host(
            &self,
            request: tonic::Request<super::DeleteHostRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /// Server streaming response type for the SyncProbes method.
        type SyncProbesStream: futures_core::Stream<
                Item = std::result::Result<super::SyncProbesResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// SyncProbes sync probes of the host.
        async fn sync_probes(
            &self,
            request: tonic::Request<tonic::Streaming<super::SyncProbesRequest>>,
        ) -> std::result::Result<tonic::Response<Self::SyncProbesStream>, tonic::Status>;
        /// Server streaming response type for the AnnounceCachePeer method.
        type AnnounceCachePeerStream: futures_core::Stream<
                Item = std::result::Result<
                    super::AnnounceCachePeerResponse,
                    tonic::Status,
                >,
            >
            + Send
            + 'static;
        /// AnnounceCachePeer announces cache peer to scheduler.
        async fn announce_cache_peer(
            &self,
            request: tonic::Request<tonic::Streaming<super::AnnounceCachePeerRequest>>,
        ) -> std::result::Result<
            tonic::Response<Self::AnnounceCachePeerStream>,
            tonic::Status,
        >;
        /// Checks information of cache peer.
        async fn stat_cache_peer(
            &self,
            request: tonic::Request<super::StatCachePeerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v2::CachePeer>,
            tonic::Status,
        >;
        /// DeleteCachePeer releases cache peer in scheduler.
        async fn delete_cache_peer(
            &self,
            request: tonic::Request<super::DeleteCachePeerRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /// UploadCacheTaskStarted uploads cache task started to scheduler.
        async fn upload_cache_task_started(
            &self,
            request: tonic::Request<super::UploadCacheTaskStartedRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /// UploadCacheTaskFinished uploads cache task finished to scheduler.
        async fn upload_cache_task_finished(
            &self,
            request: tonic::Request<super::UploadCacheTaskFinishedRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v2::CacheTask>,
            tonic::Status,
        >;
        /// UploadCacheTaskFailed uploads cache task failed to scheduler.
        async fn upload_cache_task_failed(
            &self,
            request: tonic::Request<super::UploadCacheTaskFailedRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /// Checks information of cache task.
        async fn stat_cache_task(
            &self,
            request: tonic::Request<super::StatCacheTaskRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v2::CacheTask>,
            tonic::Status,
        >;
        /// DeleteCacheTask releases cache task in scheduler.
        async fn delete_cache_task(
            &self,
            request: tonic::Request<super::DeleteCacheTaskRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
    }
    /// Scheduler RPC Service.
    #[derive(Debug)]
    pub struct SchedulerServer<T: Scheduler> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Scheduler> SchedulerServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for SchedulerServer<T>
    where
        T: Scheduler,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/scheduler.v2.Scheduler/AnnouncePeer" => {
                    #[allow(non_camel_case_types)]
                    struct AnnouncePeerSvc<T: Scheduler>(pub Arc<T>);
                    impl<
                        T: Scheduler,
                    > tonic::server::StreamingService<super::AnnouncePeerRequest>
                    for AnnouncePeerSvc<T> {
                        type Response = super::AnnouncePeerResponse;
                        type ResponseStream = T::AnnouncePeerStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::AnnouncePeerRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).announce_peer(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AnnouncePeerSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/scheduler.v2.Scheduler/StatPeer" => {
                    #[allow(non_camel_case_types)]
                    struct StatPeerSvc<T: Scheduler>(pub Arc<T>);
                    impl<
                        T: Scheduler,
                    > tonic::server::UnaryService<super::StatPeerRequest>
                    for StatPeerSvc<T> {
                        type Response = super::super::super::common::v2::Peer;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StatPeerRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).stat_peer(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StatPeerSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/scheduler.v2.Scheduler/DeletePeer" => {
                    #[allow(non_camel_case_types)]
                    struct DeletePeerSvc<T: Scheduler>(pub Arc<T>);
                    impl<
                        T: Scheduler,
                    > tonic::server::UnaryService<super::DeletePeerRequest>
                    for DeletePeerSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeletePeerRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).delete_peer(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeletePeerSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/scheduler.v2.Scheduler/StatTask" => {
                    #[allow(non_camel_case_types)]
                    struct StatTaskSvc<T: Scheduler>(pub Arc<T>);
                    impl<
                        T: Scheduler,
                    > tonic::server::UnaryService<super::StatTaskRequest>
                    for StatTaskSvc<T> {
                        type Response = super::super::super::common::v2::Task;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StatTaskRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).stat_task(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StatTaskSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/scheduler.v2.Scheduler/DeleteTask" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteTaskSvc<T: Scheduler>(pub Arc<T>);
                    impl<
                        T: Scheduler,
                    > tonic::server::UnaryService<super::DeleteTaskRequest>
                    for DeleteTaskSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteTaskRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).delete_task(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteTaskSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/scheduler.v2.Scheduler/AnnounceHost" => {
                    #[allow(non_camel_case_types)]
                    struct AnnounceHostSvc<T: Scheduler>(pub Arc<T>);
                    impl<
                        T: Scheduler,
                    > tonic::server::UnaryService<super::AnnounceHostRequest>
                    for AnnounceHostSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AnnounceHostRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).announce_host(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AnnounceHostSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/scheduler.v2.Scheduler/DeleteHost" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteHostSvc<T: Scheduler>(pub Arc<T>);
                    impl<
                        T: Scheduler,
                    > tonic::server::UnaryService<super::DeleteHostRequest>
                    for DeleteHostSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteHostRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).delete_host(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteHostSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/scheduler.v2.Scheduler/SyncProbes" => {
                    #[allow(non_camel_case_types)]
                    struct SyncProbesSvc<T: Scheduler>(pub Arc<T>);
                    impl<
                        T: Scheduler,
                    > tonic::server::StreamingService<super::SyncProbesRequest>
                    for SyncProbesSvc<T> {
                        type Response = super::SyncProbesResponse;
                        type ResponseStream = T::SyncProbesStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::SyncProbesRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).sync_probes(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SyncProbesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/scheduler.v2.Scheduler/AnnounceCachePeer" => {
                    #[allow(non_camel_case_types)]
                    struct AnnounceCachePeerSvc<T: Scheduler>(pub Arc<T>);
                    impl<
                        T: Scheduler,
                    > tonic::server::StreamingService<super::AnnounceCachePeerRequest>
                    for AnnounceCachePeerSvc<T> {
                        type Response = super::AnnounceCachePeerResponse;
                        type ResponseStream = T::AnnounceCachePeerStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::AnnounceCachePeerRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).announce_cache_peer(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AnnounceCachePeerSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/scheduler.v2.Scheduler/StatCachePeer" => {
                    #[allow(non_camel_case_types)]
                    struct StatCachePeerSvc<T: Scheduler>(pub Arc<T>);
                    impl<
                        T: Scheduler,
                    > tonic::server::UnaryService<super::StatCachePeerRequest>
                    for StatCachePeerSvc<T> {
                        type Response = super::super::super::common::v2::CachePeer;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StatCachePeerRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).stat_cache_peer(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StatCachePeerSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/scheduler.v2.Scheduler/DeleteCachePeer" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteCachePeerSvc<T: Scheduler>(pub Arc<T>);
                    impl<
                        T: Scheduler,
                    > tonic::server::UnaryService<super::DeleteCachePeerRequest>
                    for DeleteCachePeerSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteCachePeerRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).delete_cache_peer(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteCachePeerSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/scheduler.v2.Scheduler/UploadCacheTaskStarted" => {
                    #[allow(non_camel_case_types)]
                    struct UploadCacheTaskStartedSvc<T: Scheduler>(pub Arc<T>);
                    impl<
                        T: Scheduler,
                    > tonic::server::UnaryService<super::UploadCacheTaskStartedRequest>
                    for UploadCacheTaskStartedSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UploadCacheTaskStartedRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).upload_cache_task_started(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UploadCacheTaskStartedSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/scheduler.v2.Scheduler/UploadCacheTaskFinished" => {
                    #[allow(non_camel_case_types)]
                    struct UploadCacheTaskFinishedSvc<T: Scheduler>(pub Arc<T>);
                    impl<
                        T: Scheduler,
                    > tonic::server::UnaryService<super::UploadCacheTaskFinishedRequest>
                    for UploadCacheTaskFinishedSvc<T> {
                        type Response = super::super::super::common::v2::CacheTask;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::UploadCacheTaskFinishedRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).upload_cache_task_finished(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UploadCacheTaskFinishedSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/scheduler.v2.Scheduler/UploadCacheTaskFailed" => {
                    #[allow(non_camel_case_types)]
                    struct UploadCacheTaskFailedSvc<T: Scheduler>(pub Arc<T>);
                    impl<
                        T: Scheduler,
                    > tonic::server::UnaryService<super::UploadCacheTaskFailedRequest>
                    for UploadCacheTaskFailedSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UploadCacheTaskFailedRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).upload_cache_task_failed(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UploadCacheTaskFailedSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/scheduler.v2.Scheduler/StatCacheTask" => {
                    #[allow(non_camel_case_types)]
                    struct StatCacheTaskSvc<T: Scheduler>(pub Arc<T>);
                    impl<
                        T: Scheduler,
                    > tonic::server::UnaryService<super::StatCacheTaskRequest>
                    for StatCacheTaskSvc<T> {
                        type Response = super::super::super::common::v2::CacheTask;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StatCacheTaskRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).stat_cache_task(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StatCacheTaskSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/scheduler.v2.Scheduler/DeleteCacheTask" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteCacheTaskSvc<T: Scheduler>(pub Arc<T>);
                    impl<
                        T: Scheduler,
                    > tonic::server::UnaryService<super::DeleteCacheTaskRequest>
                    for DeleteCacheTaskSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteCacheTaskRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).delete_cache_task(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteCacheTaskSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: Scheduler> Clone for SchedulerServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: Scheduler> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Scheduler> tonic::server::NamedService for SchedulerServer<T> {
        const NAME: &'static str = "scheduler.v2.Scheduler";
    }
}
