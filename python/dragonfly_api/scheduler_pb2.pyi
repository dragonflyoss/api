import datetime

from . import common_pb2 as _common_pb2
from . import errordetails_pb2 as _errordetails_pb2
from google.protobuf import duration_pb2 as _duration_pb2
from google.protobuf import empty_pb2 as _empty_pb2
from google.protobuf.internal import containers as _containers
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from collections.abc import Iterable as _Iterable, Mapping as _Mapping
from typing import ClassVar as _ClassVar, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class RegisterPeerRequest(_message.Message):
    __slots__ = ("download",)
    DOWNLOAD_FIELD_NUMBER: _ClassVar[int]
    download: _common_pb2.Download
    def __init__(self, download: _Optional[_Union[_common_pb2.Download, _Mapping]] = ...) -> None: ...

class DownloadPeerStartedRequest(_message.Message):
    __slots__ = ()
    def __init__(self) -> None: ...

class DownloadPeerBackToSourceStartedRequest(_message.Message):
    __slots__ = ("description",)
    DESCRIPTION_FIELD_NUMBER: _ClassVar[int]
    description: str
    def __init__(self, description: _Optional[str] = ...) -> None: ...

class ReschedulePeerRequest(_message.Message):
    __slots__ = ("candidate_parents", "description")
    CANDIDATE_PARENTS_FIELD_NUMBER: _ClassVar[int]
    DESCRIPTION_FIELD_NUMBER: _ClassVar[int]
    candidate_parents: _containers.RepeatedCompositeFieldContainer[_common_pb2.Peer]
    description: str
    def __init__(self, candidate_parents: _Optional[_Iterable[_Union[_common_pb2.Peer, _Mapping]]] = ..., description: _Optional[str] = ...) -> None: ...

class DownloadPeerFinishedRequest(_message.Message):
    __slots__ = ()
    def __init__(self) -> None: ...

class DownloadPeerBackToSourceFinishedRequest(_message.Message):
    __slots__ = ()
    def __init__(self) -> None: ...

class DownloadPeerFailedRequest(_message.Message):
    __slots__ = ("description",)
    DESCRIPTION_FIELD_NUMBER: _ClassVar[int]
    description: str
    def __init__(self, description: _Optional[str] = ...) -> None: ...

class DownloadPeerBackToSourceFailedRequest(_message.Message):
    __slots__ = ("description",)
    DESCRIPTION_FIELD_NUMBER: _ClassVar[int]
    description: str
    def __init__(self, description: _Optional[str] = ...) -> None: ...

class DownloadPieceFinishedRequest(_message.Message):
    __slots__ = ("piece",)
    PIECE_FIELD_NUMBER: _ClassVar[int]
    piece: _common_pb2.Piece
    def __init__(self, piece: _Optional[_Union[_common_pb2.Piece, _Mapping]] = ...) -> None: ...

class DownloadPieceBackToSourceFinishedRequest(_message.Message):
    __slots__ = ("piece",)
    PIECE_FIELD_NUMBER: _ClassVar[int]
    piece: _common_pb2.Piece
    def __init__(self, piece: _Optional[_Union[_common_pb2.Piece, _Mapping]] = ...) -> None: ...

class DownloadPieceFailedRequest(_message.Message):
    __slots__ = ("piece_number", "parent_id", "temporary")
    PIECE_NUMBER_FIELD_NUMBER: _ClassVar[int]
    PARENT_ID_FIELD_NUMBER: _ClassVar[int]
    TEMPORARY_FIELD_NUMBER: _ClassVar[int]
    piece_number: int
    parent_id: str
    temporary: bool
    def __init__(self, piece_number: _Optional[int] = ..., parent_id: _Optional[str] = ..., temporary: bool = ...) -> None: ...

class DownloadPieceBackToSourceFailedRequest(_message.Message):
    __slots__ = ("piece_number", "backend", "unknown")
    PIECE_NUMBER_FIELD_NUMBER: _ClassVar[int]
    BACKEND_FIELD_NUMBER: _ClassVar[int]
    UNKNOWN_FIELD_NUMBER: _ClassVar[int]
    piece_number: int
    backend: _errordetails_pb2.Backend
    unknown: _errordetails_pb2.Unknown
    def __init__(self, piece_number: _Optional[int] = ..., backend: _Optional[_Union[_errordetails_pb2.Backend, _Mapping]] = ..., unknown: _Optional[_Union[_errordetails_pb2.Unknown, _Mapping]] = ...) -> None: ...

class AnnouncePeerRequest(_message.Message):
    __slots__ = ("host_id", "task_id", "peer_id", "register_peer_request", "download_peer_started_request", "download_peer_back_to_source_started_request", "reschedule_peer_request", "download_peer_finished_request", "download_peer_back_to_source_finished_request", "download_peer_failed_request", "download_peer_back_to_source_failed_request", "download_piece_finished_request", "download_piece_back_to_source_finished_request", "download_piece_failed_request", "download_piece_back_to_source_failed_request")
    HOST_ID_FIELD_NUMBER: _ClassVar[int]
    TASK_ID_FIELD_NUMBER: _ClassVar[int]
    PEER_ID_FIELD_NUMBER: _ClassVar[int]
    REGISTER_PEER_REQUEST_FIELD_NUMBER: _ClassVar[int]
    DOWNLOAD_PEER_STARTED_REQUEST_FIELD_NUMBER: _ClassVar[int]
    DOWNLOAD_PEER_BACK_TO_SOURCE_STARTED_REQUEST_FIELD_NUMBER: _ClassVar[int]
    RESCHEDULE_PEER_REQUEST_FIELD_NUMBER: _ClassVar[int]
    DOWNLOAD_PEER_FINISHED_REQUEST_FIELD_NUMBER: _ClassVar[int]
    DOWNLOAD_PEER_BACK_TO_SOURCE_FINISHED_REQUEST_FIELD_NUMBER: _ClassVar[int]
    DOWNLOAD_PEER_FAILED_REQUEST_FIELD_NUMBER: _ClassVar[int]
    DOWNLOAD_PEER_BACK_TO_SOURCE_FAILED_REQUEST_FIELD_NUMBER: _ClassVar[int]
    DOWNLOAD_PIECE_FINISHED_REQUEST_FIELD_NUMBER: _ClassVar[int]
    DOWNLOAD_PIECE_BACK_TO_SOURCE_FINISHED_REQUEST_FIELD_NUMBER: _ClassVar[int]
    DOWNLOAD_PIECE_FAILED_REQUEST_FIELD_NUMBER: _ClassVar[int]
    DOWNLOAD_PIECE_BACK_TO_SOURCE_FAILED_REQUEST_FIELD_NUMBER: _ClassVar[int]
    host_id: str
    task_id: str
    peer_id: str
    register_peer_request: RegisterPeerRequest
    download_peer_started_request: DownloadPeerStartedRequest
    download_peer_back_to_source_started_request: DownloadPeerBackToSourceStartedRequest
    reschedule_peer_request: ReschedulePeerRequest
    download_peer_finished_request: DownloadPeerFinishedRequest
    download_peer_back_to_source_finished_request: DownloadPeerBackToSourceFinishedRequest
    download_peer_failed_request: DownloadPeerFailedRequest
    download_peer_back_to_source_failed_request: DownloadPeerBackToSourceFailedRequest
    download_piece_finished_request: DownloadPieceFinishedRequest
    download_piece_back_to_source_finished_request: DownloadPieceBackToSourceFinishedRequest
    download_piece_failed_request: DownloadPieceFailedRequest
    download_piece_back_to_source_failed_request: DownloadPieceBackToSourceFailedRequest
    def __init__(self, host_id: _Optional[str] = ..., task_id: _Optional[str] = ..., peer_id: _Optional[str] = ..., register_peer_request: _Optional[_Union[RegisterPeerRequest, _Mapping]] = ..., download_peer_started_request: _Optional[_Union[DownloadPeerStartedRequest, _Mapping]] = ..., download_peer_back_to_source_started_request: _Optional[_Union[DownloadPeerBackToSourceStartedRequest, _Mapping]] = ..., reschedule_peer_request: _Optional[_Union[ReschedulePeerRequest, _Mapping]] = ..., download_peer_finished_request: _Optional[_Union[DownloadPeerFinishedRequest, _Mapping]] = ..., download_peer_back_to_source_finished_request: _Optional[_Union[DownloadPeerBackToSourceFinishedRequest, _Mapping]] = ..., download_peer_failed_request: _Optional[_Union[DownloadPeerFailedRequest, _Mapping]] = ..., download_peer_back_to_source_failed_request: _Optional[_Union[DownloadPeerBackToSourceFailedRequest, _Mapping]] = ..., download_piece_finished_request: _Optional[_Union[DownloadPieceFinishedRequest, _Mapping]] = ..., download_piece_back_to_source_finished_request: _Optional[_Union[DownloadPieceBackToSourceFinishedRequest, _Mapping]] = ..., download_piece_failed_request: _Optional[_Union[DownloadPieceFailedRequest, _Mapping]] = ..., download_piece_back_to_source_failed_request: _Optional[_Union[DownloadPieceBackToSourceFailedRequest, _Mapping]] = ...) -> None: ...

