/// Peer metadata.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Peer {
    /// Peer id.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Range is url range of request.
    #[prost(message, optional, tag = "2")]
    pub range: ::core::option::Option<Range>,
    /// Peer priority.
    #[prost(enumeration = "Priority", tag = "3")]
    pub priority: i32,
    /// Pieces of peer.
    #[prost(message, repeated, tag = "4")]
    pub pieces: ::prost::alloc::vec::Vec<Piece>,
    /// Peer downloads costs time.
    #[prost(message, optional, tag = "5")]
    pub cost: ::core::option::Option<::prost_wkt_types::Duration>,
    /// Peer state.
    #[prost(string, tag = "6")]
    pub state: ::prost::alloc::string::String,
    /// Task info.
    #[prost(message, optional, tag = "7")]
    pub task: ::core::option::Option<Task>,
    /// Host info.
    #[prost(message, optional, tag = "8")]
    pub host: ::core::option::Option<Host>,
    /// NeedBackToSource needs downloaded from source.
    #[prost(bool, tag = "9")]
    pub need_back_to_source: bool,
    /// Peer create time.
    #[prost(message, optional, tag = "10")]
    pub created_at: ::core::option::Option<::prost_wkt_types::Timestamp>,
    /// Peer update time.
    #[prost(message, optional, tag = "11")]
    pub updated_at: ::core::option::Option<::prost_wkt_types::Timestamp>,
}
/// Task metadata.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Task {
    /// Task id.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Task type.
    #[prost(enumeration = "TaskType", tag = "2")]
    pub r#type: i32,
    /// Download url.
    #[prost(string, tag = "3")]
    pub url: ::prost::alloc::string::String,
    /// Digest of the task digest, for example md5:xxx or sha256:yyy.
    #[prost(string, optional, tag = "4")]
    pub digest: ::core::option::Option<::prost::alloc::string::String>,
    /// URL tag identifies different task for same url.
    #[prost(string, optional, tag = "5")]
    pub tag: ::core::option::Option<::prost::alloc::string::String>,
    /// Application of task.
    #[prost(string, optional, tag = "6")]
    pub application: ::core::option::Option<::prost::alloc::string::String>,
    /// Filtered query params to generate the task id.
    /// When filter is ["Signature", "Expires", "ns"], for example:
    /// <http://example.com/xyz?Expires=e1&Signature=s1&ns=docker.io> and <http://example.com/xyz?Expires=e2&Signature=s2&ns=docker.io>
    /// will generate the same task id.
    /// Default value includes the filtered query params of s3, gcs, oss, obs, cos.
    #[prost(string, repeated, tag = "7")]
    pub filtered_query_params: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Task request headers.
    #[prost(map = "string, string", tag = "8")]
    pub request_header: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Task piece length.
    #[prost(uint64, tag = "9")]
    pub piece_length: u64,
    /// Task content length.
    #[prost(uint64, tag = "10")]
    pub content_length: u64,
    /// Task piece count.
    #[prost(uint32, tag = "11")]
    pub piece_count: u32,
    /// Task size scope.
    #[prost(enumeration = "SizeScope", tag = "12")]
    pub size_scope: i32,
    /// Pieces of task.
    #[prost(message, repeated, tag = "13")]
    pub pieces: ::prost::alloc::vec::Vec<Piece>,
    /// Task state.
    #[prost(string, tag = "14")]
    pub state: ::prost::alloc::string::String,
    /// Task peer count.
    #[prost(uint32, tag = "15")]
    pub peer_count: u32,
    /// Task contains available peer.
    #[prost(bool, tag = "16")]
    pub has_available_peer: bool,
    /// Task create time.
    #[prost(message, optional, tag = "17")]
    pub created_at: ::core::option::Option<::prost_wkt_types::Timestamp>,
    /// Task update time.
    #[prost(message, optional, tag = "18")]
    pub updated_at: ::core::option::Option<::prost_wkt_types::Timestamp>,
}
/// Host metadata.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Host {
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
    /// ID of the cluster to which the host belongs.
    #[prost(uint64, tag = "17")]
    pub scheduler_cluster_id: u64,
}
/// CPU Stat.
#[derive(serde::Serialize, serde::Deserialize)]
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
#[derive(serde::Serialize, serde::Deserialize)]
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
#[derive(serde::Serialize, serde::Deserialize)]
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
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Network {
    /// Return count of tcp connections opened and status is ESTABLISHED.
    #[prost(uint32, tag = "1")]
    pub tcp_connection_count: u32,
    /// Return count of upload tcp connections opened and status is ESTABLISHED.
    #[prost(uint32, tag = "2")]
    pub upload_tcp_connection_count: u32,
    /// Location path(area|country|province|city|...).
    #[prost(string, optional, tag = "3")]
    pub location: ::core::option::Option<::prost::alloc::string::String>,
    /// IDC where the peer host is located
    #[prost(string, optional, tag = "4")]
    pub idc: ::core::option::Option<::prost::alloc::string::String>,
}
/// Disk Stat.
#[derive(serde::Serialize, serde::Deserialize)]
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
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Build {
    /// Git version.
    #[prost(string, tag = "1")]
    pub git_version: ::prost::alloc::string::String,
    /// Git commit.
    #[prost(string, optional, tag = "2")]
    pub git_commit: ::core::option::Option<::prost::alloc::string::String>,
    /// Golang version.
    #[prost(string, optional, tag = "3")]
    pub go_version: ::core::option::Option<::prost::alloc::string::String>,
    /// Rust version.
    #[prost(string, optional, tag = "4")]
    pub rust_version: ::core::option::Option<::prost::alloc::string::String>,
    /// Build platform.
    #[prost(string, optional, tag = "5")]
    pub platform: ::core::option::Option<::prost::alloc::string::String>,
}
/// Download information.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Download {
    /// Download url.
    #[prost(string, tag = "1")]
    pub url: ::prost::alloc::string::String,
    /// Digest of the task digest, for example md5:xxx or sha256:yyy.
    #[prost(string, optional, tag = "2")]
    pub digest: ::core::option::Option<::prost::alloc::string::String>,
    /// Range is url range of request. If protocol is http, range
    /// will set in request header. If protocol is others, range
    /// will set in range field.
    #[prost(message, optional, tag = "3")]
    pub range: ::core::option::Option<Range>,
    /// Task type.
    #[prost(enumeration = "TaskType", tag = "4")]
    pub r#type: i32,
    /// URL tag identifies different task for same url.
    #[prost(string, optional, tag = "5")]
    pub tag: ::core::option::Option<::prost::alloc::string::String>,
    /// Application of task.
    #[prost(string, optional, tag = "6")]
    pub application: ::core::option::Option<::prost::alloc::string::String>,
    /// Peer priority.
    #[prost(enumeration = "Priority", tag = "7")]
    pub priority: i32,
    /// Filtered query params to generate the task id.
    /// When filter is ["Signature", "Expires", "ns"], for example:
    /// <http://example.com/xyz?Expires=e1&Signature=s1&ns=docker.io> and <http://example.com/xyz?Expires=e2&Signature=s2&ns=docker.io>
    /// will generate the same task id.
    /// Default value includes the filtered query params of s3, gcs, oss, obs, cos.
    #[prost(string, repeated, tag = "8")]
    pub filtered_query_params: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Task request headers.
    #[prost(map = "string, string", tag = "9")]
    pub request_header: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Task piece length.
    #[prost(uint64, tag = "10")]
    pub piece_length: u64,
    /// File path to be exported.
    #[prost(string, optional, tag = "11")]
    pub output_path: ::core::option::Option<::prost::alloc::string::String>,
    /// Download timeout.
    #[prost(message, optional, tag = "12")]
    pub timeout: ::core::option::Option<::prost_wkt_types::Duration>,
    /// NeedBackToSource needs downloaded from source.
    #[prost(bool, tag = "13")]
    pub need_back_to_source: bool,
    /// certificate_chain is the client certs with DER format for the backend client to download back-to-source.
    #[prost(bytes = "vec", repeated, tag = "14")]
    pub certificate_chain: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// Range represents download range.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Range {
    /// Start of range.
    #[prost(uint64, tag = "1")]
    pub start: u64,
    /// Length of range.
    #[prost(uint64, tag = "2")]
    pub length: u64,
}
/// Piece represents information of piece.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Piece {
    /// Piece number.
    #[prost(uint32, tag = "1")]
    pub number: u32,
    /// Parent peer id.
    #[prost(string, optional, tag = "2")]
    pub parent_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Piece offset.
    #[prost(uint64, tag = "3")]
    pub offset: u64,
    /// Piece length.
    #[prost(uint64, tag = "4")]
    pub length: u64,
    /// Digest of the piece data, for example md5:xxx or sha256:yyy.
    #[prost(string, tag = "5")]
    pub digest: ::prost::alloc::string::String,
    /// Piece content.
    #[prost(bytes = "vec", optional, tag = "6")]
    pub content: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    /// Traffic type.
    #[prost(enumeration = "TrafficType", optional, tag = "7")]
    pub traffic_type: ::core::option::Option<i32>,
    /// Downloading piece costs time.
    #[prost(message, optional, tag = "8")]
    pub cost: ::core::option::Option<::prost_wkt_types::Duration>,
    /// Piece create time.
    #[prost(message, optional, tag = "9")]
    pub created_at: ::core::option::Option<::prost_wkt_types::Timestamp>,
}
/// SizeScope represents size scope of task.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SizeScope {
    /// size > one piece size.
    Normal = 0,
    /// 128 byte < size <= one piece size and be plain type.
    Small = 1,
    /// size <= 128 byte and be plain type.
    Tiny = 2,
    /// size == 0 byte and be plain type.
    Empty = 3,
}
impl SizeScope {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SizeScope::Normal => "NORMAL",
            SizeScope::Small => "SMALL",
            SizeScope::Tiny => "TINY",
            SizeScope::Empty => "EMPTY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NORMAL" => Some(Self::Normal),
            "SMALL" => Some(Self::Small),
            "TINY" => Some(Self::Tiny),
            "EMPTY" => Some(Self::Empty),
            _ => None,
        }
    }
}
/// TaskType represents type of task.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TaskType {
    /// DFDAEMON is dfdeamon type of task,
    /// dfdeamon task is a normal p2p task.
    Dfdaemon = 0,
    /// DFCACHE is dfcache type of task,
    /// dfcache task is a cache task, and the task url is fake url.
    /// It can only be used for caching and cannot be downloaded back to source.
    Dfcache = 1,
    /// DFSTORE is dfstore type of task,
    /// dfstore task is a persistent task in backend.
    Dfstore = 2,
}
impl TaskType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TaskType::Dfdaemon => "DFDAEMON",
            TaskType::Dfcache => "DFCACHE",
            TaskType::Dfstore => "DFSTORE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DFDAEMON" => Some(Self::Dfdaemon),
            "DFCACHE" => Some(Self::Dfcache),
            "DFSTORE" => Some(Self::Dfstore),
            _ => None,
        }
    }
}
/// TrafficType represents type of traffic.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TrafficType {
    /// BACK_TO_SOURCE is to download traffic from the source.
    BackToSource = 0,
    /// REMOTE_PEER is to download traffic from the remote peer.
    RemotePeer = 1,
    /// LOCAL_PEER is to download traffic from the local peer.
    LocalPeer = 2,
}
impl TrafficType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TrafficType::BackToSource => "BACK_TO_SOURCE",
            TrafficType::RemotePeer => "REMOTE_PEER",
            TrafficType::LocalPeer => "LOCAL_PEER",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "BACK_TO_SOURCE" => Some(Self::BackToSource),
            "REMOTE_PEER" => Some(Self::RemotePeer),
            "LOCAL_PEER" => Some(Self::LocalPeer),
            _ => None,
        }
    }
}
/// Priority represents priority of application.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Priority {
    /// LEVEL0 has no special meaning for scheduler.
    Level0 = 0,
    /// LEVEL1 represents the download task is forbidden,
    /// and an error code is returned during the registration.
    Level1 = 1,
    /// LEVEL2 represents when the task is downloaded for the first time,
    /// allow peers to download from the other peers,
    /// but not back-to-source. When the task is not downloaded for
    /// the first time, it is scheduled normally.
    Level2 = 2,
    /// LEVEL3 represents when the task is downloaded for the first time,
    /// the normal peer is first to download back-to-source.
    /// When the task is not downloaded for the first time, it is scheduled normally.
    Level3 = 3,
    /// LEVEL4 represents when the task is downloaded for the first time,
    /// the weak peer is first triggered to back-to-source.
    /// When the task is not downloaded for the first time, it is scheduled normally.
    Level4 = 4,
    /// LEVEL5 represents when the task is downloaded for the first time,
    /// the strong peer is first triggered to back-to-source.
    /// When the task is not downloaded for the first time, it is scheduled normally.
    Level5 = 5,
    /// LEVEL6 represents when the task is downloaded for the first time,
    /// the super peer is first triggered to back-to-source.
    /// When the task is not downloaded for the first time, it is scheduled normally.
    Level6 = 6,
}
impl Priority {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Priority::Level0 => "LEVEL0",
            Priority::Level1 => "LEVEL1",
            Priority::Level2 => "LEVEL2",
            Priority::Level3 => "LEVEL3",
            Priority::Level4 => "LEVEL4",
            Priority::Level5 => "LEVEL5",
            Priority::Level6 => "LEVEL6",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LEVEL0" => Some(Self::Level0),
            "LEVEL1" => Some(Self::Level1),
            "LEVEL2" => Some(Self::Level2),
            "LEVEL3" => Some(Self::Level3),
            "LEVEL4" => Some(Self::Level4),
            "LEVEL5" => Some(Self::Level5),
            "LEVEL6" => Some(Self::Level6),
            _ => None,
        }
    }
}
