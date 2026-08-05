from . import common_pb2 as _common_pb2
from google.protobuf import empty_pb2 as _empty_pb2
from google.protobuf.internal import containers as _containers
from google.protobuf.internal import enum_type_wrapper as _enum_type_wrapper
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from collections.abc import Iterable as _Iterable, Mapping as _Mapping
from typing import ClassVar as _ClassVar, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class SourceType(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = ()
    SCHEDULER_SOURCE: _ClassVar[SourceType]
    PEER_SOURCE: _ClassVar[SourceType]
    SEED_PEER_SOURCE: _ClassVar[SourceType]
SCHEDULER_SOURCE: SourceType
PEER_SOURCE: SourceType
SEED_PEER_SOURCE: SourceType

class SeedPeerCluster(_message.Message):
    __slots__ = ("id", "name", "bio", "config")
    ID_FIELD_NUMBER: _ClassVar[int]
    NAME_FIELD_NUMBER: _ClassVar[int]
    BIO_FIELD_NUMBER: _ClassVar[int]
    CONFIG_FIELD_NUMBER: _ClassVar[int]
    id: int
    name: str
    bio: str
    config: bytes
    def __init__(self, id: _Optional[int] = ..., name: _Optional[str] = ..., bio: _Optional[str] = ..., config: _Optional[bytes] = ...) -> None: ...

class SeedPeer(_message.Message):
    __slots__ = ("id", "hostname", "type", "idc", "location", "ip", "port", "download_port", "state", "seed_peer_cluster_id", "seed_peer_cluster", "schedulers")
    ID_FIELD_NUMBER: _ClassVar[int]
    HOSTNAME_FIELD_NUMBER: _ClassVar[int]
    TYPE_FIELD_NUMBER: _ClassVar[int]
    IDC_FIELD_NUMBER: _ClassVar[int]
    LOCATION_FIELD_NUMBER: _ClassVar[int]
    IP_FIELD_NUMBER: _ClassVar[int]
    PORT_FIELD_NUMBER: _ClassVar[int]
    DOWNLOAD_PORT_FIELD_NUMBER: _ClassVar[int]
    STATE_FIELD_NUMBER: _ClassVar[int]
    SEED_PEER_CLUSTER_ID_FIELD_NUMBER: _ClassVar[int]
    SEED_PEER_CLUSTER_FIELD_NUMBER: _ClassVar[int]
    SCHEDULERS_FIELD_NUMBER: _ClassVar[int]
    id: int
    hostname: str
    type: str
    idc: str
    location: str
    ip: str
    port: int
    download_port: int
    state: str
    seed_peer_cluster_id: int
    seed_peer_cluster: SeedPeerCluster
    schedulers: _containers.RepeatedCompositeFieldContainer[Scheduler]
    def __init__(self, id: _Optional[int] = ..., hostname: _Optional[str] = ..., type: _Optional[str] = ..., idc: _Optional[str] = ..., location: _Optional[str] = ..., ip: _Optional[str] = ..., port: _Optional[int] = ..., download_port: _Optional[int] = ..., state: _Optional[str] = ..., seed_peer_cluster_id: _Optional[int] = ..., seed_peer_cluster: _Optional[_Union[SeedPeerCluster, _Mapping]] = ..., schedulers: _Optional[_Iterable[_Union[Scheduler, _Mapping]]] = ...) -> None: ...

class GetSeedPeerRequest(_message.Message):
    __slots__ = ("source_type", "hostname", "seed_peer_cluster_id", "ip")
    SOURCE_TYPE_FIELD_NUMBER: _ClassVar[int]
    HOSTNAME_FIELD_NUMBER: _ClassVar[int]
    SEED_PEER_CLUSTER_ID_FIELD_NUMBER: _ClassVar[int]
    IP_FIELD_NUMBER: _ClassVar[int]
    source_type: SourceType
    hostname: str
    seed_peer_cluster_id: int
    ip: str
    def __init__(self, source_type: _Optional[_Union[SourceType, str]] = ..., hostname: _Optional[str] = ..., seed_peer_cluster_id: _Optional[int] = ..., ip: _Optional[str] = ...) -> None: ...

class ListSeedPeersRequest(_message.Message):
    __slots__ = ("source_type", "hostname", "ip", "version", "commit")
    SOURCE_TYPE_FIELD_NUMBER: _ClassVar[int]
    HOSTNAME_FIELD_NUMBER: _ClassVar[int]
    IP_FIELD_NUMBER: _ClassVar[int]
    VERSION_FIELD_NUMBER: _ClassVar[int]
    COMMIT_FIELD_NUMBER: _ClassVar[int]
    source_type: SourceType
    hostname: str
    ip: str
    version: str
    commit: str
    def __init__(self, source_type: _Optional[_Union[SourceType, str]] = ..., hostname: _Optional[str] = ..., ip: _Optional[str] = ..., version: _Optional[str] = ..., commit: _Optional[str] = ...) -> None: ...

class ListSeedPeersResponse(_message.Message):
    __slots__ = ("seed_peers",)
    SEED_PEERS_FIELD_NUMBER: _ClassVar[int]
    seed_peers: _containers.RepeatedCompositeFieldContainer[SeedPeer]
    def __init__(self, seed_peers: _Optional[_Iterable[_Union[SeedPeer, _Mapping]]] = ...) -> None: ...

class UpdateSeedPeerRequest(_message.Message):
    __slots__ = ("source_type", "hostname", "type", "idc", "location", "ip", "port", "download_port", "seed_peer_cluster_id")
    SOURCE_TYPE_FIELD_NUMBER: _ClassVar[int]
    HOSTNAME_FIELD_NUMBER: _ClassVar[int]
    TYPE_FIELD_NUMBER: _ClassVar[int]
    IDC_FIELD_NUMBER: _ClassVar[int]
    LOCATION_FIELD_NUMBER: _ClassVar[int]
    IP_FIELD_NUMBER: _ClassVar[int]
    PORT_FIELD_NUMBER: _ClassVar[int]
    DOWNLOAD_PORT_FIELD_NUMBER: _ClassVar[int]
    SEED_PEER_CLUSTER_ID_FIELD_NUMBER: _ClassVar[int]
    source_type: SourceType
    hostname: str
    type: str
    idc: str
    location: str
    ip: str
    port: int
    download_port: int
    seed_peer_cluster_id: int
    def __init__(self, source_type: _Optional[_Union[SourceType, str]] = ..., hostname: _Optional[str] = ..., type: _Optional[str] = ..., idc: _Optional[str] = ..., location: _Optional[str] = ..., ip: _Optional[str] = ..., port: _Optional[int] = ..., download_port: _Optional[int] = ..., seed_peer_cluster_id: _Optional[int] = ...) -> None: ...

class DeleteSeedPeerRequest(_message.Message):
    __slots__ = ("source_type", "hostname", "seed_peer_cluster_id", "ip")
    SOURCE_TYPE_FIELD_NUMBER: _ClassVar[int]
    HOSTNAME_FIELD_NUMBER: _ClassVar[int]
    SEED_PEER_CLUSTER_ID_FIELD_NUMBER: _ClassVar[int]
    IP_FIELD_NUMBER: _ClassVar[int]
    source_type: SourceType
    hostname: str
    seed_peer_cluster_id: int
    ip: str
    def __init__(self, source_type: _Optional[_Union[SourceType, str]] = ..., hostname: _Optional[str] = ..., seed_peer_cluster_id: _Optional[int] = ..., ip: _Optional[str] = ...) -> None: ...

class SchedulerCluster(_message.Message):
    __slots__ = ("id", "name", "bio", "config", "client_config", "scopes", "seed_client_config")
    ID_FIELD_NUMBER: _ClassVar[int]
    NAME_FIELD_NUMBER: _ClassVar[int]
    BIO_FIELD_NUMBER: _ClassVar[int]
    CONFIG_FIELD_NUMBER: _ClassVar[int]
    CLIENT_CONFIG_FIELD_NUMBER: _ClassVar[int]
    SCOPES_FIELD_NUMBER: _ClassVar[int]
    SEED_CLIENT_CONFIG_FIELD_NUMBER: _ClassVar[int]
    id: int
    name: str
    bio: str
    config: bytes
    client_config: bytes
    scopes: bytes
    seed_client_config: bytes
    def __init__(self, id: _Optional[int] = ..., name: _Optional[str] = ..., bio: _Optional[str] = ..., config: _Optional[bytes] = ..., client_config: _Optional[bytes] = ..., scopes: _Optional[bytes] = ..., seed_client_config: _Optional[bytes] = ...) -> None: ...

class Scheduler(_message.Message):
    __slots__ = ("id", "hostname", "idc", "location", "ip", "port", "state", "scheduler_cluster_id", "scheduler_cluster", "seed_peers", "features")
    ID_FIELD_NUMBER: _ClassVar[int]
    HOSTNAME_FIELD_NUMBER: _ClassVar[int]
    IDC_FIELD_NUMBER: _ClassVar[int]
    LOCATION_FIELD_NUMBER: _ClassVar[int]
    IP_FIELD_NUMBER: _ClassVar[int]
    PORT_FIELD_NUMBER: _ClassVar[int]
    STATE_FIELD_NUMBER: _ClassVar[int]
    SCHEDULER_CLUSTER_ID_FIELD_NUMBER: _ClassVar[int]
    SCHEDULER_CLUSTER_FIELD_NUMBER: _ClassVar[int]
    SEED_PEERS_FIELD_NUMBER: _ClassVar[int]
    FEATURES_FIELD_NUMBER: _ClassVar[int]
    id: int
    hostname: str
    idc: str
    location: str
    ip: str
    port: int
    state: str
    scheduler_cluster_id: int
    scheduler_cluster: SchedulerCluster
    seed_peers: _containers.RepeatedCompositeFieldContainer[SeedPeer]
    features: bytes
    def __init__(self, id: _Optional[int] = ..., hostname: _Optional[str] = ..., idc: _Optional[str] = ..., location: _Optional[str] = ..., ip: _Optional[str] = ..., port: _Optional[int] = ..., state: _Optional[str] = ..., scheduler_cluster_id: _Optional[int] = ..., scheduler_cluster: _Optional[_Union[SchedulerCluster, _Mapping]] = ..., seed_peers: _Optional[_Iterable[_Union[SeedPeer, _Mapping]]] = ..., features: _Optional[bytes] = ...) -> None: ...

class GetSchedulerRequest(_message.Message):
    __slots__ = ("source_type", "hostname", "scheduler_cluster_id", "ip")
    SOURCE_TYPE_FIELD_NUMBER: _ClassVar[int]
    HOSTNAME_FIELD_NUMBER: _ClassVar[int]
    SCHEDULER_CLUSTER_ID_FIELD_NUMBER: _ClassVar[int]
    IP_FIELD_NUMBER: _ClassVar[int]
    source_type: SourceType
    hostname: str
    scheduler_cluster_id: int
    ip: str
    def __init__(self, source_type: _Optional[_Union[SourceType, str]] = ..., hostname: _Optional[str] = ..., scheduler_cluster_id: _Optional[int] = ..., ip: _Optional[str] = ...) -> None: ...

class UpdateSchedulerRequest(_message.Message):
    __slots__ = ("source_type", "hostname", "scheduler_cluster_id", "idc", "location", "ip", "port", "features", "config")
    SOURCE_TYPE_FIELD_NUMBER: _ClassVar[int]
    HOSTNAME_FIELD_NUMBER: _ClassVar[int]
    SCHEDULER_CLUSTER_ID_FIELD_NUMBER: _ClassVar[int]
    IDC_FIELD_NUMBER: _ClassVar[int]
    LOCATION_FIELD_NUMBER: _ClassVar[int]
    IP_FIELD_NUMBER: _ClassVar[int]
    PORT_FIELD_NUMBER: _ClassVar[int]
    FEATURES_FIELD_NUMBER: _ClassVar[int]
    CONFIG_FIELD_NUMBER: _ClassVar[int]
    source_type: SourceType
    hostname: str
    scheduler_cluster_id: int
    idc: str
    location: str
    ip: str
    port: int
    features: _containers.RepeatedScalarFieldContainer[str]
    config: bytes
    def __init__(self, source_type: _Optional[_Union[SourceType, str]] = ..., hostname: _Optional[str] = ..., scheduler_cluster_id: _Optional[int] = ..., idc: _Optional[str] = ..., location: _Optional[str] = ..., ip: _Optional[str] = ..., port: _Optional[int] = ..., features: _Optional[_Iterable[str]] = ..., config: _Optional[bytes] = ...) -> None: ...

class ListSchedulersRequest(_message.Message):
    __slots__ = ("source_type", "hostname", "ip", "idc", "location", "version", "commit", "scheduler_cluster_id")
    SOURCE_TYPE_FIELD_NUMBER: _ClassVar[int]
    HOSTNAME_FIELD_NUMBER: _ClassVar[int]
    IP_FIELD_NUMBER: _ClassVar[int]
    IDC_FIELD_NUMBER: _ClassVar[int]
    LOCATION_FIELD_NUMBER: _ClassVar[int]
    VERSION_FIELD_NUMBER: _ClassVar[int]
    COMMIT_FIELD_NUMBER: _ClassVar[int]
    SCHEDULER_CLUSTER_ID_FIELD_NUMBER: _ClassVar[int]
    source_type: SourceType
    hostname: str
    ip: str
    idc: str
    location: str
    version: str
    commit: str
    scheduler_cluster_id: int
    def __init__(self, source_type: _Optional[_Union[SourceType, str]] = ..., hostname: _Optional[str] = ..., ip: _Optional[str] = ..., idc: _Optional[str] = ..., location: _Optional[str] = ..., version: _Optional[str] = ..., commit: _Optional[str] = ..., scheduler_cluster_id: _Optional[int] = ...) -> None: ...

class ListSchedulersResponse(_message.Message):
    __slots__ = ("schedulers",)
    SCHEDULERS_FIELD_NUMBER: _ClassVar[int]
    schedulers: _containers.RepeatedCompositeFieldContainer[Scheduler]
    def __init__(self, schedulers: _Optional[_Iterable[_Union[Scheduler, _Mapping]]] = ...) -> None: ...

class URLPriority(_message.Message):
    __slots__ = ("regex", "value")
    REGEX_FIELD_NUMBER: _ClassVar[int]
    VALUE_FIELD_NUMBER: _ClassVar[int]
    regex: str
    value: _common_pb2.Priority
    def __init__(self, regex: _Optional[str] = ..., value: _Optional[_Union[_common_pb2.Priority, str]] = ...) -> None: ...

class ApplicationPriority(_message.Message):
    __slots__ = ("value", "urls")
    VALUE_FIELD_NUMBER: _ClassVar[int]
    URLS_FIELD_NUMBER: _ClassVar[int]
    value: _common_pb2.Priority
    urls: _containers.RepeatedCompositeFieldContainer[URLPriority]
    def __init__(self, value: _Optional[_Union[_common_pb2.Priority, str]] = ..., urls: _Optional[_Iterable[_Union[URLPriority, _Mapping]]] = ...) -> None: ...

class Application(_message.Message):
    __slots__ = ("id", "name", "url", "bio", "priority")
    ID_FIELD_NUMBER: _ClassVar[int]
    NAME_FIELD_NUMBER: _ClassVar[int]
    URL_FIELD_NUMBER: _ClassVar[int]
    BIO_FIELD_NUMBER: _ClassVar[int]
    PRIORITY_FIELD_NUMBER: _ClassVar[int]
    id: int
    name: str
    url: str
    bio: str
    priority: ApplicationPriority
    def __init__(self, id: _Optional[int] = ..., name: _Optional[str] = ..., url: _Optional[str] = ..., bio: _Optional[str] = ..., priority: _Optional[_Union[ApplicationPriority, _Mapping]] = ...) -> None: ...

class ListApplicationsRequest(_message.Message):
    __slots__ = ("source_type", "hostname", "ip")
    SOURCE_TYPE_FIELD_NUMBER: _ClassVar[int]
    HOSTNAME_FIELD_NUMBER: _ClassVar[int]
    IP_FIELD_NUMBER: _ClassVar[int]
    source_type: SourceType
    hostname: str
    ip: str
    def __init__(self, source_type: _Optional[_Union[SourceType, str]] = ..., hostname: _Optional[str] = ..., ip: _Optional[str] = ...) -> None: ...

class ListApplicationsResponse(_message.Message):
    __slots__ = ("applications",)
    APPLICATIONS_FIELD_NUMBER: _ClassVar[int]
    applications: _containers.RepeatedCompositeFieldContainer[Application]
    def __init__(self, applications: _Optional[_Iterable[_Union[Application, _Mapping]]] = ...) -> None: ...

class KeepAliveRequest(_message.Message):
    __slots__ = ("source_type", "hostname", "cluster_id", "ip")
    SOURCE_TYPE_FIELD_NUMBER: _ClassVar[int]
    HOSTNAME_FIELD_NUMBER: _ClassVar[int]
    CLUSTER_ID_FIELD_NUMBER: _ClassVar[int]
    IP_FIELD_NUMBER: _ClassVar[int]
    source_type: SourceType
    hostname: str
    cluster_id: int
    ip: str
    def __init__(self, source_type: _Optional[_Union[SourceType, str]] = ..., hostname: _Optional[str] = ..., cluster_id: _Optional[int] = ..., ip: _Optional[str] = ...) -> None: ...