class EmptyTaskResponse(_message.Message):
    __slots__ = ()
    def __init__(self) -> None: ...

class NormalTaskResponse(_message.Message):
    __slots__ = ("candidate_parents",)
    CANDIDATE_PARENTS_FIELD_NUMBER: _ClassVar[int]
    candidate_parents: _containers.RepeatedCompositeFieldContainer[_common_pb2.Peer]
    def __init__(self, candidate_parents: _Optional[_Iterable[_Union[_common_pb2.Peer, _Mapping]]] = ...) -> None: ...

class NeedBackToSourceResponse(_message.Message):
    __slots__ = ("description",)
    DESCRIPTION_FIELD_NUMBER: _ClassVar[int]
    description: str
    def __init__(self, description: _Optional[str] = ...) -> None: ...

class MetadataOnlyResponse(_message.Message):
    __slots__ = ()
    def __init__(self) -> None: ...

class AnnouncePeerResponse(_message.Message):
    __slots__ = ("empty_task_response", "normal_task_response", "need_back_to_source_response", "metadata_only_response")
    EMPTY_TASK_RESPONSE_FIELD_NUMBER: _ClassVar[int]
    NORMAL_TASK_RESPONSE_FIELD_NUMBER: _ClassVar[int]
    NEED_BACK_TO_SOURCE_RESPONSE_FIELD_NUMBER: _ClassVar[int]
    METADATA_ONLY_RESPONSE_FIELD_NUMBER: _ClassVar[int]
    empty_task_response: EmptyTaskResponse
    normal_task_response: NormalTaskResponse
    need_back_to_source_response: NeedBackToSourceResponse
    metadata_only_response: MetadataOnlyResponse
    def __init__(self, empty_task_response: _Optional[_Union[EmptyTaskResponse, _Mapping]] = ..., normal_task_response: _Optional[_Union[NormalTaskResponse, _Mapping]] = ..., need_back_to_source_response: _Optional[_Union[NeedBackToSourceResponse, _Mapping]] = ..., metadata_only_response: _Optional[_Union[MetadataOnlyResponse, _Mapping]] = ...) -> None: ...

class StatPeerRequest(_message.Message):
    __slots__ = ("host_id", "task_id", "peer_id")
    HOST_ID_FIELD_NUMBER: _ClassVar[int]
    TASK_ID_FIELD_NUMBER: _ClassVar[int]
    PEER_ID_FIELD_NUMBER: _ClassVar[int]
    host_id: str
    task_id: str
    peer_id: str
    def __init__(self, host_id: _Optional[str] = ..., task_id: _Optional[str] = ..., peer_id: _Optional[str] = ...) -> None: ...

class DeletePeerRequest(_message.Message):
    __slots__ = ("host_id", "task_id", "peer_id")
    HOST_ID_FIELD_NUMBER: _ClassVar[int]
    TASK_ID_FIELD_NUMBER: _ClassVar[int]
    PEER_ID_FIELD_NUMBER: _ClassVar[int]
    host_id: str
    task_id: str
    peer_id: str
    def __init__(self, host_id: _Optional[str] = ..., task_id: _Optional[str] = ..., peer_id: _Optional[str] = ...) -> None: ...

class StatTaskRequest(_message.Message):
    __slots__ = ("host_id", "task_id")
    HOST_ID_FIELD_NUMBER: _ClassVar[int]
    TASK_ID_FIELD_NUMBER: _ClassVar[int]
    host_id: str
    task_id: str
    def __init__(self, host_id: _Optional[str] = ..., task_id: _Optional[str] = ...) -> None: ...

class DeleteTaskRequest(_message.Message):
    __slots__ = ("host_id", "task_id")
    HOST_ID_FIELD_NUMBER: _ClassVar[int]
    TASK_ID_FIELD_NUMBER: _ClassVar[int]
    host_id: str
    task_id: str
    def __init__(self, host_id: _Optional[str] = ..., task_id: _Optional[str] = ...) -> None: ...

class AnnounceHostRequest(_message.Message):
    __slots__ = ("host", "interval")
    HOST_FIELD_NUMBER: _ClassVar[int]
    INTERVAL_FIELD_NUMBER: _ClassVar[int]
    host: _common_pb2.Host
    interval: _duration_pb2.Duration
    def __init__(self, host: _Optional[_Union[_common_pb2.Host, _Mapping]] = ..., interval: _Optional[_Union[datetime.timedelta, _duration_pb2.Duration, _Mapping]] = ...) -> None: ...

class ListHostsRequest(_message.Message):
    __slots__ = ("type",)
    TYPE_FIELD_NUMBER: _ClassVar[int]
    type: int
    def __init__(self, type: _Optional[int] = ...) -> None: ...

class ListHostsResponse(_message.Message):
    __slots__ = ("hosts",)
    HOSTS_FIELD_NUMBER: _ClassVar[int]
    hosts: _containers.RepeatedCompositeFieldContainer[_common_pb2.Host]
    def __init__(self, hosts: _Optional[_Iterable[_Union[_common_pb2.Host, _Mapping]]] = ...) -> None: ...

class DeleteHostRequest(_message.Message):
    __slots__ = ("host_id",)
    HOST_ID_FIELD_NUMBER: _ClassVar[int]
    host_id: str
    def __init__(self, host_id: _Optional[str] = ...) -> None: ...

class RegisterCachePeerRequest(_message.Message):
    __slots__ = ("url", "digest", "range", "type", "tag", "application", "priority", "filtered_query_params", "request_header", "piece_length", "output_path", "timeout", "disable_back_to_source", "need_back_to_source", "certificate_chain", "prefetch", "object_storage", "hdfs", "is_prefetch", "need_piece_content", "content_for_calculating_task_id", "remote_ip", "concurrent_piece_count", "actual_piece_length", "actual_content_length", "actual_piece_count")
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
    CONTENT_FOR_CALCULATING_TASK_ID_FIELD_NUMBER: _ClassVar[int]
    REMOTE_IP_FIELD_NUMBER: _ClassVar[int]
    CONCURRENT_PIECE_COUNT_FIELD_NUMBER: _ClassVar[int]
    ACTUAL_PIECE_LENGTH_FIELD_NUMBER: _ClassVar[int]
    ACTUAL_CONTENT_LENGTH_FIELD_NUMBER: _ClassVar[int]
    ACTUAL_PIECE_COUNT_FIELD_NUMBER: _ClassVar[int]
    url: str
    digest: str
    range: _common_pb2.Range
    type: _common_pb2.TaskType
    tag: str
    application: str
    priority: _common_pb2.Priority
    filtered_query_params: _containers.RepeatedScalarFieldContainer[str]
    request_header: _containers.ScalarMap[str, str]
    piece_length: int
    output_path: str
    timeout: _duration_pb2.Duration
    disable_back_to_source: bool
    need_back_to_source: bool
    certificate_chain: _containers.RepeatedScalarFieldContainer[bytes]
    prefetch: bool
    object_storage: _common_pb2.ObjectStorage
    hdfs: _common_pb2.HDFS
    is_prefetch: bool
    need_piece_content: bool
    content_for_calculating_task_id: str
    remote_ip: str
    concurrent_piece_count: int
    actual_piece_length: int
    actual_content_length: int
    actual_piece_count: int
    def __init__(self, url: _Optional[str] = ..., digest: _Optional[str] = ..., range: _Optional[_Union[_common_pb2.Range, _Mapping]] = ..., type: _Optional[_Union[_common_pb2.TaskType, str]] = ..., tag: _Optional[str] = ..., application: _Optional[str] = ..., priority: _Optional[_Union[_common_pb2.Priority, str]] = ..., filtered_query_params: _Optional[_Iterable[str]] = ..., request_header: _Optional[_Mapping[str, str]] = ..., piece_length: _Optional[int] = ..., output_path: _Optional[str] = ..., timeout: _Optional[_Union[datetime.timedelta, _duration_pb2.Duration, _Mapping]] = ..., disable_back_to_source: bool = ..., need_back_to_source: bool = ..., certificate_chain: _Optional[_Iterable[bytes]] = ..., prefetch: bool = ..., object_storage: _Optional[_Union[_common_pb2.ObjectStorage, _Mapping]] = ..., hdfs: _Optional[_Union[_common_pb2.HDFS, _Mapping]] = ..., is_prefetch: bool = ..., need_piece_content: bool = ..., content_for_calculating_task_id: _Optional[str] = ..., remote_ip: _Optional[str] = ..., concurrent_piece_count: _Optional[int] = ..., actual_piece_length: _Optional[int] = ..., actual_content_length: _Optional[int] = ..., actual_piece_count: _Optional[int] = ...) -> None: ...

