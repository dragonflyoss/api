/// RegisterPeerRequest represents peer registered request of AnnouncePeerRequest.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterPeerRequest {
    /// Task id.
    #[prost(string, tag = "1")]
    pub task_id: ::prost::alloc::string::String,
    /// Peer id.
    #[prost(string, tag = "2")]
    pub peer_id: ::prost::alloc::string::String,
    /// Task metadata.
    #[prost(message, optional, tag = "3")]
    pub metadata: ::core::option::Option<super::common::Metadata>,
}
/// DownloadPeerStartedRequest represents peer download started request of AnnouncePeerRequest.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DownloadPeerStartedRequest {}
/// DownloadPeerBackToSourceStartedRequest represents peer download back-to-source started request of AnnouncePeerRequest.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DownloadPeerBackToSourceStartedRequest {
    /// Download back-to-source reason.
    #[prost(string, tag = "1")]
    pub reason: ::prost::alloc::string::String,
}
/// DownloadPeerFinishedRequest represents peer download finished request of AnnouncePeerRequest.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DownloadPeerFinishedRequest {
    /// Total content length.
    #[prost(int64, tag = "1")]
    pub content_length: i64,
    /// Total piece count.
    #[prost(int64, tag = "2")]
    pub piece_count: i64,
}
/// DownloadPeerBackToSourceFinishedRequest represents peer download back-to-source finished request of AnnouncePeerRequest.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DownloadPeerBackToSourceFinishedRequest {
    /// Total content length.
    #[prost(int64, tag = "1")]
    pub content_length: i64,
    /// Total piece count.
    #[prost(int64, tag = "2")]
    pub piece_count: i64,
}
/// DownloadPieceFinishedRequest represents piece download finished request of AnnouncePeerRequest.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DownloadPieceFinishedRequest {
    /// Piece info.
    #[prost(message, optional, tag = "1")]
    pub piece: ::core::option::Option<super::common::Piece>,
}
/// DownloadPieceBackToSourceFinishedRequest represents piece download back-to-source finished request of AnnouncePeerRequest.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DownloadPieceBackToSourceFinishedRequest {
    /// Piece info.
    #[prost(message, optional, tag = "1")]
    pub piece: ::core::option::Option<super::common::Piece>,
}
/// AnnouncePeerRequest represents request of AnnouncePeer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnouncePeerRequest {
    #[prost(oneof = "announce_peer_request::Request", tags = "1, 2, 3, 4, 5, 6, 7")]
    pub request: ::core::option::Option<announce_peer_request::Request>,
    #[prost(oneof = "announce_peer_request::Errordetails", tags = "8, 9, 10, 11")]
    pub errordetails: ::core::option::Option<announce_peer_request::Errordetails>,
}
/// Nested message and enum types in `AnnouncePeerRequest`.
pub mod announce_peer_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Request {
        #[prost(message, tag = "1")]
        RegisterPeerRequest(super::RegisterPeerRequest),
        #[prost(message, tag = "2")]
        DownloadPeerStartedRequest(super::DownloadPeerStartedRequest),
        #[prost(message, tag = "3")]
        DownloadPeerBackToSourceStartedRequest(
            super::DownloadPeerBackToSourceStartedRequest,
        ),
        #[prost(message, tag = "4")]
        DownloadPeerFinishedRequest(super::DownloadPeerFinishedRequest),
        #[prost(message, tag = "5")]
        DownloadPeerBackToSourceFinishedRequest(
            super::DownloadPeerBackToSourceFinishedRequest,
        ),
        #[prost(message, tag = "6")]
        DownloadPieceFinishedRequest(super::DownloadPieceFinishedRequest),
        #[prost(message, tag = "7")]
        DownloadPieceBackToSourceFinishedRequest(
            super::DownloadPieceBackToSourceFinishedRequest,
        ),
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Errordetails {
        #[prost(message, tag = "8")]
        DownloadPeerBackToSourceFailed(
            super::super::errordetails::DownloadPeerBackToSourceFailed,
        ),
        #[prost(message, tag = "9")]
        DownloadPieceBackToSourceFailed(
            super::super::errordetails::DownloadPieceBackToSourceFailed,
        ),
        #[prost(message, tag = "10")]
        SyncPiecesFailed(super::super::errordetails::SyncPiecesFailed),
        #[prost(message, tag = "11")]
        DownloadPieceFailed(super::super::errordetails::DownloadPieceFailed),
    }
}
/// TinyTaskResponse represents tiny task response of AnnouncePeerResponse.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TinyTaskResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// SmallTaskResponse represents small task response of AnnouncePeerResponse.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SmallTaskResponse {
    /// Piece info.
    #[prost(message, optional, tag = "1")]
    pub piece: ::core::option::Option<super::common::Piece>,
}
/// NormalTaskResponse represents normal task response of AnnouncePeerResponse.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NormalTaskResponse {
    /// Candidate parents.
    #[prost(message, repeated, tag = "1")]
    pub candidate_parents: ::prost::alloc::vec::Vec<super::common::Peer>,
    /// Concurrent downloading count from main peer.
    #[prost(int32, tag = "2")]
    pub parallel_count: i32,
}
/// NeedBackToSourceResponse represents need back-to-source response of AnnouncePeerResponse.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NeedBackToSourceResponse {
    /// Download back-to-source reason.
    #[prost(string, tag = "1")]
    pub reason: ::prost::alloc::string::String,
}
/// AnnouncePeerResponse represents response of AnnouncePeer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnouncePeerResponse {
    #[prost(oneof = "announce_peer_response::Response", tags = "1, 2, 3, 4")]
    pub response: ::core::option::Option<announce_peer_response::Response>,
    #[prost(oneof = "announce_peer_response::Errordetails", tags = "5, 6")]
    pub errordetails: ::core::option::Option<announce_peer_response::Errordetails>,
}
/// Nested message and enum types in `AnnouncePeerResponse`.
pub mod announce_peer_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag = "1")]
        TinyTaskResponse(super::TinyTaskResponse),
        #[prost(message, tag = "2")]
        SmallTaskResponse(super::SmallTaskResponse),
        #[prost(message, tag = "3")]
        NormalTaskResponse(super::NormalTaskResponse),
        #[prost(message, tag = "4")]
        NeedBackToSourceResponse(super::NeedBackToSourceResponse),
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Errordetails {
        #[prost(message, tag = "5")]
        SchedulePeerForbidden(super::super::errordetails::SchedulePeerForbidden),
        #[prost(message, tag = "6")]
        SchedulePeerFailed(super::super::errordetails::SchedulePeerFailed),
    }
}
/// StatPeerRequest represents request of StatPeer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatPeerRequest {
    /// Task id.
    #[prost(string, tag = "1")]
    pub task_id: ::prost::alloc::string::String,
    /// Peer id.
    #[prost(string, tag = "2")]
    pub peer_id: ::prost::alloc::string::String,
}
/// TODO exchange peer request definition.
/// ExchangePeerRequest represents request of ExchangePeer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExchangePeerRequest {
    /// Task id.
    #[prost(string, tag = "1")]
    pub task_id: ::prost::alloc::string::String,
    /// Peer id.
    #[prost(string, tag = "2")]
    pub peer_id: ::prost::alloc::string::String,
}
/// TODO exchange peer response definition.
/// ExchangePeerResponse represents response of ExchangePeer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExchangePeerResponse {}
/// LeavePeerRequest represents request of LeavePeer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeavePeerRequest {
    /// Peer id.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
