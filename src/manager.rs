/// SecurityGroup represents security group of cluster.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecurityGroup {
    /// Group id.
    #[prost(uint64, tag = "1")]
    pub id: u64,
    /// Group name.
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// Group biography.
    #[prost(string, tag = "3")]
    pub bio: ::prost::alloc::string::String,
    /// Group domain.
    #[prost(string, tag = "4")]
    pub domain: ::prost::alloc::string::String,
    /// Group proxy domain.
    #[prost(string, tag = "5")]
    pub proxy_domain: ::prost::alloc::string::String,
}
/// SeedPeerCluster represents cluster of seed peer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SeedPeerCluster {
    /// Cluster id.
    #[prost(uint64, tag = "1")]
    pub id: u64,
    /// Cluster name.
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// Cluster biography.
    #[prost(string, tag = "3")]
    pub bio: ::prost::alloc::string::String,
    /// Cluster configuration.
    #[prost(bytes = "vec", tag = "4")]
    pub config: ::prost::alloc::vec::Vec<u8>,
}
/// SeedPeer represents seed peer for network.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SeedPeer {
    /// Seed peer id.
    #[prost(uint64, tag = "1")]
    pub id: u64,
    /// Seed peer hostname.
    #[prost(string, tag = "2")]
    pub host_name: ::prost::alloc::string::String,
    /// Seed peer type.
    #[prost(string, tag = "3")]
    pub r#type: ::prost::alloc::string::String,
    /// Seed peer idc.
    #[prost(string, tag = "4")]
    pub idc: ::prost::alloc::string::String,
    /// Seed peer location.
    #[prost(string, tag = "5")]
    pub location: ::prost::alloc::string::String,
    /// Seed peer ip.
    #[prost(string, tag = "6")]
    pub ip: ::prost::alloc::string::String,
    /// Seed peer grpc port.
    #[prost(int32, tag = "7")]
    pub port: i32,
    /// Seed peer download port.
    #[prost(int32, tag = "8")]
    pub download_port: i32,
    /// Seed peer state.
    #[prost(string, tag = "9")]
    pub state: ::prost::alloc::string::String,
    /// ID of the cluster to which the seed peer belongs.
    #[prost(uint64, tag = "10")]
    pub seed_peer_cluster_id: u64,
    /// Cluster to which the seed peer belongs.
    #[prost(message, optional, tag = "11")]
    pub seed_peer_cluster: ::core::option::Option<SeedPeerCluster>,
    /// Schedulers included in seed peer.
    #[prost(message, repeated, tag = "12")]
    pub schedulers: ::prost::alloc::vec::Vec<Scheduler>,
    /// Seed peer object storage port.
    #[prost(int32, tag = "13")]
    pub object_storage_port: i32,
}
/// GetSeedPeerRequest represents request of GetSeedPeer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSeedPeerRequest {
    /// Request source type.
    #[prost(enumeration = "SourceType", tag = "1")]
    pub source_type: i32,
    /// Seed peer hostname.
    #[prost(string, tag = "2")]
    pub host_name: ::prost::alloc::string::String,
    /// ID of the cluster to which the seed peer belongs.
    #[prost(uint64, tag = "3")]
    pub seed_peer_cluster_id: u64,
    /// Seed peer ip.
    #[prost(string, tag = "4")]
    pub ip: ::prost::alloc::string::String,
}
/// UpdateSeedPeerRequest represents request of UpdateSeedPeer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSeedPeerRequest {
    /// Request source type.
    #[prost(enumeration = "SourceType", tag = "1")]
    pub source_type: i32,
    /// Seed peer hostname.
    #[prost(string, tag = "2")]
    pub host_name: ::prost::alloc::string::String,
    /// Seed peer type.
    #[prost(string, tag = "3")]
    pub r#type: ::prost::alloc::string::String,
    /// Seed peer idc.
    #[prost(string, tag = "4")]
    pub idc: ::prost::alloc::string::String,
    /// Seed peer location.
    #[prost(string, tag = "5")]
    pub location: ::prost::alloc::string::String,
    /// Seed peer ip.
    #[prost(string, tag = "6")]
    pub ip: ::prost::alloc::string::String,
    /// Seed peer port.
    #[prost(int32, tag = "7")]
    pub port: i32,
    /// Seed peer download port.
    #[prost(int32, tag = "8")]
    pub download_port: i32,
    /// ID of the cluster to which the seed peer belongs.
    #[prost(uint64, tag = "9")]
    pub seed_peer_cluster_id: u64,
    /// Seed peer object storage port.
    #[prost(int32, tag = "10")]
    pub object_storage_port: i32,
}
/// SeedPeerCluster represents cluster of scheduler.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SchedulerCluster {
    /// Cluster id.
    #[prost(uint64, tag = "1")]
    pub id: u64,
    /// Cluster name.
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// Cluster biography.
    #[prost(string, tag = "3")]
    pub bio: ::prost::alloc::string::String,
    /// Cluster config.
    #[prost(bytes = "vec", tag = "4")]
    pub config: ::prost::alloc::vec::Vec<u8>,
    /// Cluster client config.
    #[prost(bytes = "vec", tag = "5")]
    pub client_config: ::prost::alloc::vec::Vec<u8>,
    /// Cluster scopes.
    #[prost(bytes = "vec", tag = "6")]
    pub scopes: ::prost::alloc::vec::Vec<u8>,
    /// Security group to which the scheduler cluster belongs.
    #[prost(message, optional, tag = "7")]
    pub security_group: ::core::option::Option<SecurityGroup>,
}
/// SeedPeerCluster represents scheduler for network.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Scheduler {
    /// Scheduler id.
    #[prost(uint64, tag = "1")]
    pub id: u64,
    /// Scheduler hostname.
    #[prost(string, tag = "2")]
    pub host_name: ::prost::alloc::string::String,
    /// Scheduler idc.
    #[prost(string, tag = "3")]
    pub idc: ::prost::alloc::string::String,
    /// Scheduler location.
    #[prost(string, tag = "4")]
    pub location: ::prost::alloc::string::String,
    /// Scheduler ip.
    #[prost(string, tag = "5")]
    pub ip: ::prost::alloc::string::String,
    /// Scheduler grpc port.
    #[prost(int32, tag = "6")]
    pub port: i32,
    /// Scheduler state.
    #[prost(string, tag = "7")]
    pub state: ::prost::alloc::string::String,
    /// ID of the cluster to which the scheduler belongs.
    #[prost(uint64, tag = "8")]
    pub scheduler_cluster_id: u64,
    /// Cluster to which the scheduler belongs.
    #[prost(message, optional, tag = "9")]
    pub scheduler_cluster: ::core::option::Option<SchedulerCluster>,
    /// Seed peers to which the scheduler belongs.
    #[prost(message, repeated, tag = "10")]
    pub seed_peers: ::prost::alloc::vec::Vec<SeedPeer>,
}
/// GetSchedulerRequest represents request of GetScheduler.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSchedulerRequest {
    /// Request source type.
    #[prost(enumeration = "SourceType", tag = "1")]
    pub source_type: i32,
    /// Scheduler hostname.
    #[prost(string, tag = "2")]
    pub host_name: ::prost::alloc::string::String,
    /// ID of the cluster to which the scheduler belongs.
    #[prost(uint64, tag = "3")]
    pub scheduler_cluster_id: u64,
    /// Scheduler ip.
    #[prost(string, tag = "4")]
    pub ip: ::prost::alloc::string::String,
}
/// UpdateSchedulerRequest represents request of UpdateScheduler.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSchedulerRequest {
    /// Request source type.
    #[prost(enumeration = "SourceType", tag = "1")]
    pub source_type: i32,
    /// Scheduler hostname.
    #[prost(string, tag = "2")]
    pub host_name: ::prost::alloc::string::String,
    /// ID of the cluster to which the scheduler belongs.
    #[prost(uint64, tag = "3")]
    pub scheduler_cluster_id: u64,
    /// Scheduler idc.
    #[prost(string, tag = "4")]
    pub idc: ::prost::alloc::string::String,
    /// Scheduler location.
    #[prost(string, tag = "5")]
    pub location: ::prost::alloc::string::String,
    /// Scheduler ip.
    #[prost(string, tag = "6")]
    pub ip: ::prost::alloc::string::String,
    /// Scheduler port.
    #[prost(int32, tag = "7")]
    pub port: i32,
}
/// ListSchedulersRequest represents request of ListSchedulers.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSchedulersRequest {
    /// Request source type.
    #[prost(enumeration = "SourceType", tag = "1")]
    pub source_type: i32,
    /// Source service hostname.
    #[prost(string, tag = "2")]
    pub host_name: ::prost::alloc::string::String,
    /// Source service ip.
    #[prost(string, tag = "3")]
    pub ip: ::prost::alloc::string::String,
    /// Source service host information.
    #[prost(map = "string, string", tag = "4")]
    pub host_info: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Dfdaemon version.
    #[prost(string, tag = "5")]
    pub version: ::prost::alloc::string::String,
    /// Dfdaemon commit.
    #[prost(string, tag = "6")]
    pub commit: ::prost::alloc::string::String,
}
/// ListSchedulersResponse represents response of ListSchedulers.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSchedulersResponse {
    /// Schedulers to which the source service belongs.
    #[prost(message, repeated, tag = "1")]
    pub schedulers: ::prost::alloc::vec::Vec<Scheduler>,
}
/// ObjectStorage represents config of object storage.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectStorage {
    /// name is object storage name of type, it can be s3, oss or obs.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Region is storage region.
    #[prost(string, tag = "2")]
    pub region: ::prost::alloc::string::String,
    /// Endpoint is datacenter endpoint.
    #[prost(string, tag = "3")]
    pub endpoint: ::prost::alloc::string::String,
    /// AccessKey is access key ID.
    #[prost(string, tag = "4")]
    pub access_key: ::prost::alloc::string::String,
    /// SecretKey is access key secret.
    #[prost(string, tag = "5")]
    pub secret_key: ::prost::alloc::string::String,
    /// S3ForcePathStyle sets force path style for s3, true by default.
    /// Set this to `true` to force the request to use path-style addressing,
    /// i.e., `<http://s3.amazonaws.com/BUCKET/KEY`.> By default, the S3 client
    /// will use virtual hosted bucket addressing when possible
    /// (`<http://BUCKET.s3.amazonaws.com/KEY`>).
    /// Refer to <https://github.com/aws/aws-sdk-go/blob/main/aws/config.go#L118.>
    #[prost(bool, tag = "6")]
    pub s3_force_path_style: bool,
}
/// GetObjectStorageRequest represents request of GetObjectStorage.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectStorageRequest {
    /// Request source type.
    #[prost(enumeration = "SourceType", tag = "1")]
    pub source_type: i32,
    /// Source service hostname.
    #[prost(string, tag = "2")]
    pub host_name: ::prost::alloc::string::String,
    /// Source service ip.
    #[prost(string, tag = "3")]
    pub ip: ::prost::alloc::string::String,
}
/// Bucket represents config of bucket.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bucket {
    /// Bucket name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// ListSchedulersRequest represents request of ListBuckets.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBucketsRequest {
    /// Request source type.
    #[prost(enumeration = "SourceType", tag = "1")]
    pub source_type: i32,
    /// Source service hostname.
    #[prost(string, tag = "2")]
    pub host_name: ::prost::alloc::string::String,
    /// Source service ip.
    #[prost(string, tag = "3")]
    pub ip: ::prost::alloc::string::String,
}
/// ListBucketsResponse represents response of ListBuckets.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBucketsResponse {
    /// Bucket configs.
    #[prost(message, repeated, tag = "1")]
    pub buckets: ::prost::alloc::vec::Vec<Bucket>,
}
/// URLPriority represents config of url priority.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UrlPriority {
    /// URL regex.
    #[prost(string, tag = "1")]
    pub regex: ::prost::alloc::string::String,
    /// URL priority value.
    #[prost(enumeration = "super::common::Priority", tag = "2")]
    pub value: i32,
}
/// ApplicationPriority represents config of application priority.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplicationPriority {
    /// Priority value.
    #[prost(enumeration = "super::common::Priority", tag = "1")]
    pub value: i32,
    /// URL priority.
    #[prost(message, repeated, tag = "2")]
    pub urls: ::prost::alloc::vec::Vec<UrlPriority>,
}
/// Application represents config of application.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Application {
    /// Application id.
    #[prost(uint64, tag = "1")]
    pub id: u64,
    /// Application name.
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// Application url.
    #[prost(string, tag = "3")]
    pub url: ::prost::alloc::string::String,
    /// Application biography.
    #[prost(string, tag = "4")]
    pub bio: ::prost::alloc::string::String,
    /// Application priority.
    #[prost(message, optional, tag = "5")]
    pub priority: ::core::option::Option<ApplicationPriority>,
}
/// ListApplicationsRequest represents request of ListApplications.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListApplicationsRequest {
    /// Request source type.
    #[prost(enumeration = "SourceType", tag = "1")]
    pub source_type: i32,
    /// Source service hostname.
    #[prost(string, tag = "2")]
    pub host_name: ::prost::alloc::string::String,
    /// Source service ip.
    #[prost(string, tag = "3")]
    pub ip: ::prost::alloc::string::String,
}
/// ListApplicationsResponse represents response of ListApplications.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListApplicationsResponse {
    /// Application configs.
    #[prost(message, repeated, tag = "1")]
    pub applications: ::prost::alloc::vec::Vec<Application>,
}
/// KeepAliveRequest represents request of KeepAlive.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeepAliveRequest {
    /// Request source type.
    #[prost(enumeration = "SourceType", tag = "1")]
    pub source_type: i32,
    /// Source service hostname.
    #[prost(string, tag = "2")]
    pub host_name: ::prost::alloc::string::String,
    /// ID of the cluster to which the source service belongs.
    #[prost(uint64, tag = "3")]
    pub cluster_id: u64,
    /// Source service ip.
    #[prost(string, tag = "4")]
    pub ip: ::prost::alloc::string::String,
}
/// Request source type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SourceType {
    /// Scheduler service.
    SchedulerSource = 0,
    /// Peer service.
    PeerSource = 1,
    /// SeedPeer service.
    SeedPeerSource = 2,
}
impl SourceType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SourceType::SchedulerSource => "SCHEDULER_SOURCE",
            SourceType::PeerSource => "PEER_SOURCE",
            SourceType::SeedPeerSource => "SEED_PEER_SOURCE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SCHEDULER_SOURCE" => Some(Self::SchedulerSource),
            "PEER_SOURCE" => Some(Self::PeerSource),
            "SEED_PEER_SOURCE" => Some(Self::SeedPeerSource),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod manager_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Manager RPC Service.
    #[derive(Debug, Clone)]
    pub struct ManagerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ManagerClient<tonic::transport::Channel> {
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
    impl<T> ManagerClient<T>
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
        ) -> ManagerClient<InterceptedService<T, F>>
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
            ManagerClient::new(InterceptedService::new(inner, interceptor))
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
        /// Get SeedPeer and SeedPeer cluster configuration.
        pub async fn get_seed_peer(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSeedPeerRequest>,
        ) -> Result<tonic::Response<super::SeedPeer>, tonic::Status> {
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
                "/manager.Manager/GetSeedPeer",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Update SeedPeer configuration.
        pub async fn update_seed_peer(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSeedPeerRequest>,
        ) -> Result<tonic::Response<super::SeedPeer>, tonic::Status> {
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
                "/manager.Manager/UpdateSeedPeer",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Get Scheduler and Scheduler cluster configuration.
        pub async fn get_scheduler(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSchedulerRequest>,
        ) -> Result<tonic::Response<super::Scheduler>, tonic::Status> {
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
                "/manager.Manager/GetScheduler",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Update scheduler configuration.
        pub async fn update_scheduler(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSchedulerRequest>,
        ) -> Result<tonic::Response<super::Scheduler>, tonic::Status> {
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
                "/manager.Manager/UpdateScheduler",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// List acitve schedulers configuration.
        pub async fn list_schedulers(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSchedulersRequest>,
        ) -> Result<tonic::Response<super::ListSchedulersResponse>, tonic::Status> {
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
                "/manager.Manager/ListSchedulers",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Get ObjectStorage configuration.
        pub async fn get_object_storage(
            &mut self,
            request: impl tonic::IntoRequest<super::GetObjectStorageRequest>,
        ) -> Result<tonic::Response<super::ObjectStorage>, tonic::Status> {
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
                "/manager.Manager/GetObjectStorage",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// List buckets configuration.
        pub async fn list_buckets(
            &mut self,
            request: impl tonic::IntoRequest<super::ListBucketsRequest>,
        ) -> Result<tonic::Response<super::ListBucketsResponse>, tonic::Status> {
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
                "/manager.Manager/ListBuckets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// List applications configuration.
        pub async fn list_applications(
            &mut self,
            request: impl tonic::IntoRequest<super::ListApplicationsRequest>,
        ) -> Result<tonic::Response<super::ListApplicationsResponse>, tonic::Status> {
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
                "/manager.Manager/ListApplications",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// KeepAlive with manager.
        pub async fn keep_alive(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::KeepAliveRequest>,
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
                "/manager.Manager/KeepAlive",
            );
            self.inner
                .client_streaming(request.into_streaming_request(), path, codec)
                .await
        }
    }
}
/// Generated server implementations.
pub mod manager_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ManagerServer.
    #[async_trait]
    pub trait Manager: Send + Sync + 'static {
        /// Get SeedPeer and SeedPeer cluster configuration.
        async fn get_seed_peer(
            &self,
            request: tonic::Request<super::GetSeedPeerRequest>,
        ) -> Result<tonic::Response<super::SeedPeer>, tonic::Status>;
        /// Update SeedPeer configuration.
        async fn update_seed_peer(
            &self,
            request: tonic::Request<super::UpdateSeedPeerRequest>,
        ) -> Result<tonic::Response<super::SeedPeer>, tonic::Status>;
        /// Get Scheduler and Scheduler cluster configuration.
        async fn get_scheduler(
            &self,
            request: tonic::Request<super::GetSchedulerRequest>,
        ) -> Result<tonic::Response<super::Scheduler>, tonic::Status>;
        /// Update scheduler configuration.
        async fn update_scheduler(
            &self,
            request: tonic::Request<super::UpdateSchedulerRequest>,
        ) -> Result<tonic::Response<super::Scheduler>, tonic::Status>;
        /// List acitve schedulers configuration.
        async fn list_schedulers(
            &self,
            request: tonic::Request<super::ListSchedulersRequest>,
        ) -> Result<tonic::Response<super::ListSchedulersResponse>, tonic::Status>;
        /// Get ObjectStorage configuration.
        async fn get_object_storage(
            &self,
            request: tonic::Request<super::GetObjectStorageRequest>,
        ) -> Result<tonic::Response<super::ObjectStorage>, tonic::Status>;
        /// List buckets configuration.
        async fn list_buckets(
            &self,
            request: tonic::Request<super::ListBucketsRequest>,
        ) -> Result<tonic::Response<super::ListBucketsResponse>, tonic::Status>;
        /// List applications configuration.
        async fn list_applications(
            &self,
            request: tonic::Request<super::ListApplicationsRequest>,
        ) -> Result<tonic::Response<super::ListApplicationsResponse>, tonic::Status>;
        /// KeepAlive with manager.
        async fn keep_alive(
            &self,
            request: tonic::Request<tonic::Streaming<super::KeepAliveRequest>>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
    }
    /// Manager RPC Service.
    #[derive(Debug)]
    pub struct ManagerServer<T: Manager> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Manager> ManagerServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ManagerServer<T>
    where
        T: Manager,
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
                "/manager.Manager/GetSeedPeer" => {
                    #[allow(non_camel_case_types)]
                    struct GetSeedPeerSvc<T: Manager>(pub Arc<T>);
                    impl<
                        T: Manager,
                    > tonic::server::UnaryService<super::GetSeedPeerRequest>
                    for GetSeedPeerSvc<T> {
                        type Response = super::SeedPeer;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetSeedPeerRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_seed_peer(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetSeedPeerSvc(inner);
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
                "/manager.Manager/UpdateSeedPeer" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateSeedPeerSvc<T: Manager>(pub Arc<T>);
                    impl<
                        T: Manager,
                    > tonic::server::UnaryService<super::UpdateSeedPeerRequest>
                    for UpdateSeedPeerSvc<T> {
                        type Response = super::SeedPeer;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateSeedPeerRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).update_seed_peer(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateSeedPeerSvc(inner);
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
                "/manager.Manager/GetScheduler" => {
                    #[allow(non_camel_case_types)]
                    struct GetSchedulerSvc<T: Manager>(pub Arc<T>);
                    impl<
                        T: Manager,
                    > tonic::server::UnaryService<super::GetSchedulerRequest>
                    for GetSchedulerSvc<T> {
                        type Response = super::Scheduler;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetSchedulerRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_scheduler(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetSchedulerSvc(inner);
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
                "/manager.Manager/UpdateScheduler" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateSchedulerSvc<T: Manager>(pub Arc<T>);
                    impl<
                        T: Manager,
                    > tonic::server::UnaryService<super::UpdateSchedulerRequest>
                    for UpdateSchedulerSvc<T> {
                        type Response = super::Scheduler;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateSchedulerRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).update_scheduler(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateSchedulerSvc(inner);
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
                "/manager.Manager/ListSchedulers" => {
                    #[allow(non_camel_case_types)]
                    struct ListSchedulersSvc<T: Manager>(pub Arc<T>);
                    impl<
                        T: Manager,
                    > tonic::server::UnaryService<super::ListSchedulersRequest>
                    for ListSchedulersSvc<T> {
                        type Response = super::ListSchedulersResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListSchedulersRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).list_schedulers(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListSchedulersSvc(inner);
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
                "/manager.Manager/GetObjectStorage" => {
                    #[allow(non_camel_case_types)]
                    struct GetObjectStorageSvc<T: Manager>(pub Arc<T>);
                    impl<
                        T: Manager,
                    > tonic::server::UnaryService<super::GetObjectStorageRequest>
                    for GetObjectStorageSvc<T> {
                        type Response = super::ObjectStorage;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetObjectStorageRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_object_storage(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetObjectStorageSvc(inner);
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
                "/manager.Manager/ListBuckets" => {
                    #[allow(non_camel_case_types)]
                    struct ListBucketsSvc<T: Manager>(pub Arc<T>);
                    impl<
                        T: Manager,
                    > tonic::server::UnaryService<super::ListBucketsRequest>
                    for ListBucketsSvc<T> {
                        type Response = super::ListBucketsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListBucketsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).list_buckets(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListBucketsSvc(inner);
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
                "/manager.Manager/ListApplications" => {
                    #[allow(non_camel_case_types)]
                    struct ListApplicationsSvc<T: Manager>(pub Arc<T>);
                    impl<
                        T: Manager,
                    > tonic::server::UnaryService<super::ListApplicationsRequest>
                    for ListApplicationsSvc<T> {
                        type Response = super::ListApplicationsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListApplicationsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).list_applications(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListApplicationsSvc(inner);
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
                "/manager.Manager/KeepAlive" => {
                    #[allow(non_camel_case_types)]
                    struct KeepAliveSvc<T: Manager>(pub Arc<T>);
                    impl<
                        T: Manager,
                    > tonic::server::ClientStreamingService<super::KeepAliveRequest>
                    for KeepAliveSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::KeepAliveRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).keep_alive(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = KeepAliveSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.client_streaming(method, req).await;
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
    impl<T: Manager> Clone for ManagerServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Manager> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Manager> tonic::server::NamedService for ManagerServer<T> {
        const NAME: &'static str = "manager.Manager";
    }
}