class DownloadCachePeerStartedRequest(_message.Message):
    __slots__ = ()
    def __init__(self) -> None: ...

class DownloadCachePeerBackToSourceStartedRequest(_message.Message):
    __slots__ = ("description",)
    DESCRIPTION_FIELD_NUMBER: _ClassVar[int]
    description: str
    def __init__(self, description: _Optional[str] = ...) -> None: ...

class RescheduleCachePeerRequest(_message.Message):
    __slots__ = ("candidate_parents", "description")
    CANDIDATE_PARENTS_FIELD_NUMBER: _ClassVar[int]
    DESCRIPTION_FIELD_NUMBER: _ClassVar[int]
    candidate_parents: _containers.RepeatedCompositeFieldContainer[_common_pb2.CachePeer]
    description: str
    def __init__(self, candidate_parents: _Optional[_Iterable[_Union[_common_pb2.CachePeer, _Mapping]]] = ..., description: _Optional[str] = ...) -> None: ...

class DownloadCachePeerFinishedRequest(_message.Message):
    __slots__ = ()
    def __init__(self) -> None: ...

class DownloadCachePeerBackToSourceFinishedRequest(_message.Message):
    __slots__ = ()
    def __init__(self) -> None: ...

class DownloadCachePeerFailedRequest(_message.Message):
    __slots__ = ("description",)
    DESCRIPTION_FIELD_NUMBER: _ClassVar[int]
    description: str
    def __init__(self, description: _Optional[str] = ...) -> None: ...

class DownloadCachePeerBackToSourceFailedRequest(_message.Message):
    __slots__ = ("description",)
    DESCRIPTION_FIELD_NUMBER: _ClassVar[int]
    description: str
    def __init__(self, description: _Optional[str] = ...) -> None: ...

class AnnounceCachePeerRequest(_message.Message):
    __slots__ = ("host_id", "task_id", "peer_id", "register_cache_peer_request", "download_cache_peer_started_request", "download_cache_peer_back_to_source_started_request", "reschedule_cache_peer_request", "download_cache_peer_finished_request", "download_cache_peer_back_to_source_finished_request", "download_cache_peer_failed_request", "download_cache_peer_back_to_source_failed_request", "download_piece_finished_request", "download_piece_back_to_source_finished_request", "download_piece_failed_request", "download_piece_back_to_source_failed_request")
    HOST_ID_FIELD_NUMBER: _ClassVar[int]
    TASK_ID_FIELD_NUMBER: _ClassVar[int]
    PEER_ID_FIELD_NUMBER: _ClassVar[int]
    REGISTER_CACHE_PEER_REQUEST_FIELD_NUMBER: _ClassVar[int]
    DOWNLOAD_CACHE_PEER_STARTED_REQUEST_FIELD_NUMBER: _ClassVar[int]
    DOWNLOAD_CACHE_PEER_BACK_TO_SOURCE_STARTED_REQUEST_FIELD_NUMBER: _ClassVar[int]
    RESCHEDULE_CACHE_PEER_REQUEST_FIELD_NUMBER: _ClassVar[int]
    DOWNLOAD_CACHE_PEER_FINISHED_REQUEST_FIELD_NUMBER: _ClassVar[int]
    DOWNLOAD_CACHE_PEER_BACK_TO_SOURCE_FINISHED_REQUEST_FIELD_NUMBER: _ClassVar[int]
    DOWNLOAD_CACHE_PEER_FAILED_REQUEST_FIELD_NUMBER: _ClassVar[int]
    DOWNLOAD_CACHE_PEER_BACK_TO_SOURCE_FAILED_REQUEST_FIELD_NUMBER: _ClassVar[int]
    DOWNLOAD_PIECE_FINISHED_REQUEST_FIELD_NUMBER: _ClassVar[int]
    DOWNLOAD_PIECE_BACK_TO_SOURCE_FINISHED_REQUEST_FIELD_NUMBER: _ClassVar[int]
    DOWNLOAD_PIECE_FAILED_REQUEST_FIELD_NUMBER: _ClassVar[int]
    DOWNLOAD_PIECE_BACK_TO_SOURCE_FAILED_REQUEST_FIELD_NUMBER: _ClassVar[int]
    host_id: str
    task_id: str
    peer_id: str
    register_cache_peer_request: RegisterCachePeerRequest
    download_cache_peer_started_request: DownloadCachePeerStartedRequest
    download_cache_peer_back_to_source_started_request: DownloadCachePeerBackToSourceStartedRequest
    reschedule_cache_peer_request: RescheduleCachePeerRequest
    download_cache_peer_finished_request: DownloadCachePeerFinishedRequest
    download_cache_peer_back_to_source_finished_request: DownloadCachePeerBackToSourceFinishedRequest
    download_cache_peer_failed_request: DownloadCachePeerFailedRequest
    download_cache_peer_back_to_source_failed_request: DownloadCachePeerBackToSourceFailedRequest
    download_piece_finished_request: DownloadPieceFinishedRequest
    download_piece_back_to_source_finished_request: DownloadPieceBackToSourceFinishedRequest
    download_piece_failed_request: DownloadPieceFailedRequest
    download_piece_back_to_source_failed_request: DownloadPieceBackToSourceFailedRequest
    def __init__(self, host_id: _Optional[str] = ..., task_id: _Optional[str] = ..., peer_id: _Optional[str] = ..., register_cache_peer_request: _Optional[_Union[RegisterCachePeerRequest, _Mapping]] = ..., download_cache_peer_started_request: _Optional[_Union[DownloadCachePeerStartedRequest, _Mapping]] = ..., download_cache_peer_back_to_source_started_request: _Optional[_Union[DownloadCachePeerBackToSourceStartedRequest, _Mapping]] = ..., reschedule_cache_peer_request: _Optional[_Union[RescheduleCachePeerRequest, _Mapping]] = ..., download_cache_peer_finished_request: _Optional[_Union[DownloadCachePeerFinishedRequest, _Mapping]] = ..., download_cache_peer_back_to_source_finished_request: _Optional[_Union[DownloadCachePeerBackToSourceFinishedRequest, _Mapping]] = ..., download_cache_peer_failed_request: _Optional[_Union[DownloadCachePeerFailedRequest, _Mapping]] = ..., download_cache_peer_back_to_source_failed_request: _Optional[_Union[DownloadCachePeerBackToSourceFailedRequest, _Mapping]] = ..., download_piece_finished_request: _Optional[_Union[DownloadPieceFinishedRequest, _Mapping]] = ..., download_piece_back_to_source_finished_request: _Optional[_Union[DownloadPieceBackToSourceFinishedRequest, _Mapping]] = ..., download_piece_failed_request: _Optional[_Union[DownloadPieceFailedRequest, _Mapping]] = ..., download_piece_back_to_source_failed_request: _Optional[_Union[DownloadPieceBackToSourceFailedRequest, _Mapping]] = ...) -> None: ...

class EmptyCacheTaskResponse(_message.Message):
    __slots__ = ()
    def __init__(self) -> None: ...

class NormalCacheTaskResponse(_message.Message):
    __slots__ = ("candidate_parents",)
    CANDIDATE_PARENTS_FIELD_NUMBER: _ClassVar[int]
    candidate_parents: _containers.RepeatedCompositeFieldContainer[_common_pb2.CachePeer]
    def __init__(self, candidate_parents: _Optional[_Iterable[_Union[_common_pb2.CachePeer, _Mapping]]] = ...) -> None: ...

class AnnounceCachePeerResponse(_message.Message):
    __slots__ = ("empty_cache_task_response", "normal_cache_task_response", "need_back_to_source_response")
    EMPTY_CACHE_TASK_RESPONSE_FIELD_NUMBER: _ClassVar[int]
    NORMAL_CACHE_TASK_RESPONSE_FIELD_NUMBER: _ClassVar[int]
    NEED_BACK_TO_SOURCE_RESPONSE_FIELD_NUMBER: _ClassVar[int]
    empty_cache_task_response: EmptyCacheTaskResponse
    normal_cache_task_response: NormalCacheTaskResponse
    need_back_to_source_response: NeedBackToSourceResponse
    def __init__(self, empty_cache_task_response: _Optional[_Union[EmptyCacheTaskResponse, _Mapping]] = ..., normal_cache_task_response: _Optional[_Union[NormalCacheTaskResponse, _Mapping]] = ..., need_back_to_source_response: _Optional[_Union[NeedBackToSourceResponse, _Mapping]] = ...) -> None: ...

class StatCachePeerRequest(_message.Message):
    __slots__ = ("host_id", "task_id", "peer_id")
    HOST_ID_FIELD_NUMBER: _ClassVar[int]
    TASK_ID_FIELD_NUMBER: _ClassVar[int]
    PEER_ID_FIELD_NUMBER: _ClassVar[int]
    host_id: str
    task_id: str
    peer_id: str
    def __init__(self, host_id: _Optional[str] = ..., task_id: _Optional[str] = ..., peer_id: _Optional[str] = ...) -> None: ...

