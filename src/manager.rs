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
    /// Cluster scopes.
    #[prost(bytes = "vec", tag = "5")]
    pub scopes: ::prost::alloc::vec::Vec<u8>,
    /// Security group to which the seed peer cluster belongs.
    #[prost(message, optional, tag = "6")]
    pub security_group: ::core::option::Option<SecurityGroup>,
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
    #[prost(string, tag = "5")]
    pub idc: ::prost::alloc::string::String,
    /// Seed peer network topology.
    #[prost(string, tag = "6")]
    pub net_topology: ::prost::alloc::string::String,
    /// Seed peer location.
    #[prost(string, tag = "7")]
    pub location: ::prost::alloc::string::String,
    /// Seed peer ip.
    #[prost(string, tag = "8")]
    pub ip: ::prost::alloc::string::String,
    /// Seed peer grpc port.
    #[prost(int32, tag = "9")]
    pub port: i32,
    /// Seed peer download port.
    #[prost(int32, tag = "10")]
    pub download_port: i32,
    /// Seed peer state.
    #[prost(string, tag = "11")]
    pub state: ::prost::alloc::string::String,
    /// ID of the cluster to which the seed peer belongs.
    #[prost(uint64, tag = "12")]
    pub seed_peer_cluster_id: u64,
    /// Cluster to which the seed peer belongs.
    #[prost(message, optional, tag = "13")]
    pub seed_peer_cluster: ::core::option::Option<SeedPeerCluster>,
    /// Schedulers included in seed peer.
    #[prost(message, repeated, tag = "14")]
    pub schedulers: ::prost::alloc::vec::Vec<Scheduler>,
    /// Seed peer object storage port.
    #[prost(int32, tag = "15")]
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
    /// Seed peer network topology.
    #[prost(string, tag = "5")]
    pub net_topology: ::prost::alloc::string::String,
    /// Seed peer location.
    #[prost(string, tag = "6")]
    pub location: ::prost::alloc::string::String,
    /// Seed peer ip.
    #[prost(string, tag = "7")]
    pub ip: ::prost::alloc::string::String,
    /// Seed peer port.
    #[prost(int32, tag = "8")]
    pub port: i32,
    /// Seed peer download port.
    #[prost(int32, tag = "9")]
    pub download_port: i32,
    /// ID of the cluster to which the seed peer belongs.
    #[prost(uint64, tag = "10")]
    pub seed_peer_cluster_id: u64,
    /// Seed peer object storage port.
    #[prost(int32, tag = "11")]
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
    /// Deprecated: Do not use.
    #[prost(string, tag = "3")]
    pub vips: ::prost::alloc::string::String,
    /// Scheduler idc.
    #[prost(string, tag = "4")]
    pub idc: ::prost::alloc::string::String,
    /// Scheduler location.
    #[prost(string, tag = "5")]
    pub location: ::prost::alloc::string::String,
    /// Deprecated: Use net_topology instead.
    #[prost(bytes = "vec", tag = "6")]
    pub net_config: ::prost::alloc::vec::Vec<u8>,
    /// Scheduler ip.
    #[prost(string, tag = "7")]
    pub ip: ::prost::alloc::string::String,
    /// Scheduler grpc port.
    #[prost(int32, tag = "8")]
    pub port: i32,
    /// Scheduler state.
    #[prost(string, tag = "9")]
    pub state: ::prost::alloc::string::String,
    /// ID of the cluster to which the scheduler belongs.
    #[prost(uint64, tag = "10")]
    pub scheduler_cluster_id: u64,
    /// Cluster to which the scheduler belongs.
    #[prost(message, optional, tag = "11")]
    pub scheduler_cluster: ::core::option::Option<SchedulerCluster>,
    /// Seed peers to which the scheduler belongs.
    #[prost(message, repeated, tag = "12")]
    pub seed_peers: ::prost::alloc::vec::Vec<SeedPeer>,
    /// Scheduler network topology.
    #[prost(string, tag = "13")]
    pub net_topology: ::prost::alloc::string::String,
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
    /// Deprecated: Do not use.
    #[prost(string, tag = "4")]
    pub vips: ::prost::alloc::string::String,
    /// Scheduler idc.
    #[prost(string, tag = "5")]
    pub idc: ::prost::alloc::string::String,
    /// Scheduler location.
    #[prost(string, tag = "6")]
    pub location: ::prost::alloc::string::String,
    /// Deprecated: Use net_topology instead.
    #[prost(bytes = "vec", tag = "7")]
    pub net_config: ::prost::alloc::vec::Vec<u8>,
    /// Scheduler ip.
    #[prost(string, tag = "8")]
    pub ip: ::prost::alloc::string::String,
    /// Scheduler port.
    #[prost(int32, tag = "9")]
    pub port: i32,
    /// Scheduler network topology.
    #[prost(string, tag = "10")]
    pub net_topology: ::prost::alloc::string::String,
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
    /// Object storage name of type.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Storage region.
    #[prost(string, tag = "2")]
    pub region: ::prost::alloc::string::String,
    /// Datacenter endpoint.
    #[prost(string, tag = "3")]
    pub endpoint: ::prost::alloc::string::String,
    /// Access key id.
    #[prost(string, tag = "4")]
    pub access_key: ::prost::alloc::string::String,
    /// Access key secret.
    #[prost(string, tag = "5")]
    pub secret_key: ::prost::alloc::string::String,
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
/// Model represents information of model.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Model {
    /// Model id.
    #[prost(string, tag = "1")]
    pub model_id: ::prost::alloc::string::String,
    /// Model name.
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// Model version id.
    #[prost(string, tag = "3")]
    pub version_id: ::prost::alloc::string::String,
    /// Scheduler id.
    #[prost(uint64, tag = "4")]
    pub scheduler_id: u64,
    /// Scheduler hostname.
    #[prost(string, tag = "5")]
    pub host_name: ::prost::alloc::string::String,
    /// Scheduler ip.
    #[prost(string, tag = "6")]
    pub ip: ::prost::alloc::string::String,
    /// Model create time.
    #[prost(message, optional, tag = "7")]
    pub created_at: ::core::option::Option<::prost_types::Timestamp>,
    /// Model update time.
    #[prost(message, optional, tag = "8")]
    pub updated_at: ::core::option::Option<::prost_types::Timestamp>,
}
/// ListModelsRequest represents request of ListModels.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListModelsRequest {
    /// Scheduler id.
    #[prost(uint64, tag = "1")]
    pub scheduler_id: u64,
}
/// ListModelsResponse represents response of ListModels.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListModelsResponse {
    /// Model informations.
    #[prost(message, repeated, tag = "1")]
    pub models: ::prost::alloc::vec::Vec<Model>,
}
/// GetModelRequest represents request of GetModel.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetModelRequest {
    /// Scheduler id.
    #[prost(uint64, tag = "1")]
    pub scheduler_id: u64,
    /// Model id.
    #[prost(string, tag = "2")]
    pub model_id: ::prost::alloc::string::String,
}
/// CreateModelRequest represents request of CreateModel.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateModelRequest {
    /// Model id.
    #[prost(string, tag = "1")]
    pub model_id: ::prost::alloc::string::String,
    /// Model name.
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// Model version id.
    #[prost(string, tag = "3")]
    pub version_id: ::prost::alloc::string::String,
    /// Scheduler id.
    #[prost(uint64, tag = "4")]
    pub scheduler_id: u64,
    /// Scheduler hostname.
    #[prost(string, tag = "5")]
    pub host_name: ::prost::alloc::string::String,
    /// Scheduler ip.
    #[prost(string, tag = "6")]
    pub ip: ::prost::alloc::string::String,
}
/// UpdateModelRequest represents request of UpdateModel.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateModelRequest {
    /// Model id.
    #[prost(string, tag = "1")]
    pub model_id: ::prost::alloc::string::String,
    /// Model name.
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// Model version id.
    #[prost(string, tag = "3")]
    pub version_id: ::prost::alloc::string::String,
    /// Scheduler id.
    #[prost(uint64, tag = "4")]
    pub scheduler_id: u64,
    /// Scheduler hostname.
    #[prost(string, tag = "5")]
    pub host_name: ::prost::alloc::string::String,
    /// Scheduler ip.
    #[prost(string, tag = "6")]
    pub ip: ::prost::alloc::string::String,
}
/// DeleteModelRequest represents request of DeleteModel.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteModelRequest {
    /// Scheduler id.
    #[prost(uint64, tag = "1")]
    pub scheduler_id: u64,
    /// Model id.
    #[prost(string, tag = "2")]
    pub model_id: ::prost::alloc::string::String,
}
/// ModelVersion represents information of model version.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModelVersion {
    /// Model version id.
    #[prost(string, tag = "1")]
    pub version_id: ::prost::alloc::string::String,
    /// Model version data.
    #[prost(bytes = "vec", tag = "2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    /// Model version mae.
    #[prost(double, tag = "3")]
    pub mae: f64,
    /// Model version mse.
    #[prost(double, tag = "4")]
    pub mse: f64,
    /// Model version rmse.
    #[prost(double, tag = "5")]
    pub rmse: f64,
    /// Model version r^2.
    #[prost(double, tag = "6")]
    pub r2: f64,
    /// Model create time.
    #[prost(message, optional, tag = "7")]
    pub created_at: ::core::option::Option<::prost_types::Timestamp>,
    /// Model update time.
    #[prost(message, optional, tag = "8")]
    pub updated_at: ::core::option::Option<::prost_types::Timestamp>,
}
/// ListModelVersionsRequest represents request of ListModelVersions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListModelVersionsRequest {
    /// Scheduler id.
    #[prost(uint64, tag = "1")]
    pub scheduler_id: u64,
    /// Model id.
    #[prost(string, tag = "2")]
    pub model_id: ::prost::alloc::string::String,
}
/// ListModelVersionsResponse represents response of ListModelVersions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListModelVersionsResponse {
    /// Model version informations.
    #[prost(message, repeated, tag = "1")]
    pub model_versions: ::prost::alloc::vec::Vec<ModelVersion>,
}
/// GetModelVersionRequest represents request of GetModelVersion.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetModelVersionRequest {
    /// Scheduler id.
    #[prost(uint64, tag = "1")]
    pub scheduler_id: u64,
    /// Model id.
    #[prost(string, tag = "2")]
    pub model_id: ::prost::alloc::string::String,
    /// Model version id.
    #[prost(string, tag = "3")]
    pub version_id: ::prost::alloc::string::String,
}
/// CreateModelVersionRequest represents request of CreateModelVersion.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateModelVersionRequest {
    /// Scheduler id.
    #[prost(uint64, tag = "1")]
    pub scheduler_id: u64,
    /// Model id.
    #[prost(string, tag = "2")]
    pub model_id: ::prost::alloc::string::String,
    /// Model version data.
    #[prost(bytes = "vec", tag = "3")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    /// Model version mae.
    #[prost(double, tag = "4")]
    pub mae: f64,
    /// Model version mse.
    #[prost(double, tag = "5")]
    pub mse: f64,
    /// Model version rmse.
    #[prost(double, tag = "6")]
    pub rmse: f64,
    /// Model version r^2.
    #[prost(double, tag = "7")]
    pub r2: f64,
}
/// UpdateModelVersionRequest represents request of UpdateModelVersion.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateModelVersionRequest {
    /// Model version id.
    #[prost(string, tag = "1")]
    pub version_id: ::prost::alloc::string::String,
    /// Scheduler id.
    #[prost(uint64, tag = "2")]
    pub scheduler_id: u64,
    /// Model id.
    #[prost(string, tag = "3")]
    pub model_id: ::prost::alloc::string::String,
    /// Model version data.
    #[prost(bytes = "vec", tag = "4")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    /// Model version mae.
    #[prost(double, tag = "5")]
    pub mae: f64,
    /// Model version mse.
    #[prost(double, tag = "6")]
    pub mse: f64,
    /// Model version rmse.
    #[prost(double, tag = "7")]
    pub rmse: f64,
    /// Model version r^2.
    #[prost(double, tag = "8")]
    pub r2: f64,
}
/// DeleteModelVersionRequest represents request of DeleteModelVersion.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteModelVersionRequest {
    /// Scheduler id.
    #[prost(uint64, tag = "1")]
    pub scheduler_id: u64,
    /// Model id.
    #[prost(string, tag = "2")]
    pub model_id: ::prost::alloc::string::String,
    /// Model version id.
    #[prost(string, tag = "3")]
    pub version_id: ::prost::alloc::string::String,
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
        /// List models information.
        pub async fn list_models(
            &mut self,
            request: impl tonic::IntoRequest<super::ListModelsRequest>,
        ) -> Result<tonic::Response<super::ListModelsResponse>, tonic::Status> {
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
                "/manager.Manager/ListModels",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Get model information.
        pub async fn get_model(
            &mut self,
            request: impl tonic::IntoRequest<super::GetModelRequest>,
        ) -> Result<tonic::Response<super::Model>, tonic::Status> {
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
            let path = http::uri::PathAndQuery::from_static("/manager.Manager/GetModel");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Create model information.
        pub async fn create_model(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateModelRequest>,
        ) -> Result<tonic::Response<super::Model>, tonic::Status> {
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
                "/manager.Manager/CreateModel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Update model information.
        pub async fn update_model(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateModelRequest>,
        ) -> Result<tonic::Response<super::Model>, tonic::Status> {
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
                "/manager.Manager/UpdateModel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Delete model information.
        pub async fn delete_model(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteModelRequest>,
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
                "/manager.Manager/DeleteModel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// List model versions information.
        pub async fn list_model_versions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListModelVersionsRequest>,
        ) -> Result<tonic::Response<super::ListModelVersionsResponse>, tonic::Status> {
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
                "/manager.Manager/ListModelVersions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Get model version information.
        pub async fn get_model_version(
            &mut self,
            request: impl tonic::IntoRequest<super::GetModelVersionRequest>,
        ) -> Result<tonic::Response<super::ModelVersion>, tonic::Status> {
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
                "/manager.Manager/GetModelVersion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Create model version information.
        pub async fn create_model_version(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateModelVersionRequest>,
        ) -> Result<tonic::Response<super::ModelVersion>, tonic::Status> {
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
                "/manager.Manager/CreateModelVersion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Update model version information.
        pub async fn update_model_version(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateModelVersionRequest>,
        ) -> Result<tonic::Response<super::ModelVersion>, tonic::Status> {
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
                "/manager.Manager/UpdateModelVersion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Delete model version information.
        pub async fn delete_model_version(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteModelVersionRequest>,
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
                "/manager.Manager/DeleteModelVersion",
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
        /// List models information.
        async fn list_models(
            &self,
            request: tonic::Request<super::ListModelsRequest>,
        ) -> Result<tonic::Response<super::ListModelsResponse>, tonic::Status>;
        /// Get model information.
        async fn get_model(
            &self,
            request: tonic::Request<super::GetModelRequest>,
        ) -> Result<tonic::Response<super::Model>, tonic::Status>;
        /// Create model information.
        async fn create_model(
            &self,
            request: tonic::Request<super::CreateModelRequest>,
        ) -> Result<tonic::Response<super::Model>, tonic::Status>;
        /// Update model information.
        async fn update_model(
            &self,
            request: tonic::Request<super::UpdateModelRequest>,
        ) -> Result<tonic::Response<super::Model>, tonic::Status>;
        /// Delete model information.
        async fn delete_model(
            &self,
            request: tonic::Request<super::DeleteModelRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        /// List model versions information.
        async fn list_model_versions(
            &self,
            request: tonic::Request<super::ListModelVersionsRequest>,
        ) -> Result<tonic::Response<super::ListModelVersionsResponse>, tonic::Status>;
        /// Get model version information.
        async fn get_model_version(
            &self,
            request: tonic::Request<super::GetModelVersionRequest>,
        ) -> Result<tonic::Response<super::ModelVersion>, tonic::Status>;
        /// Create model version information.
        async fn create_model_version(
            &self,
            request: tonic::Request<super::CreateModelVersionRequest>,
        ) -> Result<tonic::Response<super::ModelVersion>, tonic::Status>;
        /// Update model version information.
        async fn update_model_version(
            &self,
            request: tonic::Request<super::UpdateModelVersionRequest>,
        ) -> Result<tonic::Response<super::ModelVersion>, tonic::Status>;
        /// Delete model version information.
        async fn delete_model_version(
            &self,
            request: tonic::Request<super::DeleteModelVersionRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
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
                "/manager.Manager/ListModels" => {
                    #[allow(non_camel_case_types)]
                    struct ListModelsSvc<T: Manager>(pub Arc<T>);
                    impl<
                        T: Manager,
                    > tonic::server::UnaryService<super::ListModelsRequest>
                    for ListModelsSvc<T> {
                        type Response = super::ListModelsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListModelsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_models(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListModelsSvc(inner);
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
                "/manager.Manager/GetModel" => {
                    #[allow(non_camel_case_types)]
                    struct GetModelSvc<T: Manager>(pub Arc<T>);
                    impl<T: Manager> tonic::server::UnaryService<super::GetModelRequest>
                    for GetModelSvc<T> {
                        type Response = super::Model;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetModelRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_model(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetModelSvc(inner);
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
                "/manager.Manager/CreateModel" => {
                    #[allow(non_camel_case_types)]
                    struct CreateModelSvc<T: Manager>(pub Arc<T>);
                    impl<
                        T: Manager,
                    > tonic::server::UnaryService<super::CreateModelRequest>
                    for CreateModelSvc<T> {
                        type Response = super::Model;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateModelRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).create_model(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateModelSvc(inner);
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
                "/manager.Manager/UpdateModel" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateModelSvc<T: Manager>(pub Arc<T>);
                    impl<
                        T: Manager,
                    > tonic::server::UnaryService<super::UpdateModelRequest>
                    for UpdateModelSvc<T> {
                        type Response = super::Model;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateModelRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).update_model(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateModelSvc(inner);
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
                "/manager.Manager/DeleteModel" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteModelSvc<T: Manager>(pub Arc<T>);
                    impl<
                        T: Manager,
                    > tonic::server::UnaryService<super::DeleteModelRequest>
                    for DeleteModelSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteModelRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).delete_model(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteModelSvc(inner);
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
                "/manager.Manager/ListModelVersions" => {
                    #[allow(non_camel_case_types)]
                    struct ListModelVersionsSvc<T: Manager>(pub Arc<T>);
                    impl<
                        T: Manager,
                    > tonic::server::UnaryService<super::ListModelVersionsRequest>
                    for ListModelVersionsSvc<T> {
                        type Response = super::ListModelVersionsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListModelVersionsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).list_model_versions(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListModelVersionsSvc(inner);
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
                "/manager.Manager/GetModelVersion" => {
                    #[allow(non_camel_case_types)]
                    struct GetModelVersionSvc<T: Manager>(pub Arc<T>);
                    impl<
                        T: Manager,
                    > tonic::server::UnaryService<super::GetModelVersionRequest>
                    for GetModelVersionSvc<T> {
                        type Response = super::ModelVersion;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetModelVersionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_model_version(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetModelVersionSvc(inner);
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
                "/manager.Manager/CreateModelVersion" => {
                    #[allow(non_camel_case_types)]
                    struct CreateModelVersionSvc<T: Manager>(pub Arc<T>);
                    impl<
                        T: Manager,
                    > tonic::server::UnaryService<super::CreateModelVersionRequest>
                    for CreateModelVersionSvc<T> {
                        type Response = super::ModelVersion;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateModelVersionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).create_model_version(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateModelVersionSvc(inner);
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
                "/manager.Manager/UpdateModelVersion" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateModelVersionSvc<T: Manager>(pub Arc<T>);
                    impl<
                        T: Manager,
                    > tonic::server::UnaryService<super::UpdateModelVersionRequest>
                    for UpdateModelVersionSvc<T> {
                        type Response = super::ModelVersion;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateModelVersionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).update_model_version(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateModelVersionSvc(inner);
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
                "/manager.Manager/DeleteModelVersion" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteModelVersionSvc<T: Manager>(pub Arc<T>);
                    impl<
                        T: Manager,
                    > tonic::server::UnaryService<super::DeleteModelVersionRequest>
                    for DeleteModelVersionSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteModelVersionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).delete_model_version(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteModelVersionSvc(inner);
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