/// StatTaskRequest represents request of StatTask.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatTaskRequest {
    /// Task id.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
/// AnnounceHostRequest represents request of AnnounceHost.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnounceHostRequest {
    /// Host id.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Host type.
    #[prost(uint32, tag = "2")]
    pub r#type: u32,
    /// Hostname.
    #[prost(string, tag = "3")]
    pub hostname: ::prost::alloc::string::String,
    /// Host ip.
    #[prost(string, tag = "4")]
    pub ip: ::prost::alloc::string::String,
    /// Port of grpc service.
    #[prost(int32, tag = "5")]
    pub port: i32,
    /// Port of download server.
    #[prost(int32, tag = "6")]
    pub download_port: i32,
    /// Host OS.
    #[prost(string, tag = "7")]
    pub os: ::prost::alloc::string::String,
    /// Host platform.
    #[prost(string, tag = "8")]
    pub platform: ::prost::alloc::string::String,
    /// Host platform family.
    #[prost(string, tag = "9")]
    pub platform_family: ::prost::alloc::string::String,
    /// Host platform version.
    #[prost(string, tag = "10")]
    pub platform_version: ::prost::alloc::string::String,
    /// Host kernel version.
    #[prost(string, tag = "11")]
    pub kernel_version: ::prost::alloc::string::String,
    /// CPU Stat.
    #[prost(message, optional, tag = "12")]
    pub cpu: ::core::option::Option<Cpu>,
    /// Memory Stat.
    #[prost(message, optional, tag = "13")]
    pub memory: ::core::option::Option<Memory>,
    /// Network Stat.
    #[prost(message, optional, tag = "14")]
    pub network: ::core::option::Option<Network>,
    /// Disk Stat.
    #[prost(message, optional, tag = "15")]
    pub disk: ::core::option::Option<Disk>,
    /// Build information.
    #[prost(message, optional, tag = "16")]
    pub build: ::core::option::Option<Build>,
}
/// CPU Stat.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cpu {
    /// Number of logical cores in the system.
    #[prost(uint32, tag = "1")]
    pub logical_count: u32,
    /// Number of physical cores in the system
    #[prost(uint32, tag = "2")]
    pub physical_count: u32,
    /// Percent calculates the percentage of cpu used.
    #[prost(double, tag = "3")]
    pub percent: f64,
    /// Calculates the percentage of cpu used by process.
    #[prost(double, tag = "4")]
    pub process_percent: f64,
    /// CPUTimes contains the amounts of time the CPU has spent performing different kinds of work.
    #[prost(message, optional, tag = "5")]
    pub times: ::core::option::Option<CpuTimes>,
}
/// CPUTimes contains the amounts of time the CPU has spent performing different
/// kinds of work. Time units are in seconds.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CpuTimes {
    /// CPU time of user.
    #[prost(double, tag = "1")]
    pub user: f64,
    /// CPU time of system.
    #[prost(double, tag = "2")]
    pub system: f64,
    /// CPU time of idle.
    #[prost(double, tag = "3")]
    pub idle: f64,
    /// CPU time of nice.
    #[prost(double, tag = "4")]
    pub nice: f64,
    /// CPU time of iowait.
    #[prost(double, tag = "5")]
    pub iowait: f64,
    /// CPU time of irq.
    #[prost(double, tag = "6")]
    pub irq: f64,
    /// CPU time of softirq.
    #[prost(double, tag = "7")]
    pub softirq: f64,
    /// CPU time of steal.
    #[prost(double, tag = "8")]
    pub steal: f64,
    /// CPU time of guest.
    #[prost(double, tag = "9")]
    pub guest: f64,
    /// CPU time of guest nice.
    #[prost(double, tag = "10")]
    pub guest_nice: f64,
}
/// Memory Stat.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Memory {
    /// Total amount of RAM on this system.
    #[prost(uint64, tag = "1")]
    pub total: u64,
    /// RAM available for programs to allocate.
    #[prost(uint64, tag = "2")]
    pub available: u64,
    /// RAM used by programs.
    #[prost(uint64, tag = "3")]
    pub used: u64,
    /// Percentage of RAM used by programs.
    #[prost(double, tag = "4")]
    pub used_percent: f64,
    /// Calculates the percentage of memory used by process.
    #[prost(double, tag = "5")]
    pub process_used_percent: f64,
    /// This is the kernel's notion of free memory.
    #[prost(uint64, tag = "6")]
    pub free: u64,
}
/// Network Stat.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Network {
    /// Return count of tcp connections opened and status is ESTABLISHED.
    #[prost(uint32, tag = "1")]
    pub tcp_connection_count: u32,
    /// Return count of upload tcp connections opened and status is ESTABLISHED.
    #[prost(uint32, tag = "2")]
    pub upload_tcp_connection_count: u32,
    /// Security domain for network.
    #[prost(string, tag = "3")]
    pub security_domain: ::prost::alloc::string::String,
    /// Location path(area|country|province|city|...).
    #[prost(string, tag = "4")]
    pub location: ::prost::alloc::string::String,
    /// IDC where the peer host is located
    #[prost(string, tag = "5")]
    pub idc: ::prost::alloc::string::String,
    /// Network topology(switch|router|...).
    #[prost(string, tag = "6")]
    pub net_topology: ::prost::alloc::string::String,
}
/// Disk Stat.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Disk {
    /// Total amount of disk on the data path of dragonfly.
    #[prost(uint64, tag = "1")]
    pub total: u64,
    /// Free amount of disk on the data path of dragonfly.
    #[prost(uint64, tag = "2")]
    pub free: u64,
    /// Used amount of disk on the data path of dragonfly.
    #[prost(uint64, tag = "3")]
    pub used: u64,
    /// Used percent of disk on the data path of dragonfly directory.
    #[prost(double, tag = "4")]
    pub used_percent: f64,
    /// Total amount of indoes on the data path of dragonfly directory.
    #[prost(uint64, tag = "5")]
    pub inodes_total: u64,
    /// Used amount of indoes on the data path of dragonfly directory.
    #[prost(uint64, tag = "6")]
    pub inodes_used: u64,
    /// Free amount of indoes on the data path of dragonfly directory.
    #[prost(uint64, tag = "7")]
    pub inodes_free: u64,
    /// Used percent of indoes on the data path of dragonfly directory.
    #[prost(double, tag = "8")]
    pub inodes_used_percent: f64,
}
/// Build information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Build {
    /// Git version.
    #[prost(string, tag = "1")]
    pub git_version: ::prost::alloc::string::String,
    /// Git commit.
    #[prost(string, tag = "2")]
    pub git_commit: ::prost::alloc::string::String,
    /// Golang version.
    #[prost(string, tag = "3")]
    pub go_version: ::prost::alloc::string::String,
    /// Build platform.
    #[prost(string, tag = "4")]
    pub platform: ::prost::alloc::string::String,
}
/// LeaveHostRequest represents request of LeaveHost.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeaveHostRequest {
    /// Host id.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
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
            D: std::convert::TryInto<tonic::transport::Endpoint>,
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
        /// AnnouncePeer announces peer to scheduler.
        pub async fn announce_peer(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::AnnouncePeerRequest,
            >,
        ) -> Result<
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
                "/scheduler.Scheduler/AnnouncePeer",
            );
            self.inner.streaming(request.into_streaming_request(), path, codec).await
        }
        /// Checks information of peer.
        pub async fn stat_peer(
            &mut self,
            request: impl tonic::IntoRequest<super::StatPeerRequest>,
        ) -> Result<tonic::Response<super::super::common::Peer>, tonic::Status> {
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
                "/scheduler.Scheduler/StatPeer",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// LeavePeer releases peer in scheduler.
        pub async fn leave_peer(
            &mut self,
            request: impl tonic::IntoRequest<super::LeavePeerRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
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
                "/scheduler.Scheduler/LeavePeer",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// TODO exchange peer api definition.
        /// ExchangePeer exchanges peer information.
        pub async fn exchange_peer(
            &mut self,
            request: impl tonic::IntoRequest<super::ExchangePeerRequest>,
        ) -> Result<tonic::Response<super::ExchangePeerResponse>, tonic::Status> {
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
                "/scheduler.Scheduler/ExchangePeer",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Checks information of task.
        pub async fn stat_task(
            &mut self,
            request: impl tonic::IntoRequest<super::StatTaskRequest>,
        ) -> Result<tonic::Response<super::super::common::Task>, tonic::Status> {
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
                "/scheduler.Scheduler/StatTask",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// AnnounceHost announces host to scheduler.
        pub async fn announce_host(
            &mut self,
            request: impl tonic::IntoRequest<super::AnnounceHostRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
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
                "/scheduler.Scheduler/AnnounceHost",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// LeaveHost releases host in scheduler.
        pub async fn leave_host(
            &mut self,
            request: impl tonic::IntoRequest<super::LeaveHostRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
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
                "/scheduler.Scheduler/LeaveHost",
            );
            self.inner.unary(request.into_request(), path, codec).await
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
                Item = Result<super::AnnouncePeerResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// AnnouncePeer announces peer to scheduler.
        async fn announce_peer(
            &self,
            request: tonic::Request<tonic::Streaming<super::AnnouncePeerRequest>>,
        ) -> Result<tonic::Response<Self::AnnouncePeerStream>, tonic::Status>;
        /// Checks information of peer.
        async fn stat_peer(
            &self,
            request: tonic::Request<super::StatPeerRequest>,
        ) -> Result<tonic::Response<super::super::common::Peer>, tonic::Status>;
        /// LeavePeer releases peer in scheduler.
        async fn leave_peer(
            &self,
            request: tonic::Request<super::LeavePeerRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        /// TODO exchange peer api definition.
        /// ExchangePeer exchanges peer information.
        async fn exchange_peer(
            &self,
            request: tonic::Request<super::ExchangePeerRequest>,
        ) -> Result<tonic::Response<super::ExchangePeerResponse>, tonic::Status>;
        /// Checks information of task.
        async fn stat_task(
            &self,
            request: tonic::Request<super::StatTaskRequest>,
        ) -> Result<tonic::Response<super::super::common::Task>, tonic::Status>;
        /// AnnounceHost announces host to scheduler.
        async fn announce_host(
            &self,
            request: tonic::Request<super::AnnounceHostRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        /// LeaveHost releases host in scheduler.
        async fn leave_host(
            &self,
            request: tonic::Request<super::LeaveHostRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
    }
    /// Scheduler RPC Service.
    #[derive(Debug)]
    pub struct SchedulerServer<T: Scheduler> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
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
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/scheduler.Scheduler/AnnouncePeer" => {
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
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).announce_peer(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AnnouncePeerSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/scheduler.Scheduler/StatPeer" => {
                    #[allow(non_camel_case_types)]
                    struct StatPeerSvc<T: Scheduler>(pub Arc<T>);
                    impl<
                        T: Scheduler,
                    > tonic::server::UnaryService<super::StatPeerRequest>
                    for StatPeerSvc<T> {
                        type Response = super::super::common::Peer;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StatPeerRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).stat_peer(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StatPeerSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/scheduler.Scheduler/LeavePeer" => {
                    #[allow(non_camel_case_types)]
                    struct LeavePeerSvc<T: Scheduler>(pub Arc<T>);
                    impl<
                        T: Scheduler,
                    > tonic::server::UnaryService<super::LeavePeerRequest>
                    for LeavePeerSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LeavePeerRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).leave_peer(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = LeavePeerSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/scheduler.Scheduler/ExchangePeer" => {
                    #[allow(non_camel_case_types)]
                    struct ExchangePeerSvc<T: Scheduler>(pub Arc<T>);
                    impl<
                        T: Scheduler,
                    > tonic::server::UnaryService<super::ExchangePeerRequest>
                    for ExchangePeerSvc<T> {
                        type Response = super::ExchangePeerResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ExchangePeerRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).exchange_peer(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ExchangePeerSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/scheduler.Scheduler/StatTask" => {
                    #[allow(non_camel_case_types)]
                    struct StatTaskSvc<T: Scheduler>(pub Arc<T>);
                    impl<
                        T: Scheduler,
                    > tonic::server::UnaryService<super::StatTaskRequest>
                    for StatTaskSvc<T> {
                        type Response = super::super::common::Task;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StatTaskRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).stat_task(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StatTaskSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/scheduler.Scheduler/AnnounceHost" => {
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
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).announce_host(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AnnounceHostSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/scheduler.Scheduler/LeaveHost" => {
                    #[allow(non_camel_case_types)]
                    struct LeaveHostSvc<T: Scheduler>(pub Arc<T>);
                    impl<
                        T: Scheduler,
                    > tonic::server::UnaryService<super::LeaveHostRequest>
                    for LeaveHostSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LeaveHostRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).leave_host(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = LeaveHostSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
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
            }
        }
    }
    impl<T: Scheduler> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Scheduler> tonic::server::NamedService for SchedulerServer<T> {
        const NAME: &'static str = "scheduler.Scheduler";
    }
}