class DeleteCachePeerRequest(_message.Message):
    __slots__ = ("host_id", "task_id", "peer_id")
    HOST_ID_FIELD_NUMBER: _ClassVar[int]
    TASK_ID_FIELD_NUMBER: _ClassVar[int]
    PEER_ID_FIELD_NUMBER: _ClassVar[int]
    host_id: str
    task_id: str
    peer_id: str
    def __init__(self, host_id: _Optional[str] = ..., task_id: _Optional[str] = ..., peer_id: _Optional[str] = ...) -> None: ...

class StatCacheTaskRequest(_message.Message):
    __slots__ = ("host_id", "task_id")
    HOST_ID_FIELD_NUMBER: _ClassVar[int]
    TASK_ID_FIELD_NUMBER: _ClassVar[int]
    host_id: str
    task_id: str
    def __init__(self, host_id: _Optional[str] = ..., task_id: _Optional[str] = ...) -> None: ...

class DeleteCacheTaskRequest(_message.Message):
    __slots__ = ("host_id", "task_id")
    HOST_ID_FIELD_NUMBER: _ClassVar[int]
    TASK_ID_FIELD_NUMBER: _ClassVar[int]
    host_id: str
    task_id: str
    def __init__(self, host_id: _Optional[str] = ..., task_id: _Optional[str] = ...) -> None: ...

class RegisterPersistentPeerRequest(_message.Message):
    __slots__ = ("url", "object_storage", "persistent", "output_path", "concurrent_piece_count", "piece_count", "need_back_to_source")
    URL_FIELD_NUMBER: _ClassVar[int]
    OBJECT_STORAGE_FIELD_NUMBER: _ClassVar[int]
    PERSISTENT_FIELD_NUMBER: _ClassVar[int]
    OUTPUT_PATH_FIELD_NUMBER: _ClassVar[int]
    CONCURRENT_PIECE_COUNT_FIELD_NUMBER: _ClassVar[int]
    PIECE_COUNT_FIELD_NUMBER: _ClassVar[int]
    NEED_BACK_TO_SOURCE_FIELD_NUMBER: _ClassVar[int]
    url: str
    object_storage: _common_pb2.ObjectStorage
    persistent: bool
    output_path: str
    concurrent_piece_count: int
    piece_count: int
    need_back_to_source: bool
    def __init__(self, url: _Optional[str] = ..., object_storage: _Optional[_Union[_common_pb2.ObjectStorage, _Mapping]] = ..., persistent: bool = ..., output_path: _Optional[str] = ..., concurrent_piece_count: _Optional[int] = ..., piece_count: _Optional[int] = ..., need_back_to_source: bool = ...) -> None: ...

class DownloadPersistentPeerStartedRequest(_message.Message):
    __slots__ = ()
    def __init__(self) -> None: ...

class DownloadPersistentPeerBackToSourceStartedRequest(_message.Message):
    __slots__ = ("description",)
    DESCRIPTION_FIELD_NUMBER: _ClassVar[int]
    description: str
    def __init__(self, description: _Optional[str] = ...) -> None: ...

class ReschedulePersistentPeerRequest(_message.Message):
    __slots__ = ("candidate_parents", "description")
    CANDIDATE_PARENTS_FIELD_NUMBER: _ClassVar[int]
    DESCRIPTION_FIELD_NUMBER: _ClassVar[int]
    candidate_parents: _containers.RepeatedCompositeFieldContainer[_common_pb2.PersistentPeer]
    description: str
    def __init__(self, candidate_parents: _Optional[_Iterable[_Union[_common_pb2.PersistentPeer, _Mapping]]] = ..., description: _Optional[str] = ...) -> None: ...

class DownloadPersistentPeerFinishedRequest(_message.Message):
    __slots__ = ()
    def __init__(self) -> None: ...

class DownloadPersistentPeerBackToSourceFinishedRequest(_message.Message):
    __slots__ = ()
    def __init__(self) -> None: ...

class DownloadPersistentPeerFailedRequest(_message.Message):
    __slots__ = ("description",)
    DESCRIPTION_FIELD_NUMBER: _ClassVar[int]
    description: str
    def __init__(self, description: _Optional[str] = ...) -> None: ...

class DownloadPersistentPeerBackToSourceFailedRequest(_message.Message):
    __slots__ = ("description",)
    DESCRIPTION_FIELD_NUMBER: _ClassVar[int]
    description: str
    def __init__(self, description: _Optional[str] = ...) -> None: ...

class AnnouncePersistentPeerRequest(_message.Message):
    __slots__ = ("host_id", "task_id", "peer_id", "register_persistent_peer_request", "download_persistent_peer_started_request", "download_persistent_peer_back_to_source_started_request", "reschedule_persistent_peer_request", "download_persistent_peer_finished_request", "download_persistent_peer_back_to_source_finished_request", "download_persistent_peer_failed_request", "download_persistent_peer_back_to_source_failed_request", "download_piece_finished_request", "download_piece_back_to_source_finished_request", "download_piece_failed_request", "download_piece_back_to_source_failed_request")
    HOST_ID_FIELD_NUMBER: _ClassVar[int]
    TASK_ID_FIELD_NUMBER: _ClassVar[int]
    PEER_ID_FIELD_NUMBER: _ClassVar[int]
    REGISTER_PERSISTENT_PEER_REQUEST_FIELD_NUMBER: _ClassVar[int]
    DOWNLOAD_PERSISTENT_PEER_STARTED_REQUEST_FIELD_NUMBER: _ClassVar[int]
    DOWNLOAD_PERSISTENT_PEER_BACK_TO_SOURCE_STARTED_REQUEST_FIELD_NUMBER: _ClassVar[int]
    RESCHEDULE_PERSISTENT_PEER_REQUEST_FIELD_NUMBER: _ClassVar[int]
    DOWNLOAD_PERSISTENT_PEER_FINISHED_REQUEST_FIELD_NUMBER: _ClassVar[int]
    DOWNLOAD_PERSISTENT_PEER_BACK_TO_SOURCE_FINISHED_REQUEST_FIELD_NUMBER: _ClassVar[int]
    DOWNLOAD_PERSISTENT_PEER_FAILED_REQUEST_FIELD_NUMBER: _ClassVar[int]
    DOWNLOAD_PERSISTENT_PEER_BACK_TO_SOURCE_FAILED_REQUEST_FIELD_NUMBER: _ClassVar[int]
    DOWNLOAD_PIECE_FINISHED_REQUEST_FIELD_NUMBER: _ClassVar[int]
    DOWNLOAD_PIECE_BACK_TO_SOURCE_FINISHED_REQUEST_FIELD_NUMBER: _ClassVar[int]
    DOWNLOAD_PIECE_FAILED_REQUEST_FIELD_NUMBER: _ClassVar[int]
    DOWNLOAD_PIECE_BACK_TO_SOURCE_FAILED_REQUEST_FIELD_NUMBER: _ClassVar[int]
    host_id: str
    task_id: str
    peer_id: str
    register_persistent_peer_request: RegisterPersistentPeerRequest
    download_persistent_peer_started_request: DownloadPersistentPeerStartedRequest
    download_persistent_peer_back_to_source_started_request: DownloadPersistentPeerBackToSourceStartedRequest
    reschedule_persistent_peer_request: ReschedulePersistentPeerRequest
    download_persistent_peer_finished_request: DownloadPersistentPeerFinishedRequest
    download_persistent_peer_back_to_source_finished_request: DownloadPersistentPeerBackToSourceFinishedRequest
    download_persistent_peer_failed_request: DownloadPersistentPeerFailedRequest
    download_persistent_peer_back_to_source_failed_request: DownloadPersistentPeerBackToSourceFailedRequest
    download_piece_finished_request: DownloadPieceFinishedRequest
    download_piece_back_to_source_finished_request: DownloadPieceBackToSourceFinishedRequest
    download_piece_failed_request: DownloadPieceFailedRequest
    download_piece_back_to_source_failed_request: DownloadPieceBackToSourceFailedRequest
    def __init__(self, host_id: _Optional[str] = ..., task_id: _Optional[str] = ..., peer_id: _Optional[str] = ..., register_persistent_peer_request: _Optional[_Union[RegisterPersistentPeerRequest, _Mapping]] = ..., download_persistent_peer_started_request: _Optional[_Union[DownloadPersistentPeerStartedRequest, _Mapping]] = ..., download_persistent_peer_back_to_source_started_request: _Optional[_Union[DownloadPersistentPeerBackToSourceStartedRequest, _Mapping]] = ..., reschedule_persistent_peer_request: _Optional[_Union[ReschedulePersistentPeerRequest, _Mapping]] = ..., download_persistent_peer_finished_request: _Optional[_Union[DownloadPersistentPeerFinishedRequest, _Mapping]] = ..., download_persistent_peer_back_to_source_finished_request: _Optional[_Union[DownloadPersistentPeerBackToSourceFinishedRequest, _Mapping]] = ..., download_persistent_peer_failed_request: _Optional[_Union[DownloadPersistentPeerFailedRequest, _Mapping]] = ..., download_persistent_peer_back_to_source_failed_request: _Optional[_Union[DownloadPersistentPeerBackToSourceFailedRequest, _Mapping]] = ..., download_piece_finished_request: _Optional[_Union[DownloadPieceFinishedRequest, _Mapping]] = ..., download_piece_back_to_source_finished_request: _Optional[_Union[DownloadPieceBackToSourceFinishedRequest, _Mapping]] = ..., download_piece_failed_request: _Optional[_Union[DownloadPieceFailedRequest, _Mapping]] = ..., download_piece_back_to_source_failed_request: _Optional[_Union[DownloadPieceBackToSourceFailedRequest, _Mapping]] = ...) -> None: ...

