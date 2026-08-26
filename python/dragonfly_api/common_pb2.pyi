import datetime

from google.protobuf import duration_pb2 as _duration_pb2
from google.protobuf import timestamp_pb2 as _timestamp_pb2
from google.protobuf.internal import containers as _containers
from google.protobuf.internal import enum_type_wrapper as _enum_type_wrapper
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from collections.abc import Iterable as _Iterable, Mapping as _Mapping
from typing import ClassVar as _ClassVar, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class SizeScope(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = ()
    NORMAL: _ClassVar[SizeScope]
    SMALL: _ClassVar[SizeScope]
    TINY: _ClassVar[SizeScope]
    EMPTY: _ClassVar[SizeScope]

class TaskType(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = ()
    STANDARD: _ClassVar[TaskType]
    PERSISTENT: _ClassVar[TaskType]
    PERSISTENT_CACHE: _ClassVar[TaskType]
    CACHE: _ClassVar[TaskType]

class TrafficType(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = ()
    BACK_TO_SOURCE: _ClassVar[TrafficType]
    REMOTE_PEER: _ClassVar[TrafficType]
    LOCAL_PEER: _ClassVar[TrafficType]

class Priority(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = ()
    LEVEL0: _ClassVar[Priority]
    LEVEL1: _ClassVar[Priority]
    LEVEL2: _ClassVar[Priority]
    LEVEL3: _ClassVar[Priority]
    LEVEL4: _ClassVar[Priority]
    LEVEL5: _ClassVar[Priority]
    LEVEL6: _ClassVar[Priority]
NORMAL: SizeScope
SMALL: SizeScope
TINY: SizeScope
EMPTY: SizeScope
STANDARD: TaskType
PERSISTENT: TaskType
PERSISTENT_CACHE: TaskType
CACHE: TaskType
BACK_TO_SOURCE: TrafficType
REMOTE_PEER: TrafficType
LOCAL_PEER: TrafficType
LEVEL0: Priority
LEVEL1: Priority
LEVEL2: Priority
LEVEL3: Priority
LEVEL4: Priority
LEVEL5: Priority
LEVEL6: Priority

class Peer(_message.Message):
    __slots__ = ("id", "range", "priority", "pieces", "cost", "state", "task", "host", "need_back_to_source", "created_at", "updated_at", "concurrent_piece_count")
    ID_FIELD_NUMBER: _ClassVar[int]
    RANGE_FIELD_NUMBER: _ClassVar[int]
    PRIORITY_FIELD_NUMBER: _ClassVar[int]
    PIECES_FIELD_NUMBER: _ClassVar[int]
    COST_FIELD_NUMBER: _ClassVar[int]
    STATE_FIELD_NUMBER: _ClassVar[int]
    TASK_FIELD_NUMBER: _ClassVar[int]
    HOST_FIELD_NUMBER: _ClassVar[int]
    NEED_BACK_TO_SOURCE_FIELD_NUMBER: _ClassVar[int]
    CREATED_AT_FIELD_NUMBER: _ClassVar[int]
    UPDATED_AT_FIELD_NUMBER: _ClassVar[int]
    CONCURRENT_PIECE_COUNT_FIELD_NUMBER: _ClassVar[int]
    id: str
    range: Range
    priority: Priority
    pieces: _containers.RepeatedCompositeFieldContainer[Piece]
    cost: _duration_pb2.Duration
    state: str
    task: Task
    host: Host
    need_back_to_source: bool
    created_at: _timestamp_pb2.Timestamp
    updated_at: _timestamp_pb2.Timestamp
    concurrent_piece_count: int
    def __init__(self, id: _Optional[str] = ..., range: _Optional[_Union[Range, _Mapping]] = ..., priority: _Optional[_Union[Priority, str]] = ..., pieces: _Optional[_Iterable[_Union[Piece, _Mapping]]] = ..., cost: _Optional[_Union[datetime.timedelta, _duration_pb2.Duration, _Mapping]] = ..., state: _Optional[str] = ..., task: _Optional[_Union[Task, _Mapping]] = ..., host: _Optional[_Union[Host, _Mapping]] = ..., need_back_to_source: bool = ..., created_at: _Optional[_Union[datetime.datetime, _timestamp_pb2.Timestamp, _Mapping]] = ..., updated_at: _Optional[_Union[datetime.datetime, _timestamp_pb2.Timestamp, _Mapping]] = ..., concurrent_piece_count: _Optional[int] = ...) -> None: ...

class CachePeer(_message.Message):
    __slots__ = ("id", "range", "priority", "pieces", "cost", "state", "task", "host", "need_back_to_source", "created_at", "updated_at", "concurrent_piece_count")
    ID_FIELD_NUMBER: _ClassVar[int]
    RANGE_FIELD_NUMBER: _ClassVar[int]
    PRIORITY_FIELD_NUMBER: _ClassVar[int]
    PIECES_FIELD_NUMBER: _ClassVar[int]
    COST_FIELD_NUMBER: _ClassVar[int]
    STATE_FIELD_NUMBER: _ClassVar[int]
    TASK_FIELD_NUMBER: _ClassVar[int]
    HOST_FIELD_NUMBER: _ClassVar[int]
    NEED_BACK_TO_SOURCE_FIELD_NUMBER: _ClassVar[int]
    CREATED_AT_FIELD_NUMBER: _ClassVar[int]
    UPDATED_AT_FIELD_NUMBER: _ClassVar[int]
    CONCURRENT_PIECE_COUNT_FIELD_NUMBER: _ClassVar[int]
    id: str
    range: Range
    priority: Priority
    pieces: _containers.RepeatedCompositeFieldContainer[Piece]
    cost: _duration_pb2.Duration
    state: str
    task: CacheTask
    host: Host
    need_back_to_source: bool
    created_at: _timestamp_pb2.Timestamp
    updated_at: _timestamp_pb2.Timestamp
    concurrent_piece_count: int
    def __init__(self, id: _Optional[str] = ..., range: _Optional[_Union[Range, _Mapping]] = ..., priority: _Optional[_Union[Priority, str]] = ..., pieces: _Optional[_Iterable[_Union[Piece, _Mapping]]] = ..., cost: _Optional[_Union[datetime.timedelta, _duration_pb2.Duration, _Mapping]] = ..., state: _Optional[str] = ..., task: _Optional[_Union[CacheTask, _Mapping]] = ..., host: _Optional[_Union[Host, _Mapping]] = ..., need_back_to_source: bool = ..., created_at: _Optional[_Union[datetime.datetime, _timestamp_pb2.Timestamp, _Mapping]] = ..., updated_at: _Optional[_Union[datetime.datetime, _timestamp_pb2.Timestamp, _Mapping]] = ..., concurrent_piece_count: _Optional[int] = ...) -> None: ...

class PersistentPeer(_message.Message):
    __slots__ = ("id", "persistent", "cost", "state", "task", "host", "created_at", "updated_at", "concurrent_piece_count")
    ID_FIELD_NUMBER: _ClassVar[int]
    PERSISTENT_FIELD_NUMBER: _ClassVar[int]
    COST_FIELD_NUMBER: _ClassVar[int]
    STATE_FIELD_NUMBER: _ClassVar[int]
    TASK_FIELD_NUMBER: _ClassVar[int]
    HOST_FIELD_NUMBER: _ClassVar[int]
    CREATED_AT_FIELD_NUMBER: _ClassVar[int]
    UPDATED_AT_FIELD_NUMBER: _ClassVar[int]
    CONCURRENT_PIECE_COUNT_FIELD_NUMBER: _ClassVar[int]
    id: str
    persistent: bool
    cost: _duration_pb2.Duration
    state: str
    task: PersistentTask
    host: Host
    created_at: _timestamp_pb2.Timestamp
    updated_at: _timestamp_pb2.Timestamp
    concurrent_piece_count: int
    def __init__(self, id: _Optional[str] = ..., persistent: bool = ..., cost: _Optional[_Union[datetime.timedelta, _duration_pb2.Duration, _Mapping]] = ..., state: _Optional[str] = ..., task: _Optional[_Union[PersistentTask, _Mapping]] = ..., host: _Optional[_Union[Host, _Mapping]] = ..., created_at: _Optional[_Union[datetime.datetime, _timestamp_pb2.Timestamp, _Mapping]] = ..., updated_at: _Optional[_Union[datetime.datetime, _timestamp_pb2.Timestamp, _Mapping]] = ..., concurrent_piece_count: _Optional[int] = ...) -> None: ...

class PersistentCachePeer(_message.Message):
    __slots__ = ("id", "persistent", "cost", "state", "task", "host", "created_at", "updated_at", "concurrent_piece_count")
    ID_FIELD_NUMBER: _ClassVar[int]
    PERSISTENT_FIELD_NUMBER: _ClassVar[int]
    COST_FIELD_NUMBER: _ClassVar[int]
    STATE_FIELD_NUMBER: _ClassVar[int]
    TASK_FIELD_NUMBER: _ClassVar[int]
    HOST_FIELD_NUMBER: _ClassVar[int]
    CREATED_AT_FIELD_NUMBER: _ClassVar[int]
    UPDATED_AT_FIELD_NUMBER: _ClassVar[int]
    CONCURRENT_PIECE_COUNT_FIELD_NUMBER: _ClassVar[int]
    id: str
    persistent: bool
    cost: _duration_pb2.Duration
    state: str
    task: PersistentCacheTask
    host: Host
    created_at: _timestamp_pb2.Timestamp
    updated_at: _timestamp_pb2.Timestamp
    concurrent_piece_count: int
    def __init__(self, id: _Optional[str] = ..., persistent: bool = ..., cost: _Optional[_Union[datetime.timedelta, _duration_pb2.Duration, _Mapping]] = ..., state: _Optional[str] = ..., task: _Optional[_Union[PersistentCacheTask, _Mapping]] = ..., host: _Optional[_Union[Host, _Mapping]] = ..., created_at: _Optional[_Union[datetime.datetime, _timestamp_pb2.Timestamp, _Mapping]] = ..., updated_at: _Optional[_Union[datetime.datetime, _timestamp_pb2.Timestamp, _Mapping]] = ..., concurrent_piece_count: _Optional[int] = ...) -> None: ...

class Task(_message.Message):
    __slots__ = ("id", "type", "url", "digest", "tag", "application", "filtered_query_params", "request_header", "content_length", "piece_count", "size_scope", "pieces", "state", "peer_count", "has_available_peer", "created_at", "updated_at")
    class RequestHeaderEntry(_message.Message):
        __slots__ = ("key", "value")
        KEY_FIELD_NUMBER: _ClassVar[int]
        VALUE_FIELD_NUMBER: _ClassVar[int]
        key: str
        value: str
        def __init__(self, key: _Optional[str] = ..., value: _Optional[str] = ...) -> None: ...
    ID_FIELD_NUMBER: _ClassVar[int]
    TYPE_FIELD_NUMBER: _ClassVar[int]
    URL_FIELD_NUMBER: _ClassVar[int]
    DIGEST_FIELD_NUMBER: _ClassVar[int]
    TAG_FIELD_NUMBER: _ClassVar[int]
    APPLICATION_FIELD_NUMBER: _ClassVar[int]
    FILTERED_QUERY_PARAMS_FIELD_NUMBER: _ClassVar[int]
    REQUEST_HEADER_FIELD_NUMBER: _ClassVar[int]
    CONTENT_LENGTH_FIELD_NUMBER: _ClassVar[int]
    PIECE_COUNT_FIELD_NUMBER: _ClassVar[int]
    SIZE_SCOPE_FIELD_NUMBER: _ClassVar[int]
    PIECES_FIELD_NUMBER: _ClassVar[int]
    STATE_FIELD_NUMBER: _ClassVar[int]
    PEER_COUNT_FIELD_NUMBER: _ClassVar[int]
    HAS_AVAILABLE_PEER_FIELD_NUMBER: _ClassVar[int]
    CREATED_AT_FIELD_NUMBER: _ClassVar[int]
    UPDATED_AT_FIELD_NUMBER: _ClassVar[int]
    id: str
    type: TaskType
    url: str
    digest: str
    tag: str
    application: str
    filtered_query_params: _containers.RepeatedScalarFieldContainer[str]
    request_header: _containers.ScalarMap[str, str]
    content_length: int
    piece_count: int
    size_scope: SizeScope
    pieces: _containers.RepeatedCompositeFieldContainer[Piece]
    state: str
    peer_count: int
    has_available_peer: bool
    created_at: _timestamp_pb2.Timestamp
    updated_at: _timestamp_pb2.Timestamp
    def __init__(self, id: _Optional[str] = ..., type: _Optional[_Union[TaskType, str]] = ..., url: _Optional[str] = ..., digest: _Optional[str] = ..., tag: _Optional[str] = ..., application: _Optional[str] = ..., filtered_query_params: _Optional[_Iterable[str]] = ..., request_header: _Optional[_Mapping[str, str]] = ..., content_length: _Optional[int] = ..., piece_count: _Optional[int] = ..., size_scope: _Optional[_Union[SizeScope, str]] = ..., pieces: _Optional[_Iterable[_Union[Piece, _Mapping]]] = ..., state: _Optional[str] = ..., peer_count: _Optional[int] = ..., has_available_peer: bool = ..., created_at: _Optional[_Union[datetime.datetime, _timestamp_pb2.Timestamp, _Mapping]] = ..., updated_at: _Optional[_Union[datetime.datetime, _timestamp_pb2.Timestamp, _Mapping]] = ...) -> None: ...

class CacheTask(_message.Message):
    __slots__ = ("id", "type", "url", "digest", "tag", "application", "filtered_query_params", "request_header", "content_length", "piece_count", "size_scope", "pieces", "state", "peer_count", "has_available_peer", "created_at", "updated_at")
    class RequestHeaderEntry(_message.Message):
        __slots__ = ("key", "value")
        KEY_FIELD_NUMBER: _ClassVar[int]
        VALUE_FIELD_NUMBER: _ClassVar[int]
        key: str
        value: str
        def __init__(self, key: _Optional[str] = ..., value: _Optional[str] = ...) -> None: ...
    ID_FIELD_NUMBER: _ClassVar[int]
    TYPE_FIELD_NUMBER: _ClassVar[int]
    URL_FIELD_NUMBER: _ClassVar[int]
    DIGEST_FIELD_NUMBER: _ClassVar[int]
    TAG_FIELD_NUMBER: _ClassVar[int]
    APPLICATION_FIELD_NUMBER: _ClassVar[int]
    FILTERED_QUERY_PARAMS_FIELD_NUMBER: _ClassVar[int]
    REQUEST_HEADER_FIELD_NUMBER: _ClassVar[int]
    CONTENT_LENGTH_FIELD_NUMBER: _ClassVar[int]
    PIECE_COUNT_FIELD_NUMBER: _ClassVar[int]
    SIZE_SCOPE_FIELD_NUMBER: _ClassVar[int]
    PIECES_FIELD_NUMBER: _ClassVar[int]
    STATE_FIELD_NUMBER: _ClassVar[int]
    PEER_COUNT_FIELD_NUMBER: _ClassVar[int]
    HAS_AVAILABLE_PEER_FIELD_NUMBER: _ClassVar[int]
    CREATED_AT_FIELD_NUMBER: _ClassVar[int]
    UPDATED_AT_FIELD_NUMBER: _ClassVar[int]
    id: str
    type: TaskType
    url: str
    digest: str
    tag: str
    application: str
    filtered_query_params: _containers.RepeatedScalarFieldContainer[str]
    request_header: _containers.ScalarMap[str, str]
    content_length: int
    piece_count: int
    size_scope: SizeScope
    pieces: _containers.RepeatedCompositeFieldContainer[Piece]
    state: str
    peer_count: int
    has_available_peer: bool
    created_at: _timestamp_pb2.Timestamp
    updated_at: _timestamp_pb2.Timestamp
    def __init__(self, id: _Optional[str] = ..., type: _Optional[_Union[TaskType, str]] = ..., url: _Optional[str] = ..., digest: _Optional[str] = ..., tag: _Optional[str] = ..., application: _Optional[str] = ..., filtered_query_params: _Optional[_Iterable[str]] = ..., request_header: _Optional[_Mapping[str, str]] = ..., content_length: _Optional[int] = ..., piece_count: _Optional[int] = ..., size_scope: _Optional[_Union[SizeScope, str]] = ..., pieces: _Optional[_Iterable[_Union[Piece, _Mapping]]] = ..., state: _Optional[str] = ..., peer_count: _Optional[int] = ..., has_available_peer: bool = ..., created_at: _Optional[_Union[datetime.datetime, _timestamp_pb2.Timestamp, _Mapping]] = ..., updated_at: _Optional[_Union[datetime.datetime, _timestamp_pb2.Timestamp, _Mapping]] = ...) -> None: ...

class PersistentTask(_message.Message):
    __slots__ = ("id", "persistent_replica_count", "current_persistent_replica_count", "current_replica_count", "content_length", "piece_count", "state", "ttl", "created_at", "updated_at")
    ID_FIELD_NUMBER: _ClassVar[int]
    PERSISTENT_REPLICA_COUNT_FIELD_NUMBER: _ClassVar[int]
    CURRENT_PERSISTENT_REPLICA_COUNT_FIELD_NUMBER: _ClassVar[int]
    CURRENT_REPLICA_COUNT_FIELD_NUMBER: _ClassVar[int]
    CONTENT_LENGTH_FIELD_NUMBER: _ClassVar[int]
    PIECE_COUNT_FIELD_NUMBER: _ClassVar[int]
    STATE_FIELD_NUMBER: _ClassVar[int]
    TTL_FIELD_NUMBER: _ClassVar[int]
    CREATED_AT_FIELD_NUMBER: _ClassVar[int]
    UPDATED_AT_FIELD_NUMBER: _ClassVar[int]
    id: str
    persistent_replica_count: int
    current_persistent_replica_count: int
    current_replica_count: int
    content_length: int
    piece_count: int
    state: str
    ttl: _duration_pb2.Duration
    created_at: _timestamp_pb2.Timestamp
    updated_at: _timestamp_pb2.Timestamp
    def __init__(self, id: _Optional[str] = ..., persistent_replica_count: _Optional[int] = ..., current_persistent_replica_count: _Optional[int] = ..., current_replica_count: _Optional[int] = ..., content_length: _Optional[int] = ..., piece_count: _Optional[int] = ..., state: _Optional[str] = ..., ttl: _Optional[_Union[datetime.timedelta, _duration_pb2.Duration, _Mapping]] = ..., created_at: _Optional[_Union[datetime.datetime, _timestamp_pb2.Timestamp, _Mapping]] = ..., updated_at: _Optional[_Union[datetime.datetime, _timestamp_pb2.Timestamp, _Mapping]] = ...) -> None: ...

class PersistentCacheTask(_message.Message):
    __slots__ = ("id", "persistent_replica_count", "current_persistent_replica_count", "current_replica_count", "tag", "application", "piece_length", "content_length", "piece_count", "state", "ttl", "created_at", "updated_at")
    ID_FIELD_NUMBER: _ClassVar[int]
    PERSISTENT_REPLICA_COUNT_FIELD_NUMBER: _ClassVar[int]
    CURRENT_PERSISTENT_REPLICA_COUNT_FIELD_NUMBER: _ClassVar[int]
    CURRENT_REPLICA_COUNT_FIELD_NUMBER: _ClassVar[int]
    TAG_FIELD_NUMBER: _ClassVar[int]
    APPLICATION_FIELD_NUMBER: _ClassVar[int]
    PIECE_LENGTH_FIELD_NUMBER: _ClassVar[int]
    CONTENT_LENGTH_FIELD_NUMBER: _ClassVar[int]
    PIECE_COUNT_FIELD_NUMBER: _ClassVar[int]
    STATE_FIELD_NUMBER: _ClassVar[int]
    TTL_FIELD_NUMBER: _ClassVar[int]
    CREATED_AT_FIELD_NUMBER: _ClassVar[int]
    UPDATED_AT_FIELD_NUMBER: _ClassVar[int]
    id: str
    persistent_replica_count: int
    current_persistent_replica_count: int
    current_replica_count: int
    tag: str
    application: str
    piece_length: int
    content_length: int
    piece_count: int
    state: str
    ttl: _duration_pb2.Duration
    created_at: _timestamp_pb2.Timestamp
    updated_at: _timestamp_pb2.Timestamp
    def __init__(self, id: _Optional[str] = ..., persistent_replica_count: _Optional[int] = ..., current_persistent_replica_count: _Optional[int] = ..., current_replica_count: _Optional[int] = ..., tag: _Optional[str] = ..., application: _Optional[str] = ..., piece_length: _Optional[int] = ..., content_length: _Optional[int] = ..., piece_count: _Optional[int] = ..., state: _Optional[str] = ..., ttl: _Optional[_Union[datetime.timedelta, _duration_pb2.Duration, _Mapping]] = ..., created_at: _Optional[_Union[datetime.datetime, _timestamp_pb2.Timestamp, _Mapping]] = ..., updated_at: _Optional[_Union[datetime.datetime, _timestamp_pb2.Timestamp, _Mapping]] = ...) -> None: ...

class Host(_message.Message):
    __slots__ = ("id", "type", "hostname", "ip", "port", "download_port", "os", "platform", "platform_family", "platform_version", "kernel_version", "cpu", "memory", "network", "disk", "build", "scheduler_cluster_id", "disable_shared", "proxy_port", "name")
    ID_FIELD_NUMBER: _ClassVar[int]
    TYPE_FIELD_NUMBER: _ClassVar[int]
    HOSTNAME_FIELD_NUMBER: _ClassVar[int]
    IP_FIELD_NUMBER: _ClassVar[int]
    PORT_FIELD_NUMBER: _ClassVar[int]
    DOWNLOAD_PORT_FIELD_NUMBER: _ClassVar[int]
    OS_FIELD_NUMBER: _ClassVar[int]
    PLATFORM_FIELD_NUMBER: _ClassVar[int]
    PLATFORM_FAMILY_FIELD_NUMBER: _ClassVar[int]
    PLATFORM_VERSION_FIELD_NUMBER: _ClassVar[int]
    KERNEL_VERSION_FIELD_NUMBER: _ClassVar[int]
    CPU_FIELD_NUMBER: _ClassVar[int]
    MEMORY_FIELD_NUMBER: _ClassVar[int]
    NETWORK_FIELD_NUMBER: _ClassVar[int]
    DISK_FIELD_NUMBER: _ClassVar[int]
    BUILD_FIELD_NUMBER: _ClassVar[int]
    SCHEDULER_CLUSTER_ID_FIELD_NUMBER: _ClassVar[int]
    DISABLE_SHARED_FIELD_NUMBER: _ClassVar[int]
    PROXY_PORT_FIELD_NUMBER: _ClassVar[int]
    NAME_FIELD_NUMBER: _ClassVar[int]
    id: str
    type: int
    hostname: str
    ip: str
    port: int
    download_port: int
    os: str
    platform: str
    platform_family: str
    platform_version: str
    kernel_version: str
    cpu: CPU
    memory: Memory
    network: Network
    disk: Disk
    build: Build
    scheduler_cluster_id: int
    disable_shared: bool
    proxy_port: int
    name: str
    def __init__(self, id: _Optional[str] = ..., type: _Optional[int] = ..., hostname: _Optional[str] = ..., ip: _Optional[str] = ..., port: _Optional[int] = ..., download_port: _Optional[int] = ..., os: _Optional[str] = ..., platform: _Optional[str] = ..., platform_family: _Optional[str] = ..., platform_version: _Optional[str] = ..., kernel_version: _Optional[str] = ..., cpu: _Optional[_Union[CPU, _Mapping]] = ..., memory: _Optional[_Union[Memory, _Mapping]] = ..., network: _Optional[_Union[Network, _Mapping]] = ..., disk: _Optional[_Union[Disk, _Mapping]] = ..., build: _Optional[_Union[Build, _Mapping]] = ..., scheduler_cluster_id: _Optional[int] = ..., disable_shared: bool = ..., proxy_port: _Optional[int] = ..., name: _Optional[str] = ...) -> None: ...

class CPU(_message.Message):
    __slots__ = ("logical_count", "physical_count", "percent", "process_percent", "times", "cgroup")
    LOGICAL_COUNT_FIELD_NUMBER: _ClassVar[int]
    PHYSICAL_COUNT_FIELD_NUMBER: _ClassVar[int]
    PERCENT_FIELD_NUMBER: _ClassVar[int]
    PROCESS_PERCENT_FIELD_NUMBER: _ClassVar[int]
    TIMES_FIELD_NUMBER: _ClassVar[int]
    CGROUP_FIELD_NUMBER: _ClassVar[int]
    logical_count: int
    physical_count: int
    percent: float
    process_percent: float
    times: CPUTimes
    cgroup: CgroupCPU
    def __init__(self, logical_count: _Optional[int] = ..., physical_count: _Optional[int] = ..., percent: _Optional[float] = ..., process_percent: _Optional[float] = ..., times: _Optional[_Union[CPUTimes, _Mapping]] = ..., cgroup: _Optional[_Union[CgroupCPU, _Mapping]] = ...) -> None: ...

class CgroupCPU(_message.Message):
    __slots__ = ("period", "quota", "used_percent")
    PERIOD_FIELD_NUMBER: _ClassVar[int]
    QUOTA_FIELD_NUMBER: _ClassVar[int]
    USED_PERCENT_FIELD_NUMBER: _ClassVar[int]
    period: int
    quota: int
    used_percent: float
    def __init__(self, period: _Optional[int] = ..., quota: _Optional[int] = ..., used_percent: _Optional[float] = ...) -> None: ...

class CPUTimes(_message.Message):
    __slots__ = ("user", "system", "idle", "nice", "iowait", "irq", "softirq", "steal", "guest", "guest_nice")
    USER_FIELD_NUMBER: _ClassVar[int]
    SYSTEM_FIELD_NUMBER: _ClassVar[int]
    IDLE_FIELD_NUMBER: _ClassVar[int]
    NICE_FIELD_NUMBER: _ClassVar[int]
    IOWAIT_FIELD_NUMBER: _ClassVar[int]
    IRQ_FIELD_NUMBER: _ClassVar[int]
    SOFTIRQ_FIELD_NUMBER: _ClassVar[int]
    STEAL_FIELD_NUMBER: _ClassVar[int]
    GUEST_FIELD_NUMBER: _ClassVar[int]
    GUEST_NICE_FIELD_NUMBER: _ClassVar[int]
    user: float
    system: float
    idle: float
    nice: float
    iowait: float
    irq: float
    softirq: float
    steal: float
    guest: float
    guest_nice: float
    def __init__(self, user: _Optional[float] = ..., system: _Optional[float] = ..., idle: _Optional[float] = ..., nice: _Optional[float] = ..., iowait: _Optional[float] = ..., irq: _Optional[float] = ..., softirq: _Optional[float] = ..., steal: _Optional[float] = ..., guest: _Optional[float] = ..., guest_nice: _Optional[float] = ...) -> None: ...

class Memory(_message.Message):
    __slots__ = ("total", "available", "used", "used_percent", "process_used_percent", "free", "cgroup")
    TOTAL_FIELD_NUMBER: _ClassVar[int]
    AVAILABLE_FIELD_NUMBER: _ClassVar[int]
    USED_FIELD_NUMBER: _ClassVar[int]
    USED_PERCENT_FIELD_NUMBER: _ClassVar[int]
    PROCESS_USED_PERCENT_FIELD_NUMBER: _ClassVar[int]
    FREE_FIELD_NUMBER: _ClassVar[int]
    CGROUP_FIELD_NUMBER: _ClassVar[int]
    total: int
    available: int
    used: int
    used_percent: float
    process_used_percent: float
    free: int
    cgroup: CgroupMemory
    def __init__(self, total: _Optional[int] = ..., available: _Optional[int] = ..., used: _Optional[int] = ..., used_percent: _Optional[float] = ..., process_used_percent: _Optional[float] = ..., free: _Optional[int] = ..., cgroup: _Optional[_Union[CgroupMemory, _Mapping]] = ...) -> None: ...

class CgroupMemory(_message.Message):
    __slots__ = ("limit", "usage", "used_percent")
    LIMIT_FIELD_NUMBER: _ClassVar[int]
    USAGE_FIELD_NUMBER: _ClassVar[int]
    USED_PERCENT_FIELD_NUMBER: _ClassVar[int]
    limit: int
    usage: int
    used_percent: float
    def __init__(self, limit: _Optional[int] = ..., usage: _Optional[int] = ..., used_percent: _Optional[float] = ...) -> None: ...

class Network(_message.Message):
    __slots__ = ("tcp_connection_count", "upload_tcp_connection_count", "location", "idc", "max_rx_bandwidth", "rx_bandwidth", "max_tx_bandwidth", "tx_bandwidth")
    TCP_CONNECTION_COUNT_FIELD_NUMBER: _ClassVar[int]
    UPLOAD_TCP_CONNECTION_COUNT_FIELD_NUMBER: _ClassVar[int]
    LOCATION_FIELD_NUMBER: _ClassVar[int]
    IDC_FIELD_NUMBER: _ClassVar[int]
    MAX_RX_BANDWIDTH_FIELD_NUMBER: _ClassVar[int]
    RX_BANDWIDTH_FIELD_NUMBER: _ClassVar[int]
    MAX_TX_BANDWIDTH_FIELD_NUMBER: _ClassVar[int]
    TX_BANDWIDTH_FIELD_NUMBER: _ClassVar[int]
    tcp_connection_count: int
    upload_tcp_connection_count: int
    location: str
    idc: str
    max_rx_bandwidth: int
    rx_bandwidth: int
    max_tx_bandwidth: int
    tx_bandwidth: int
    def __init__(self, tcp_connection_count: _Optional[int] = ..., upload_tcp_connection_count: _Optional[int] = ..., location: _Optional[str] = ..., idc: _Optional[str] = ..., max_rx_bandwidth: _Optional[int] = ..., rx_bandwidth: _Optional[int] = ..., max_tx_bandwidth: _Optional[int] = ..., tx_bandwidth: _Optional[int] = ...) -> None: ...

class Disk(_message.Message):
    __slots__ = ("total", "free", "used", "used_percent", "inodes_total", "inodes_used", "inodes_free", "inodes_used_percent", "read_bandwidth", "write_bandwidth", "cgroup")
    TOTAL_FIELD_NUMBER: _ClassVar[int]
    FREE_FIELD_NUMBER: _ClassVar[int]
    USED_FIELD_NUMBER: _ClassVar[int]
    USED_PERCENT_FIELD_NUMBER: _ClassVar[int]
    INODES_TOTAL_FIELD_NUMBER: _ClassVar[int]
    INODES_USED_FIELD_NUMBER: _ClassVar[int]
    INODES_FREE_FIELD_NUMBER: _ClassVar[int]
    INODES_USED_PERCENT_FIELD_NUMBER: _ClassVar[int]
    READ_BANDWIDTH_FIELD_NUMBER: _ClassVar[int]
    WRITE_BANDWIDTH_FIELD_NUMBER: _ClassVar[int]
    CGROUP_FIELD_NUMBER: _ClassVar[int]
    total: int
    free: int
    used: int
    used_percent: float
    inodes_total: int
    inodes_used: int
    inodes_free: int
    inodes_used_percent: float
    read_bandwidth: int
    write_bandwidth: int
    cgroup: CgroupDisk
    def __init__(self, total: _Optional[int] = ..., free: _Optional[int] = ..., used: _Optional[int] = ..., used_percent: _Optional[float] = ..., inodes_total: _Optional[int] = ..., inodes_used: _Optional[int] = ..., inodes_free: _Optional[int] = ..., inodes_used_percent: _Optional[float] = ..., read_bandwidth: _Optional[int] = ..., write_bandwidth: _Optional[int] = ..., cgroup: _Optional[_Union[CgroupDisk, _Mapping]] = ...) -> None: ...

class CgroupDisk(_message.Message):
    __slots__ = ("read_bandwidth", "write_bandwidth")
    READ_BANDWIDTH_FIELD_NUMBER: _ClassVar[int]
    WRITE_BANDWIDTH_FIELD_NUMBER: _ClassVar[int]
    read_bandwidth: int
    write_bandwidth: int
    def __init__(self, read_bandwidth: _Optional[int] = ..., write_bandwidth: _Optional[int] = ...) -> None: ...

class Build(_message.Message):
    __slots__ = ("git_version", "git_commit", "go_version", "rust_version", "platform")
    GIT_VERSION_FIELD_NUMBER: _ClassVar[int]
    GIT_COMMIT_FIELD_NUMBER: _ClassVar[int]
    GO_VERSION_FIELD_NUMBER: _ClassVar[int]
    RUST_VERSION_FIELD_NUMBER: _ClassVar[int]
    PLATFORM_FIELD_NUMBER: _ClassVar[int]
    git_version: str
    git_commit: str
    go_version: str
    rust_version: str
    platform: str
    def __init__(self, git_version: _Optional[str] = ..., git_commit: _Optional[str] = ..., go_version: _Optional[str] = ..., rust_version: _Optional[str] = ..., platform: _Optional[str] = ...) -> None: ...

class Download(_message.Message):
    __slots__ = ("url", "digest", "range", "type", "tag", "application", "priority", "filtered_query_params", "request_header", "piece_length", "output_path", "timeout", "disable_back_to_source", "need_back_to_source", "certificate_chain", "prefetch", "object_storage", "hdfs", "is_prefetch", "need_piece_content", "force_hard_link", "content_for_calculating_task_id", "remote_ip", "concurrent_piece_count", "overwrite", "actual_piece_length", "actual_content_length", "actual_piece_count", "enable_task_id_based_blob_digest", "hugging_face", "model_scope", "metadata_only", "open_csg", "need_scheduling")
    class RequestHeaderEntry(_message.Message):
        __slots__ = ("key", "value")
        KEY_FIELD_NUMBER: _ClassVar[int]
        VALUE_FIELD_NUMBER: _ClassVar[int]
        key: str
        value: str
        def __init__(self, key: _Optional[str] = ..., value: _Optional[str] = ...) -> None: ...
    URL_FIELD_NUMBER: _ClassVar[int]
    DIGEST_FIELD_NUMBER: _ClassVar[int]
    RANGE_FIELD_NUMBER: _ClassVar[int]
    TYPE_FIELD_NUMBER: _ClassVar[int]
    TAG_FIELD_NUMBER: _ClassVar[int]
    APPLICATION_FIELD_NUMBER: _ClassVar[int]
    PRIORITY_FIELD_NUMBER: _ClassVar[int]
    FILTERED_QUERY_PARAMS_FIELD_NUMBER: _ClassVar[int]
    REQUEST_HEADER_FIELD_NUMBER: _ClassVar[int]
    PIECE_LENGTH_FIELD_NUMBER: _ClassVar[int]
    OUTPUT_PATH_FIELD_NUMBER: _ClassVar[int]
    TIMEOUT_FIELD_NUMBER: _ClassVar[int]
    DISABLE_BACK_TO_SOURCE_FIELD_NUMBER: _ClassVar[int]
    NEED_BACK_TO_SOURCE_FIELD_NUMBER: _ClassVar[int]
    CERTIFICATE_CHAIN_FIELD_NUMBER: _ClassVar[int]
    PREFETCH_FIELD_NUMBER: _ClassVar[int]
    OBJECT_STORAGE_FIELD_NUMBER: _ClassVar[int]
    HDFS_FIELD_NUMBER: _ClassVar[int]
    IS_PREFETCH_FIELD_NUMBER: _ClassVar[int]
    NEED_PIECE_CONTENT_FIELD_NUMBER: _ClassVar[int]
    FORCE_HARD_LINK_FIELD_NUMBER: _ClassVar[int]
    CONTENT_FOR_CALCULATING_TASK_ID_FIELD_NUMBER: _ClassVar[int]
    REMOTE_IP_FIELD_NUMBER: _ClassVar[int]
    CONCURRENT_PIECE_COUNT_FIELD_NUMBER: _ClassVar[int]
    OVERWRITE_FIELD_NUMBER: _ClassVar[int]
    ACTUAL_PIECE_LENGTH_FIELD_NUMBER: _ClassVar[int]
    ACTUAL_CONTENT_LENGTH_FIELD_NUMBER: _ClassVar[int]
    ACTUAL_PIECE_COUNT_FIELD_NUMBER: _ClassVar[int]
    ENABLE_TASK_ID_BASED_BLOB_DIGEST_FIELD_NUMBER: _ClassVar[int]
    HUGGING_FACE_FIELD_NUMBER: _ClassVar[int]
    MODEL_SCOPE_FIELD_NUMBER: _ClassVar[int]
    METADATA_ONLY_FIELD_NUMBER: _ClassVar[int]
    OPEN_CSG_FIELD_NUMBER: _ClassVar[int]
    NEED_SCHEDULING_FIELD_NUMBER: _ClassVar[int]
    url: str
    digest: str
    range: Range
    type: TaskType
    tag: str
    application: str
    priority: Priority
    filtered_query_params: _containers.RepeatedScalarFieldContainer[str]
    request_header: _containers.ScalarMap[str, str]
    piece_length: int
    output_path: str
    timeout: _duration_pb2.Duration
    disable_back_to_source: bool
    need_back_to_source: bool
    certificate_chain: _containers.RepeatedScalarFieldContainer[bytes]
    prefetch: bool
    object_storage: ObjectStorage
    hdfs: HDFS
    is_prefetch: bool
    need_piece_content: bool
    force_hard_link: bool
    content_for_calculating_task_id: str
    remote_ip: str
    concurrent_piece_count: int
    overwrite: bool
    actual_piece_length: int
    actual_content_length: int
    actual_piece_count: int
    enable_task_id_based_blob_digest: bool
    hugging_face: HuggingFace
    model_scope: ModelScope
    metadata_only: bool
    open_csg: OpenCSG
    need_scheduling: bool
    def __init__(self, url: _Optional[str] = ..., digest: _Optional[str] = ..., range: _Optional[_Union[Range, _Mapping]] = ..., type: _Optional[_Union[TaskType, str]] = ..., tag: _Optional[str] = ..., application: _Optional[str] = ..., priority: _Optional[_Union[Priority, str]] = ..., filtered_query_params: _Optional[_Iterable[str]] = ..., request_header: _Optional[_Mapping[str, str]] = ..., piece_length: _Optional[int] = ..., output_path: _Optional[str] = ..., timeout: _Optional[_Union[datetime.timedelta, _duration_pb2.Duration, _Mapping]] = ..., disable_back_to_source: bool = ..., need_back_to_source: bool = ..., certificate_chain: _Optional[_Iterable[bytes]] = ..., prefetch: bool = ..., object_storage: _Optional[_Union[ObjectStorage, _Mapping]] = ..., hdfs: _Optional[_Union[HDFS, _Mapping]] = ..., is_prefetch: bool = ..., need_piece_content: bool = ..., force_hard_link: bool = ..., content_for_calculating_task_id: _Optional[str] = ..., remote_ip: _Optional[str] = ..., concurrent_piece_count: _Optional[int] = ..., overwrite: bool = ..., actual_piece_length: _Optional[int] = ..., actual_content_length: _Optional[int] = ..., actual_piece_count: _Optional[int] = ..., enable_task_id_based_blob_digest: bool = ..., hugging_face: _Optional[_Union[HuggingFace, _Mapping]] = ..., model_scope: _Optional[_Union[ModelScope, _Mapping]] = ..., metadata_only: bool = ..., open_csg: _Optional[_Union[OpenCSG, _Mapping]] = ..., need_scheduling: bool = ...) -> None: ...

class ObjectStorage(_message.Message):
    __slots__ = ("region", "endpoint", "access_key_id", "access_key_secret", "session_token", "credential_path", "predefined_acl", "security_token", "insecure_skip_verify")
    REGION_FIELD_NUMBER: _ClassVar[int]
    ENDPOINT_FIELD_NUMBER: _ClassVar[int]
    ACCESS_KEY_ID_FIELD_NUMBER: _ClassVar[int]
    ACCESS_KEY_SECRET_FIELD_NUMBER: _ClassVar[int]
    SESSION_TOKEN_FIELD_NUMBER: _ClassVar[int]
    CREDENTIAL_PATH_FIELD_NUMBER: _ClassVar[int]
    PREDEFINED_ACL_FIELD_NUMBER: _ClassVar[int]
    SECURITY_TOKEN_FIELD_NUMBER: _ClassVar[int]
    INSECURE_SKIP_VERIFY_FIELD_NUMBER: _ClassVar[int]
    region: str
    endpoint: str
    access_key_id: str
    access_key_secret: str
    session_token: str
    credential_path: str
    predefined_acl: str
    security_token: str
    insecure_skip_verify: bool
    def __init__(self, region: _Optional[str] = ..., endpoint: _Optional[str] = ..., access_key_id: _Optional[str] = ..., access_key_secret: _Optional[str] = ..., session_token: _Optional[str] = ..., credential_path: _Optional[str] = ..., predefined_acl: _Optional[str] = ..., security_token: _Optional[str] = ..., insecure_skip_verify: bool = ...) -> None: ...

class HDFS(_message.Message):
    __slots__ = ("delegation_token",)
    DELEGATION_TOKEN_FIELD_NUMBER: _ClassVar[int]
    delegation_token: str
    def __init__(self, delegation_token: _Optional[str] = ...) -> None: ...

class HuggingFace(_message.Message):
    __slots__ = ("token", "revision", "base_url")
    TOKEN_FIELD_NUMBER: _ClassVar[int]
    REVISION_FIELD_NUMBER: _ClassVar[int]
    BASE_URL_FIELD_NUMBER: _ClassVar[int]
    token: str
    revision: str
    base_url: str
    def __init__(self, token: _Optional[str] = ..., revision: _Optional[str] = ..., base_url: _Optional[str] = ...) -> None: ...

class ModelScope(_message.Message):
    __slots__ = ("token", "revision", "base_url")
    TOKEN_FIELD_NUMBER: _ClassVar[int]
    REVISION_FIELD_NUMBER: _ClassVar[int]
    BASE_URL_FIELD_NUMBER: _ClassVar[int]
    token: str
    revision: str
    base_url: str
    def __init__(self, token: _Optional[str] = ..., revision: _Optional[str] = ..., base_url: _Optional[str] = ...) -> None: ...

class OpenCSG(_message.Message):
    __slots__ = ("token", "revision", "base_url")
    TOKEN_FIELD_NUMBER: _ClassVar[int]
    REVISION_FIELD_NUMBER: _ClassVar[int]
    BASE_URL_FIELD_NUMBER: _ClassVar[int]
    token: str
    revision: str
    base_url: str
    def __init__(self, token: _Optional[str] = ..., revision: _Optional[str] = ..., base_url: _Optional[str] = ...) -> None: ...

class Range(_message.Message):
    __slots__ = ("start", "length")
    START_FIELD_NUMBER: _ClassVar[int]
    LENGTH_FIELD_NUMBER: _ClassVar[int]
    start: int
    length: int
    def __init__(self, start: _Optional[int] = ..., length: _Optional[int] = ...) -> None: ...

class Piece(_message.Message):
    __slots__ = ("number", "parent_id", "offset", "length", "digest", "content", "traffic_type", "cost", "created_at")
    NUMBER_FIELD_NUMBER: _ClassVar[int]
    PARENT_ID_FIELD_NUMBER: _ClassVar[int]
    OFFSET_FIELD_NUMBER: _ClassVar[int]
    LENGTH_FIELD_NUMBER: _ClassVar[int]
    DIGEST_FIELD_NUMBER: _ClassVar[int]
    CONTENT_FIELD_NUMBER: _ClassVar[int]
    TRAFFIC_TYPE_FIELD_NUMBER: _ClassVar[int]
    COST_FIELD_NUMBER: _ClassVar[int]
    CREATED_AT_FIELD_NUMBER: _ClassVar[int]
    number: int
    parent_id: str
    offset: int
    length: int
    digest: str
    content: bytes
    traffic_type: TrafficType
    cost: _duration_pb2.Duration
    created_at: _timestamp_pb2.Timestamp
    def __init__(self, number: _Optional[int] = ..., parent_id: _Optional[str] = ..., offset: _Optional[int] = ..., length: _Optional[int] = ..., digest: _Optional[str] = ..., content: _Optional[bytes] = ..., traffic_type: _Optional[_Union[TrafficType, str]] = ..., cost: _Optional[_Union[datetime.timedelta, _duration_pb2.Duration, _Mapping]] = ..., created_at: _Optional[_Union[datetime.datetime, _timestamp_pb2.Timestamp, _Mapping]] = ...) -> None: ...
