/// Peer metadata.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Peer {
    /// Peer id.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Pieces of peer.
    #[prost(message, repeated, tag = "2")]
    pub pieces: ::prost::alloc::vec::Vec<Piece>,
    /// Task info.
    #[prost(message, optional, tag = "3")]
    pub task: ::core::option::Option<Task>,
    /// Host info.
    #[prost(message, optional, tag = "4")]
    pub host: ::core::option::Option<Host>,
    /// Peer state.
    #[prost(string, tag = "5")]
    pub state: ::prost::alloc::string::String,
    /// Peer create time.
    #[prost(message, optional, tag = "6")]
    pub created_at: ::core::option::Option<::prost_types::Timestamp>,
    /// Peer update time.
    #[prost(message, optional, tag = "7")]
    pub updated_at: ::core::option::Option<::prost_types::Timestamp>,
}
/// Task metadata.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Task {
    /// Task id.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Host type.
    #[prost(string, tag = "2")]
    pub r#type: ::prost::alloc::string::String,
    /// Task size scope.
    #[prost(enumeration = "SizeScope", tag = "3")]
    pub size_scope: i32,
    /// Pieces of task.
    #[prost(message, repeated, tag = "4")]
    pub pieces: ::prost::alloc::vec::Vec<Piece>,
    /// Task state.
    #[prost(string, tag = "5")]
    pub state: ::prost::alloc::string::String,
    /// Task metadata.
    #[prost(message, optional, tag = "6")]
    pub metadata: ::core::option::Option<Metadata>,
    /// Task content length.
    #[prost(int64, tag = "7")]
    pub content_length: i64,
    /// Task peer count.
    #[prost(int32, tag = "8")]
    pub peer_count: i32,
    /// Task contains available peer.
    #[prost(bool, tag = "9")]
    pub has_available_peer: bool,
    /// Task create time.
    #[prost(message, optional, tag = "10")]
    pub created_at: ::core::option::Option<::prost_types::Timestamp>,
    /// Task update time.
    #[prost(message, optional, tag = "11")]
    pub updated_at: ::core::option::Option<::prost_types::Timestamp>,
}
/// Host metadata.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Host {
    /// Host id.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Host ipv4.
    #[prost(string, tag = "2")]
    pub ipv4: ::prost::alloc::string::String,
    /// Host ipv6.
    #[prost(string, tag = "3")]
    pub ipv6: ::prost::alloc::string::String,
    /// Peer hostname.
    #[prost(string, tag = "4")]
    pub hostname: ::prost::alloc::string::String,
    /// Port of grpc service.
    #[prost(int32, tag = "5")]
    pub port: i32,
    /// Port of download server.
    #[prost(int32, tag = "6")]
    pub download_port: i32,
    /// Security domain for network.
    #[prost(string, tag = "7")]
    pub security_domain: ::prost::alloc::string::String,
    /// Host location(area, country, province, city, etc.).
    #[prost(string, repeated, tag = "8")]
    pub location: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// IDC where the peer host is located.
    #[prost(string, tag = "9")]
    pub idc: ::prost::alloc::string::String,
    /// Network topology(switch, router, etc.).
    #[prost(string, repeated, tag = "10")]
    pub net_topology: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Range represents download range.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Range {
    /// Begin of range.
    #[prost(uint64, tag = "1")]
    pub begin: u64,
    /// End of range.
    #[prost(uint64, tag = "2")]
    pub end: u64,
}
/// Metadata represents metadata of task.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Metadata {
    /// Download url.
    #[prost(string, tag = "1")]
    pub url: ::prost::alloc::string::String,
    /// Digest of the pieces digest, for example md5:xxx or sha256:yyy.
    #[prost(string, tag = "2")]
    pub digest: ::prost::alloc::string::String,
    /// Range is url range of request.
    #[prost(message, optional, tag = "3")]
    pub range: ::core::option::Option<Range>,
    /// Task type.
    #[prost(enumeration = "TaskType", tag = "4")]
    pub r#type: i32,
    /// URL tag identifies different task for same url.
    #[prost(string, tag = "5")]
    pub tag: ::prost::alloc::string::String,
    /// Application of task.
    #[prost(string, tag = "6")]
    pub application: ::prost::alloc::string::String,
    /// Peer priority.
    #[prost(enumeration = "Priority", tag = "7")]
    pub priority: i32,
    /// Filter url used to generate task id.
    #[prost(string, repeated, tag = "8")]
    pub filters: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Task request headers.
    #[prost(map = "string, string", tag = "9")]
    pub header: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Task piece size.
    #[prost(int32, tag = "10")]
    pub piece_size: i32,
}
/// Piece represents information of piece.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Piece {
    /// Piece number.
    #[prost(uint32, tag = "1")]
    pub number: u32,
    /// Parent peer id.
    #[prost(string, tag = "2")]
    pub parent_id: ::prost::alloc::string::String,
    /// Piece offset.
    #[prost(uint64, tag = "3")]
    pub offset: u64,
    /// Piece size.
    #[prost(uint64, tag = "4")]
    pub size: u64,
    /// Digest of the piece data, for example md5:xxx or sha256:yyy.
    #[prost(string, tag = "5")]
    pub digest: ::prost::alloc::string::String,
    /// Traffic type.
    #[prost(enumeration = "TrafficType", tag = "6")]
    pub traffic_type: i32,
    /// Downloading piece costs time.
    #[prost(message, optional, tag = "7")]
    pub cost: ::core::option::Option<::prost_types::Duration>,
    /// Piece create time.
    #[prost(message, optional, tag = "8")]
    pub created_at: ::core::option::Option<::prost_types::Timestamp>,
}
/// ExtendAttribute represents extend of attribution.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtendAttribute {
    /// Task response header, eg: HTTP Response Header
    #[prost(map = "string, string", tag = "1")]
    pub header: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Task response code, eg: HTTP Status Code
    #[prost(int32, tag = "2")]
    pub status_code: i32,
    /// Task response status, eg: HTTP Status
    #[prost(string, tag = "3")]
    pub status: ::prost::alloc::string::String,
}
/// SizeScope represents size scope of task.
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