class EmptyPersistentTaskResponse(_message.Message):
    __slots__ = ()
    def __init__(self) -> None: ...

class NormalPersistentTaskResponse(_message.Message):
    __slots__ = ("candidate_parents",)
    CANDIDATE_PARENTS_FIELD_NUMBER: _ClassVar[int]
    candidate_parents: _containers.RepeatedCompositeFieldContainer[_common_pb2.PersistentPeer]
    def __init__(self, candidate_parents: _Optional[_Iterable[_Union[_common_pb2.PersistentPeer, _Mapping]]] = ...) -> None: ...

class AnnouncePersistentPeerResponse(_message.Message):
    __slots__ = ("empty_persistent_task_response", "normal_persistent_task_response", "need_back_to_source_response")
    EMPTY_PERSISTENT_TASK_RESPONSE_FIELD_NUMBER: _ClassVar[int]
    NORMAL_PERSISTENT_TASK_RESPONSE_FIELD_NUMBER: _ClassVar[int]
    NEED_BACK_TO_SOURCE_RESPONSE_FIELD_NUMBER: _ClassVar[int]
    empty_persistent_task_response: EmptyPersistentTaskResponse
    normal_persistent_task_response: NormalPersistentTaskResponse
    need_back_to_source_response: NeedBackToSourceResponse
    def __init__(self, empty_persistent_task_response: _Optional[_Union[EmptyPersistentTaskResponse, _Mapping]] = ..., normal_persistent_task_response: _Optional[_Union[NormalPersistentTaskResponse, _Mapping]] = ..., need_back_to_source_response: _Optional[_Union[NeedBackToSourceResponse, _Mapping]] = ...) -> None: ...

class StatPersistentPeerRequest(_message.Message):
    __slots__ = ("host_id", "task_id", "peer_id")
    HOST_ID_FIELD_NUMBER: _ClassVar[int]
    TASK_ID_FIELD_NUMBER: _ClassVar[int]
    PEER_ID_FIELD_NUMBER: _ClassVar[int]
    host_id: str
    task_id: str
    peer_id: str
    def __init__(self, host_id: _Optional[str] = ..., task_id: _Optional[str] = ..., peer_id: _Optional[str] = ...) -> None: ...

class DeletePersistentPeerRequest(_message.Message):
    __slots__ = ("host_id", "task_id", "peer_id")
    HOST_ID_FIELD_NUMBER: _ClassVar[int]
    TASK_ID_FIELD_NUMBER: _ClassVar[int]
    PEER_ID_FIELD_NUMBER: _ClassVar[int]
    host_id: str
    task_id: str
    peer_id: str
    def __init__(self, host_id: _Optional[str] = ..., task_id: _Optional[str] = ..., peer_id: _Optional[str] = ...) -> None: ...

class UploadPersistentTaskStartedRequest(_message.Message):
    __slots__ = ("host_id", "task_id", "peer_id", "url", "object_storage", "persistent_replica_count", "content_length", "piece_count", "ttl")
    HOST_ID_FIELD_NUMBER: _ClassVar[int]
    TASK_ID_FIELD_NUMBER: _ClassVar[int]
    PEER_ID_FIELD_NUMBER: _ClassVar[int]
    URL_FIELD_NUMBER: _ClassVar[int]
    OBJECT_STORAGE_FIELD_NUMBER: _ClassVar[int]
    PERSISTENT_REPLICA_COUNT_FIELD_NUMBER: _ClassVar[int]
    CONTENT_LENGTH_FIELD_NUMBER: _ClassVar[int]
    PIECE_COUNT_FIELD_NUMBER: _ClassVar[int]
    TTL_FIELD_NUMBER: _ClassVar[int]
    host_id: str
    task_id: str
    peer_id: str
    url: str
    object_storage: _common_pb2.ObjectStorage
    persistent_replica_count: int
    content_length: int
    piece_count: int
    ttl: _duration_pb2.Duration
    def __init__(self, host_id: _Optional[str] = ..., task_id: _Optional[str] = ..., peer_id: _Optional[str] = ..., url: _Optional[str] = ..., object_storage: _Optional[_Union[_common_pb2.ObjectStorage, _Mapping]] = ..., persistent_replica_count: _Optional[int] = ..., content_length: _Optional[int] = ..., piece_count: _Optional[int] = ..., ttl: _Optional[_Union[datetime.timedelta, _duration_pb2.Duration, _Mapping]] = ...) -> None: ...

class UploadPersistentTaskFinishedRequest(_message.Message):
    __slots__ = ("host_id", "task_id", "peer_id")
    HOST_ID_FIELD_NUMBER: _ClassVar[int]
    TASK_ID_FIELD_NUMBER: _ClassVar[int]
    PEER_ID_FIELD_NUMBER: _ClassVar[int]
    host_id: str
    task_id: str
    peer_id: str
    def __init__(self, host_id: _Optional[str] = ..., task_id: _Optional[str] = ..., peer_id: _Optional[str] = ...) -> None: ...

class UploadPersistentTaskFailedRequest(_message.Message):
    __slots__ = ("host_id", "task_id", "peer_id", "description")
    HOST_ID_FIELD_NUMBER: _ClassVar[int]
    TASK_ID_FIELD_NUMBER: _ClassVar[int]
    PEER_ID_FIELD_NUMBER: _ClassVar[int]
    DESCRIPTION_FIELD_NUMBER: _ClassVar[int]
    host_id: str
    task_id: str
    peer_id: str
    description: str
    def __init__(self, host_id: _Optional[str] = ..., task_id: _Optional[str] = ..., peer_id: _Optional[str] = ..., description: _Optional[str] = ...) -> None: ...

class StatPersistentTaskRequest(_message.Message):
    __slots__ = ("host_id", "task_id")
    HOST_ID_FIELD_NUMBER: _ClassVar[int]
    TASK_ID_FIELD_NUMBER: _ClassVar[int]
    host_id: str
    task_id: str
    def __init__(self, host_id: _Optional[str] = ..., task_id: _Optional[str] = ...) -> None: ...

class DeletePersistentTaskRequest(_message.Message):
    __slots__ = ("host_id", "task_id")
    HOST_ID_FIELD_NUMBER: _ClassVar[int]
    TASK_ID_FIELD_NUMBER: _ClassVar[int]
    host_id: str
    task_id: str
    def __init__(self, host_id: _Optional[str] = ..., task_id: _Optional[str] = ...) -> None: ...

class RegisterPersistentCachePeerRequest(_message.Message):
    __slots__ = ("persistent", "tag", "application", "piece_length", "output_path", "concurrent_piece_count", "piece_count")
    PERSISTENT_FIELD_NUMBER: _ClassVar[int]
    TAG_FIELD_NUMBER: _ClassVar[int]
    APPLICATION_FIELD_NUMBER: _ClassVar[int]
    PIECE_LENGTH_FIELD_NUMBER: _ClassVar[int]
    OUTPUT_PATH_FIELD_NUMBER: _ClassVar[int]
    CONCURRENT_PIECE_COUNT_FIELD_NUMBER: _ClassVar[int]
    PIECE_COUNT_FIELD_NUMBER: _ClassVar[int]
    persistent: bool
    tag: str
    application: str
    piece_length: int
    output_path: str
    concurrent_piece_count: int
    piece_count: int
    def __init__(self, persistent: bool = ..., tag: _Optional[str] = ..., application: _Optional[str] = ..., piece_length: _Optional[int] = ..., output_path: _Optional[str] = ..., concurrent_piece_count: _Optional[int] = ..., piece_count: _Optional[int] = ...) -> None: ...

class DownloadPersistentCachePeerStartedRequest(_message.Message):
    __slots__ = ()
    def __init__(self) -> None: ...

class ReschedulePersistentCachePeerRequest(_message.Message):
    __slots__ = ("candidate_parents", "description")
    CANDIDATE_PARENTS_FIELD_NUMBER: _ClassVar[int]
    DESCRIPTION_FIELD_NUMBER: _ClassVar[int]
    candidate_parents: _containers.RepeatedCompositeFieldContainer[_common_pb2.PersistentCachePeer]
    description: str
    def __init__(self, candidate_parents: _Optional[_Iterable[_Union[_common_pb2.PersistentCachePeer, _Mapping]]] = ..., description: _Optional[str] = ...) -> None: ...

class DownloadPersistentCachePeerFinishedRequest(_message.Message):
    __slots__ = ()
    def __init__(self) -> None: ...

class DownloadPersistentCachePeerFailedRequest(_message.Message):
    __slots__ = ("description",)
    DESCRIPTION_FIELD_NUMBER: _ClassVar[int]
    description: str
    def __init__(self, description: _Optional[str] = ...) -> None: ...

class AnnouncePersistentCachePeerRequest(_message.Message):
    __slots__ = ("host_id", "task_id", "peer_id", "register_persistent_cache_peer_request", "download_persistent_cache_peer_started_request", "reschedule_persistent_cache_peer_request", "download_persistent_cache_peer_finished_request", "download_persistent_cache_peer_failed_request", "download_piece_finished_request", "download_piece_failed_request")
    HOST_ID_FIELD_NUMBER: _ClassVar[int]
    TASK_ID_FIELD_NUMBER: _ClassVar[int]
    PEER_ID_FIELD_NUMBER: _ClassVar[int]
    REGISTER_PERSISTENT_CACHE_PEER_REQUEST_FIELD_NUMBER: _ClassVar[int]
    DOWNLOAD_PERSISTENT_CACHE_PEER_STARTED_REQUEST_FIELD_NUMBER: _ClassVar[int]
    RESCHEDULE_PERSISTENT_CACHE_PEER_REQUEST_FIELD_NUMBER: _ClassVar[int]
    DOWNLOAD_PERSISTENT_CACHE_PEER_FINISHED_REQUEST_FIELD_NUMBER: _ClassVar[int]
    DOWNLOAD_PERSISTENT_CACHE_PEER_FAILED_REQUEST_FIELD_NUMBER: _ClassVar[int]
    DOWNLOAD_PIECE_FINISHED_REQUEST_FIELD_NUMBER: _ClassVar[int]
    DOWNLOAD_PIECE_FAILED_REQUEST_FIELD_NUMBER: _ClassVar[int]
    host_id: str
    task_id: str
    peer_id: str
    register_persistent_cache_peer_request: RegisterPersistentCachePeerRequest
    download_persistent_cache_peer_started_request: DownloadPersistentCachePeerStartedRequest
    reschedule_persistent_cache_peer_request: ReschedulePersistentCachePeerRequest
    download_persistent_cache_peer_finished_request: DownloadPersistentCachePeerFinishedRequest
    download_persistent_cache_peer_failed_request: DownloadPersistentCachePeerFailedRequest
    download_piece_finished_request: DownloadPieceFinishedRequest
    download_piece_failed_request: DownloadPieceFailedRequest
    def __init__(self, host_id: _Optional[str] = ..., task_id: _Optional[str] = ..., peer_id: _Optional[str] = ..., register_persistent_cache_peer_request: _Optional[_Union[RegisterPersistentCachePeerRequest, _Mapping]] = ..., download_persistent_cache_peer_started_request: _Optional[_Union[DownloadPersistentCachePeerStartedRequest, _Mapping]] = ..., reschedule_persistent_cache_peer_request: _Optional[_Union[ReschedulePersistentCachePeerRequest, _Mapping]] = ..., download_persistent_cache_peer_finished_request: _Optional[_Union[DownloadPersistentCachePeerFinishedRequest, _Mapping]] = ..., download_persistent_cache_peer_failed_request: _Optional[_Union[DownloadPersistentCachePeerFailedRequest, _Mapping]] = ..., download_piece_finished_request: _Optional[_Union[DownloadPieceFinishedRequest, _Mapping]] = ..., download_piece_failed_request: _Optional[_Union[DownloadPieceFailedRequest, _Mapping]] = ...) -> None: ...

class EmptyPersistentCacheTaskResponse(_message.Message):
    __slots__ = ()
    def __init__(self) -> None: ...

class NormalPersistentCacheTaskResponse(_message.Message):
    __slots__ = ("candidate_cache_parents",)
    CANDIDATE_CACHE_PARENTS_FIELD_NUMBER: _ClassVar[int]
    candidate_cache_parents: _containers.RepeatedCompositeFieldContainer[_common_pb2.PersistentCachePeer]
    def __init__(self, candidate_cache_parents: _Optional[_Iterable[_Union[_common_pb2.PersistentCachePeer, _Mapping]]] = ...) -> None: ...

class AnnouncePersistentCachePeerResponse(_message.Message):
    __slots__ = ("empty_persistent_cache_task_response", "normal_persistent_cache_task_response")
    EMPTY_PERSISTENT_CACHE_TASK_RESPONSE_FIELD_NUMBER: _ClassVar[int]
    NORMAL_PERSISTENT_CACHE_TASK_RESPONSE_FIELD_NUMBER: _ClassVar[int]
    empty_persistent_cache_task_response: EmptyPersistentCacheTaskResponse
    normal_persistent_cache_task_response: NormalPersistentCacheTaskResponse
    def __init__(self, empty_persistent_cache_task_response: _Optional[_Union[EmptyPersistentCacheTaskResponse, _Mapping]] = ..., normal_persistent_cache_task_response: _Optional[_Union[NormalPersistentCacheTaskResponse, _Mapping]] = ...) -> None: ...

class StatPersistentCachePeerRequest(_message.Message):
    __slots__ = ("host_id", "task_id", "peer_id")
    HOST_ID_FIELD_NUMBER: _ClassVar[int]
    TASK_ID_FIELD_NUMBER: _ClassVar[int]
    PEER_ID_FIELD_NUMBER: _ClassVar[int]
    host_id: str
    task_id: str
    peer_id: str
    def __init__(self, host_id: _Optional[str] = ..., task_id: _Optional[str] = ..., peer_id: _Optional[str] = ...) -> None: ...

class DeletePersistentCachePeerRequest(_message.Message):
    __slots__ = ("host_id", "task_id", "peer_id")
    HOST_ID_FIELD_NUMBER: _ClassVar[int]
    TASK_ID_FIELD_NUMBER: _ClassVar[int]
    PEER_ID_FIELD_NUMBER: _ClassVar[int]
    host_id: str
    task_id: str
    peer_id: str
    def __init__(self, host_id: _Optional[str] = ..., task_id: _Optional[str] = ..., peer_id: _Optional[str] = ...) -> None: ...

class UploadPersistentCacheTaskStartedRequest(_message.Message):
    __slots__ = ("host_id", "task_id", "peer_id", "persistent_replica_count", "tag", "application", "piece_length", "content_length", "piece_count", "ttl")
    HOST_ID_FIELD_NUMBER: _ClassVar[int]
    TASK_ID_FIELD_NUMBER: _ClassVar[int]
    PEER_ID_FIELD_NUMBER: _ClassVar[int]
    PERSISTENT_REPLICA_COUNT_FIELD_NUMBER: _ClassVar[int]
    TAG_FIELD_NUMBER: _ClassVar[int]
    APPLICATION_FIELD_NUMBER: _ClassVar[int]
    PIECE_LENGTH_FIELD_NUMBER: _ClassVar[int]
    CONTENT_LENGTH_FIELD_NUMBER: _ClassVar[int]
    PIECE_COUNT_FIELD_NUMBER: _ClassVar[int]
    TTL_FIELD_NUMBER: _ClassVar[int]
    host_id: str
    task_id: str
    peer_id: str
    persistent_replica_count: int
    tag: str
    application: str
    piece_length: int
    content_length: int
    piece_count: int
    ttl: _duration_pb2.Duration
    def __init__(self, host_id: _Optional[str] = ..., task_id: _Optional[str] = ..., peer_id: _Optional[str] = ..., persistent_replica_count: _Optional[int] = ..., tag: _Optional[str] = ..., application: _Optional[str] = ..., piece_length: _Optional[int] = ..., content_length: _Optional[int] = ..., piece_count: _Optional[int] = ..., ttl: _Optional[_Union[datetime.timedelta, _duration_pb2.Duration, _Mapping]] = ...) -> None: ...

class UploadPersistentCacheTaskFinishedRequest(_message.Message):
    __slots__ = ("host_id", "task_id", "peer_id")
    HOST_ID_FIELD_NUMBER: _ClassVar[int]
    TASK_ID_FIELD_NUMBER: _ClassVar[int]
    PEER_ID_FIELD_NUMBER: _ClassVar[int]
    host_id: str
    task_id: str
    peer_id: str
    def __init__(self, host_id: _Optional[str] = ..., task_id: _Optional[str] = ..., peer_id: _Optional[str] = ...) -> None: ...

class UploadPersistentCacheTaskFailedRequest(_message.Message):
    __slots__ = ("host_id", "task_id", "peer_id", "description")
    HOST_ID_FIELD_NUMBER: _ClassVar[int]
    TASK_ID_FIELD_NUMBER: _ClassVar[int]
    PEER_ID_FIELD_NUMBER: _ClassVar[int]
    DESCRIPTION_FIELD_NUMBER: _ClassVar[int]
    host_id: str
    task_id: str
    peer_id: str
    description: str
    def __init__(self, host_id: _Optional[str] = ..., task_id: _Optional[str] = ..., peer_id: _Optional[str] = ..., description: _Optional[str] = ...) -> None: ...

class StatPersistentCacheTaskRequest(_message.Message):
    __slots__ = ("host_id", "task_id")
    HOST_ID_FIELD_NUMBER: _ClassVar[int]
    TASK_ID_FIELD_NUMBER: _ClassVar[int]
    host_id: str
    task_id: str
    def __init__(self, host_id: _Optional[str] = ..., task_id: _Optional[str] = ...) -> None: ...

class DeletePersistentCacheTaskRequest(_message.Message):
    __slots__ = ("host_id", "task_id")
    HOST_ID_FIELD_NUMBER: _ClassVar[int]
    TASK_ID_FIELD_NUMBER: _ClassVar[int]
    host_id: str
    task_id: str
    def __init__(self, host_id: _Optional[str] = ..., task_id: _Optional[str] = ...) -> None: ...

class PreheatImageRequest(_message.Message):
    __slots__ = ("url", "piece_length", "tag", "application", "filtered_query_params", "header", "username", "password", "platform", "scope", "ips", "percentage", "count", "concurrent_task_count", "concurrent_peer_count", "timeout", "priority", "certificate_chain", "insecure_skip_verify")
    class HeaderEntry(_message.Message):
        __slots__ = ("key", "value")
        KEY_FIELD_NUMBER: _ClassVar[int]
        VALUE_FIELD_NUMBER: _ClassVar[int]
        key: str
        value: str
        def __init__(self, key: _Optional[str] = ..., value: _Optional[str] = ...) -> None: ...
    URL_FIELD_NUMBER: _ClassVar[int]
    PIECE_LENGTH_FIELD_NUMBER: _ClassVar[int]
    TAG_FIELD_NUMBER: _ClassVar[int]
    APPLICATION_FIELD_NUMBER: _ClassVar[int]
    FILTERED_QUERY_PARAMS_FIELD_NUMBER: _ClassVar[int]
    HEADER_FIELD_NUMBER: _ClassVar[int]
    USERNAME_FIELD_NUMBER: _ClassVar[int]
    PASSWORD_FIELD_NUMBER: _ClassVar[int]
    PLATFORM_FIELD_NUMBER: _ClassVar[int]
    SCOPE_FIELD_NUMBER: _ClassVar[int]
    IPS_FIELD_NUMBER: _ClassVar[int]
    PERCENTAGE_FIELD_NUMBER: _ClassVar[int]
    COUNT_FIELD_NUMBER: _ClassVar[int]
    CONCURRENT_TASK_COUNT_FIELD_NUMBER: _ClassVar[int]
    CONCURRENT_PEER_COUNT_FIELD_NUMBER: _ClassVar[int]
    TIMEOUT_FIELD_NUMBER: _ClassVar[int]
    PRIORITY_FIELD_NUMBER: _ClassVar[int]
    CERTIFICATE_CHAIN_FIELD_NUMBER: _ClassVar[int]
    INSECURE_SKIP_VERIFY_FIELD_NUMBER: _ClassVar[int]
    url: str
    piece_length: int
    tag: str
    application: str
    filtered_query_params: _containers.RepeatedScalarFieldContainer[str]
    header: _containers.ScalarMap[str, str]
    username: str
    password: str
    platform: str
    scope: str
    ips: _containers.RepeatedScalarFieldContainer[str]
    percentage: int
    count: int
    concurrent_task_count: int
    concurrent_peer_count: int
    timeout: _duration_pb2.Duration
    priority: _common_pb2.Priority
    certificate_chain: _containers.RepeatedScalarFieldContainer[bytes]
    insecure_skip_verify: bool
    def __init__(self, url: _Optional[str] = ..., piece_length: _Optional[int] = ..., tag: _Optional[str] = ..., application: _Optional[str] = ..., filtered_query_params: _Optional[_Iterable[str]] = ..., header: _Optional[_Mapping[str, str]] = ..., username: _Optional[str] = ..., password: _Optional[str] = ..., platform: _Optional[str] = ..., scope: _Optional[str] = ..., ips: _Optional[_Iterable[str]] = ..., percentage: _Optional[int] = ..., count: _Optional[int] = ..., concurrent_task_count: _Optional[int] = ..., concurrent_peer_count: _Optional[int] = ..., timeout: _Optional[_Union[datetime.timedelta, _duration_pb2.Duration, _Mapping]] = ..., priority: _Optional[_Union[_common_pb2.Priority, str]] = ..., certificate_chain: _Optional[_Iterable[bytes]] = ..., insecure_skip_verify: bool = ...) -> None: ...

class StatImageRequest(_message.Message):
    __slots__ = ("url", "piece_length", "tag", "application", "filtered_query_params", "header", "username", "password", "platform", "concurrent_layer_count", "concurrent_peer_count", "timeout", "certificate_chain", "insecure_skip_verify", "scope", "enable_task_id_based_blob_digest")
    class HeaderEntry(_message.Message):
        __slots__ = ("key", "value")
        KEY_FIELD_NUMBER: _ClassVar[int]
        VALUE_FIELD_NUMBER: _ClassVar[int]
        key: str
        value: str
        def __init__(self, key: _Optional[str] = ..., value: _Optional[str] = ...) -> None: ...
    URL_FIELD_NUMBER: _ClassVar[int]
    PIECE_LENGTH_FIELD_NUMBER: _ClassVar[int]
    TAG_FIELD_NUMBER: _ClassVar[int]
    APPLICATION_FIELD_NUMBER: _ClassVar[int]
    FILTERED_QUERY_PARAMS_FIELD_NUMBER: _ClassVar[int]
    HEADER_FIELD_NUMBER: _ClassVar[int]
    USERNAME_FIELD_NUMBER: _ClassVar[int]
    PASSWORD_FIELD_NUMBER: _ClassVar[int]
    PLATFORM_FIELD_NUMBER: _ClassVar[int]
    CONCURRENT_LAYER_COUNT_FIELD_NUMBER: _ClassVar[int]
    CONCURRENT_PEER_COUNT_FIELD_NUMBER: _ClassVar[int]
    TIMEOUT_FIELD_NUMBER: _ClassVar[int]
    CERTIFICATE_CHAIN_FIELD_NUMBER: _ClassVar[int]
    INSECURE_SKIP_VERIFY_FIELD_NUMBER: _ClassVar[int]
    SCOPE_FIELD_NUMBER: _ClassVar[int]
    ENABLE_TASK_ID_BASED_BLOB_DIGEST_FIELD_NUMBER: _ClassVar[int]
    url: str
    piece_length: int
    tag: str
    application: str
    filtered_query_params: _containers.RepeatedScalarFieldContainer[str]
    header: _containers.ScalarMap[str, str]
    username: str
    password: str
    platform: str
    concurrent_layer_count: int
    concurrent_peer_count: int
    timeout: _duration_pb2.Duration
    certificate_chain: _containers.RepeatedScalarFieldContainer[bytes]
    insecure_skip_verify: bool
    scope: str
    enable_task_id_based_blob_digest: bool
    def __init__(self, url: _Optional[str] = ..., piece_length: _Optional[int] = ..., tag: _Optional[str] = ..., application: _Optional[str] = ..., filtered_query_params: _Optional[_Iterable[str]] = ..., header: _Optional[_Mapping[str, str]] = ..., username: _Optional[str] = ..., password: _Optional[str] = ..., platform: _Optional[str] = ..., concurrent_layer_count: _Optional[int] = ..., concurrent_peer_count: _Optional[int] = ..., timeout: _Optional[_Union[datetime.timedelta, _duration_pb2.Duration, _Mapping]] = ..., certificate_chain: _Optional[_Iterable[bytes]] = ..., insecure_skip_verify: bool = ..., scope: _Optional[str] = ..., enable_task_id_based_blob_digest: bool = ...) -> None: ...

class StatImageResponse(_message.Message):
    __slots__ = ("image", "peers")
    IMAGE_FIELD_NUMBER: _ClassVar[int]
    PEERS_FIELD_NUMBER: _ClassVar[int]
    image: Image
    peers: _containers.RepeatedCompositeFieldContainer[PeerImage]
    def __init__(self, image: _Optional[_Union[Image, _Mapping]] = ..., peers: _Optional[_Iterable[_Union[PeerImage, _Mapping]]] = ...) -> None: ...

class PeerImage(_message.Message):
    __slots__ = ("ip", "hostname", "cached_layers")
    IP_FIELD_NUMBER: _ClassVar[int]
    HOSTNAME_FIELD_NUMBER: _ClassVar[int]
    CACHED_LAYERS_FIELD_NUMBER: _ClassVar[int]
    ip: str
    hostname: str
    cached_layers: _containers.RepeatedCompositeFieldContainer[Layer]
    def __init__(self, ip: _Optional[str] = ..., hostname: _Optional[str] = ..., cached_layers: _Optional[_Iterable[_Union[Layer, _Mapping]]] = ...) -> None: ...

class Image(_message.Message):
    __slots__ = ("layers",)
    LAYERS_FIELD_NUMBER: _ClassVar[int]
    layers: _containers.RepeatedCompositeFieldContainer[Layer]
    def __init__(self, layers: _Optional[_Iterable[_Union[Layer, _Mapping]]] = ...) -> None: ...

class Layer(_message.Message):
    __slots__ = ("url", "is_finished")
    URL_FIELD_NUMBER: _ClassVar[int]
    IS_FINISHED_FIELD_NUMBER: _ClassVar[int]
    url: str
    is_finished: bool
    def __init__(self, url: _Optional[str] = ..., is_finished: bool = ...) -> None: ...

class PreheatFileRequest(_message.Message):
    __slots__ = ("url", "piece_length", "tag", "application", "filtered_query_params", "header", "scope", "ips", "percentage", "count", "concurrent_task_count", "concurrent_peer_count", "timeout", "priority", "certificate_chain", "insecure_skip_verify", "object_storage", "hdfs", "output_path")
    class HeaderEntry(_message.Message):
        __slots__ = ("key", "value")
        KEY_FIELD_NUMBER: _ClassVar[int]
        VALUE_FIELD_NUMBER: _ClassVar[int]
        key: str
        value: str
        def __init__(self, key: _Optional[str] = ..., value: _Optional[str] = ...) -> None: ...
    URL_FIELD_NUMBER: _ClassVar[int]
    PIECE_LENGTH_FIELD_NUMBER: _ClassVar[int]
    TAG_FIELD_NUMBER: _ClassVar[int]
    APPLICATION_FIELD_NUMBER: _ClassVar[int]
    FILTERED_QUERY_PARAMS_FIELD_NUMBER: _ClassVar[int]
    HEADER_FIELD_NUMBER: _ClassVar[int]
    SCOPE_FIELD_NUMBER: _ClassVar[int]
    IPS_FIELD_NUMBER: _ClassVar[int]
    PERCENTAGE_FIELD_NUMBER: _ClassVar[int]
    COUNT_FIELD_NUMBER: _ClassVar[int]
    CONCURRENT_TASK_COUNT_FIELD_NUMBER: _ClassVar[int]
    CONCURRENT_PEER_COUNT_FIELD_NUMBER: _ClassVar[int]
    TIMEOUT_FIELD_NUMBER: _ClassVar[int]
    PRIORITY_FIELD_NUMBER: _ClassVar[int]
    CERTIFICATE_CHAIN_FIELD_NUMBER: _ClassVar[int]
    INSECURE_SKIP_VERIFY_FIELD_NUMBER: _ClassVar[int]
    OBJECT_STORAGE_FIELD_NUMBER: _ClassVar[int]
    HDFS_FIELD_NUMBER: _ClassVar[int]
    OUTPUT_PATH_FIELD_NUMBER: _ClassVar[int]
    url: str
    piece_length: int
    tag: str
    application: str
    filtered_query_params: _containers.RepeatedScalarFieldContainer[str]
    header: _containers.ScalarMap[str, str]
    scope: str
    ips: _containers.RepeatedScalarFieldContainer[str]
    percentage: int
    count: int
    concurrent_task_count: int
    concurrent_peer_count: int
    timeout: _duration_pb2.Duration
    priority: _common_pb2.Priority
    certificate_chain: _containers.RepeatedScalarFieldContainer[bytes]
    insecure_skip_verify: bool
    object_storage: _common_pb2.ObjectStorage
    hdfs: _common_pb2.HDFS
    output_path: str
    def __init__(self, url: _Optional[str] = ..., piece_length: _Optional[int] = ..., tag: _Optional[str] = ..., application: _Optional[str] = ..., filtered_query_params: _Optional[_Iterable[str]] = ..., header: _Optional[_Mapping[str, str]] = ..., scope: _Optional[str] = ..., ips: _Optional[_Iterable[str]] = ..., percentage: _Optional[int] = ..., count: _Optional[int] = ..., concurrent_task_count: _Optional[int] = ..., concurrent_peer_count: _Optional[int] = ..., timeout: _Optional[_Union[datetime.timedelta, _duration_pb2.Duration, _Mapping]] = ..., priority: _Optional[_Union[_common_pb2.Priority, str]] = ..., certificate_chain: _Optional[_Iterable[bytes]] = ..., insecure_skip_verify: bool = ..., object_storage: _Optional[_Union[_common_pb2.ObjectStorage, _Mapping]] = ..., hdfs: _Optional[_Union[_common_pb2.HDFS, _Mapping]] = ..., output_path: _Optional[str] = ...) -> None: ...

class StatFileRequest(_message.Message):
    __slots__ = ("url", "piece_length", "tag", "application", "filtered_query_params", "header", "concurrent_peer_count", "timeout", "certificate_chain", "insecure_skip_verify", "object_storage", "hdfs")
    class HeaderEntry(_message.Message):
        __slots__ = ("key", "value")
        KEY_FIELD_NUMBER: _ClassVar[int]
        VALUE_FIELD_NUMBER: _ClassVar[int]
        key: str
        value: str
        def __init__(self, key: _Optional[str] = ..., value: _Optional[str] = ...) -> None: ...
    URL_FIELD_NUMBER: _ClassVar[int]
    PIECE_LENGTH_FIELD_NUMBER: _ClassVar[int]
    TAG_FIELD_NUMBER: _ClassVar[int]
    APPLICATION_FIELD_NUMBER: _ClassVar[int]
    FILTERED_QUERY_PARAMS_FIELD_NUMBER: _ClassVar[int]
    HEADER_FIELD_NUMBER: _ClassVar[int]
    CONCURRENT_PEER_COUNT_FIELD_NUMBER: _ClassVar[int]
    TIMEOUT_FIELD_NUMBER: _ClassVar[int]
    CERTIFICATE_CHAIN_FIELD_NUMBER: _ClassVar[int]
    INSECURE_SKIP_VERIFY_FIELD_NUMBER: _ClassVar[int]
    OBJECT_STORAGE_FIELD_NUMBER: _ClassVar[int]
    HDFS_FIELD_NUMBER: _ClassVar[int]
    url: str
    piece_length: int
    tag: str
    application: str
    filtered_query_params: _containers.RepeatedScalarFieldContainer[str]
    header: _containers.ScalarMap[str, str]
    concurrent_peer_count: int
    timeout: _duration_pb2.Duration
    certificate_chain: _containers.RepeatedScalarFieldContainer[bytes]
    insecure_skip_verify: bool
    object_storage: _common_pb2.ObjectStorage
    hdfs: _common_pb2.HDFS
    def __init__(self, url: _Optional[str] = ..., piece_length: _Optional[int] = ..., tag: _Optional[str] = ..., application: _Optional[str] = ..., filtered_query_params: _Optional[_Iterable[str]] = ..., header: _Optional[_Mapping[str, str]] = ..., concurrent_peer_count: _Optional[int] = ..., timeout: _Optional[_Union[datetime.timedelta, _duration_pb2.Duration, _Mapping]] = ..., certificate_chain: _Optional[_Iterable[bytes]] = ..., insecure_skip_verify: bool = ..., object_storage: _Optional[_Union[_common_pb2.ObjectStorage, _Mapping]] = ..., hdfs: _Optional[_Union[_common_pb2.HDFS, _Mapping]] = ...) -> None: ...

class StatFileResponse(_message.Message):
    __slots__ = ("peers",)
    PEERS_FIELD_NUMBER: _ClassVar[int]
    peers: _containers.RepeatedCompositeFieldContainer[PeerFile]
    def __init__(self, peers: _Optional[_Iterable[_Union[PeerFile, _Mapping]]] = ...) -> None: ...

class PeerFile(_message.Message):
    __slots__ = ("ip", "hostname", "cached_files")
    IP_FIELD_NUMBER: _ClassVar[int]
    HOSTNAME_FIELD_NUMBER: _ClassVar[int]
    CACHED_FILES_FIELD_NUMBER: _ClassVar[int]
    ip: str
    hostname: str
    cached_files: _containers.RepeatedCompositeFieldContainer[File]
    def __init__(self, ip: _Optional[str] = ..., hostname: _Optional[str] = ..., cached_files: _Optional[_Iterable[_Union[File, _Mapping]]] = ...) -> None: ...

class File(_message.Message):
    __slots__ = ("url", "is_finished")
    URL_FIELD_NUMBER: _ClassVar[int]
    IS_FINISHED_FIELD_NUMBER: _ClassVar[int]
    url: str
    is_finished: bool
    def __init__(self, url: _Optional[str] = ..., is_finished: bool = ...) -> None: ...
