import datetime

from . import common_pb2 as _common_pb2
from google.protobuf import duration_pb2 as _duration_pb2
from google.protobuf import empty_pb2 as _empty_pb2
from google.protobuf import timestamp_pb2 as _timestamp_pb2
from google.protobuf.internal import containers as _containers
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from collections.abc import Iterable as _Iterable, Mapping as _Mapping
from typing import ClassVar as _ClassVar, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class DownloadTaskRequest(_message.Message):
    __slots__ = ("download",)
    DOWNLOAD_FIELD_NUMBER: _ClassVar[int]
    download: _common_pb2.Download
    def __init__(self, download: _Optional[_Union[_common_pb2.Download, _Mapping]] = ...) -> None: ...

class DownloadTaskStartedResponse(_message.Message):
    __slots__ = ("content_length", "range", "response_header", "pieces", "is_finished")
    class ResponseHeaderEntry(_message.Message):
        __slots__ = ("key", "value")
        KEY_FIELD_NUMBER: _ClassVar[int]
        VALUE_FIELD_NUMBER: _ClassVar[int]
        key: str
        value: str
        def __init__(self, key: _Optional[str] = ..., value: _Optional[str] = ...) -> None: ...
    CONTENT_LENGTH_FIELD_NUMBER: _ClassVar[int]
    RANGE_FIELD_NUMBER: _ClassVar[int]
    RESPONSE_HEADER_FIELD_NUMBER: _ClassVar[int]
    PIECES_FIELD_NUMBER: _ClassVar[int]
    IS_FINISHED_FIELD_NUMBER: _ClassVar[int]
    content_length: int
    range: _common_pb2.Range
    response_header: _containers.ScalarMap[str, str]
    pieces: _containers.RepeatedCompositeFieldContainer[_common_pb2.Piece]
    is_finished: bool
    def __init__(self, content_length: _Optional[int] = ..., range: _Optional[_Union[_common_pb2.Range, _Mapping]] = ..., response_header: _Optional[_Mapping[str, str]] = ..., pieces: _Optional[_Iterable[_Union[_common_pb2.Piece, _Mapping]]] = ..., is_finished: bool = ...) -> None: ...

class DownloadPieceFinishedResponse(_message.Message):
    __slots__ = ("piece",)
    PIECE_FIELD_NUMBER: _ClassVar[int]
    piece: _common_pb2.Piece
    def __init__(self, piece: _Optional[_Union[_common_pb2.Piece, _Mapping]] = ...) -> None: ...

class DownloadTaskResponse(_message.Message):
    __slots__ = ("host_id", "task_id", "peer_id", "download_task_started_response", "download_piece_finished_response")
    HOST_ID_FIELD_NUMBER: _ClassVar[int]
    TASK_ID_FIELD_NUMBER: _ClassVar[int]
    PEER_ID_FIELD_NUMBER: _ClassVar[int]
    DOWNLOAD_TASK_STARTED_RESPONSE_FIELD_NUMBER: _ClassVar[int]
    DOWNLOAD_PIECE_FINISHED_RESPONSE_FIELD_NUMBER: _ClassVar[int]
    host_id: str
    task_id: str
    peer_id: str
    download_task_started_response: DownloadTaskStartedResponse
    download_piece_finished_response: DownloadPieceFinishedResponse
    def __init__(self, host_id: _Optional[str] = ..., task_id: _Optional[str] = ..., peer_id: _Optional[str] = ..., download_task_started_response: _Optional[_Union[DownloadTaskStartedResponse, _Mapping]] = ..., download_piece_finished_response: _Optional[_Union[DownloadPieceFinishedResponse, _Mapping]] = ...) -> None: ...

class SyncPiecesRequest(_message.Message):
    __slots__ = ("host_id", "task_id", "interested_piece_numbers")
    HOST_ID_FIELD_NUMBER: _ClassVar[int]
    TASK_ID_FIELD_NUMBER: _ClassVar[int]
    INTERESTED_PIECE_NUMBERS_FIELD_NUMBER: _ClassVar[int]
    host_id: str
    task_id: str
    interested_piece_numbers: _containers.RepeatedScalarFieldContainer[int]
    def __init__(self, host_id: _Optional[str] = ..., task_id: _Optional[str] = ..., interested_piece_numbers: _Optional[_Iterable[int]] = ...) -> None: ...

class SyncPiecesResponse(_message.Message):
    __slots__ = ("number", "offset", "length", "ip", "tcp_port", "quic_port")
    NUMBER_FIELD_NUMBER: _ClassVar[int]
    OFFSET_FIELD_NUMBER: _ClassVar[int]
    LENGTH_FIELD_NUMBER: _ClassVar[int]
    IP_FIELD_NUMBER: _ClassVar[int]
    TCP_PORT_FIELD_NUMBER: _ClassVar[int]
    QUIC_PORT_FIELD_NUMBER: _ClassVar[int]
    number: int
    offset: int
    length: int
    ip: str
    tcp_port: int
    quic_port: int
    def __init__(self, number: _Optional[int] = ..., offset: _Optional[int] = ..., length: _Optional[int] = ..., ip: _Optional[str] = ..., tcp_port: _Optional[int] = ..., quic_port: _Optional[int] = ...) -> None: ...

class StatTaskRequest(_message.Message):
    __slots__ = ("task_id", "remote_ip")
    TASK_ID_FIELD_NUMBER: _ClassVar[int]
    REMOTE_IP_FIELD_NUMBER: _ClassVar[int]
    task_id: str
    remote_ip: str
    def __init__(self, task_id: _Optional[str] = ..., remote_ip: _Optional[str] = ...) -> None: ...

class StatLocalTaskRequest(_message.Message):
    __slots__ = ("task_id", "remote_ip")
    TASK_ID_FIELD_NUMBER: _ClassVar[int]
    REMOTE_IP_FIELD_NUMBER: _ClassVar[int]
    task_id: str
    remote_ip: str
    def __init__(self, task_id: _Optional[str] = ..., remote_ip: _Optional[str] = ...) -> None: ...

class StatLocalTaskResponse(_message.Message):
    __slots__ = ("task_id", "piece_length", "content_length", "response_header", "uploading_count", "uploaded_count", "created_at", "updated_at", "prefetched_at", "failed_at", "finished_at")
    class ResponseHeaderEntry(_message.Message):
        __slots__ = ("key", "value")
        KEY_FIELD_NUMBER: _ClassVar[int]
        VALUE_FIELD_NUMBER: _ClassVar[int]
        key: str
        value: str
        def __init__(self, key: _Optional[str] = ..., value: _Optional[str] = ...) -> None: ...
    TASK_ID_FIELD_NUMBER: _ClassVar[int]
    PIECE_LENGTH_FIELD_NUMBER: _ClassVar[int]
    CONTENT_LENGTH_FIELD_NUMBER: _ClassVar[int]
    RESPONSE_HEADER_FIELD_NUMBER: _ClassVar[int]
    UPLOADING_COUNT_FIELD_NUMBER: _ClassVar[int]
    UPLOADED_COUNT_FIELD_NUMBER: _ClassVar[int]
    CREATED_AT_FIELD_NUMBER: _ClassVar[int]
    UPDATED_AT_FIELD_NUMBER: _ClassVar[int]
    PREFETCHED_AT_FIELD_NUMBER: _ClassVar[int]
    FAILED_AT_FIELD_NUMBER: _ClassVar[int]
    FINISHED_AT_FIELD_NUMBER: _ClassVar[int]
    task_id: str
    piece_length: int
    content_length: int
    response_header: _containers.ScalarMap[str, str]
    uploading_count: int
    uploaded_count: int
    created_at: _timestamp_pb2.Timestamp
    updated_at: _timestamp_pb2.Timestamp
    prefetched_at: _timestamp_pb2.Timestamp
    failed_at: _timestamp_pb2.Timestamp
    finished_at: _timestamp_pb2.Timestamp
    def __init__(self, task_id: _Optional[str] = ..., piece_length: _Optional[int] = ..., content_length: _Optional[int] = ..., response_header: _Optional[_Mapping[str, str]] = ..., uploading_count: _Optional[int] = ..., uploaded_count: _Optional[int] = ..., created_at: _Optional[_Union[datetime.datetime, _timestamp_pb2.Timestamp, _Mapping]] = ..., updated_at: _Optional[_Union[datetime.datetime, _timestamp_pb2.Timestamp, _Mapping]] = ..., prefetched_at: _Optional[_Union[datetime.datetime, _timestamp_pb2.Timestamp, _Mapping]] = ..., failed_at: _Optional[_Union[datetime.datetime, _timestamp_pb2.Timestamp, _Mapping]] = ..., finished_at: _Optional[_Union[datetime.datetime, _timestamp_pb2.Timestamp, _Mapping]] = ...) -> None: ...

class ListLocalTasksRequest(_message.Message):
    __slots__ = ("remote_ip",)
    REMOTE_IP_FIELD_NUMBER: _ClassVar[int]
    remote_ip: str
    def __init__(self, remote_ip: _Optional[str] = ...) -> None: ...

class ListLocalTasksResponse(_message.Message):
    __slots__ = ("tasks",)
    TASKS_FIELD_NUMBER: _ClassVar[int]
    tasks: _containers.RepeatedCompositeFieldContainer[StatLocalTaskResponse]
    def __init__(self, tasks: _Optional[_Iterable[_Union[StatLocalTaskResponse, _Mapping]]] = ...) -> None: ...

class ListTaskEntriesRequest(_message.Message):
    __slots__ = ("task_id", "url", "request_header", "timeout", "certificate_chain", "object_storage", "hdfs", "remote_ip", "hugging_face", "model_scope", "open_csg")
    class RequestHeaderEntry(_message.Message):
        __slots__ = ("key", "value")
        KEY_FIELD_NUMBER: _ClassVar[int]
        VALUE_FIELD_NUMBER: _ClassVar[int]
        key: str
        value: str
        def __init__(self, key: _Optional[str] = ..., value: _Optional[str] = ...) -> None: ...
    TASK_ID_FIELD_NUMBER: _ClassVar[int]
    URL_FIELD_NUMBER: _ClassVar[int]
    REQUEST_HEADER_FIELD_NUMBER: _ClassVar[int]
    TIMEOUT_FIELD_NUMBER: _ClassVar[int]
    CERTIFICATE_CHAIN_FIELD_NUMBER: _ClassVar[int]
    OBJECT_STORAGE_FIELD_NUMBER: _ClassVar[int]
    HDFS_FIELD_NUMBER: _ClassVar[int]
    REMOTE_IP_FIELD_NUMBER: _ClassVar[int]
    HUGGING_FACE_FIELD_NUMBER: _ClassVar[int]
    MODEL_SCOPE_FIELD_NUMBER: _ClassVar[int]
    OPEN_CSG_FIELD_NUMBER: _ClassVar[int]
    task_id: str
    url: str
    request_header: _containers.ScalarMap[str, str]
    timeout: _duration_pb2.Duration
    certificate_chain: _containers.RepeatedScalarFieldContainer[bytes]
    object_storage: _common_pb2.ObjectStorage
    hdfs: _common_pb2.HDFS
    remote_ip: str
    hugging_face: _common_pb2.HuggingFace
    model_scope: _common_pb2.ModelScope
    open_csg: _common_pb2.OpenCSG
    def __init__(self, task_id: _Optional[str] = ..., url: _Optional[str] = ..., request_header: _Optional[_Mapping[str, str]] = ..., timeout: _Optional[_Union[datetime.timedelta, _duration_pb2.Duration, _Mapping]] = ..., certificate_chain: _Optional[_Iterable[bytes]] = ..., object_storage: _Optional[_Union[_common_pb2.ObjectStorage, _Mapping]] = ..., hdfs: _Optional[_Union[_common_pb2.HDFS, _Mapping]] = ..., remote_ip: _Optional[str] = ..., hugging_face: _Optional[_Union[_common_pb2.HuggingFace, _Mapping]] = ..., model_scope: _Optional[_Union[_common_pb2.ModelScope, _Mapping]] = ..., open_csg: _Optional[_Union[_common_pb2.OpenCSG, _Mapping]] = ...) -> None: ...

class ListTaskEntriesResponse(_message.Message):
    __slots__ = ("content_length", "response_header", "status_code", "entries")
    class ResponseHeaderEntry(_message.Message):
        __slots__ = ("key", "value")
        KEY_FIELD_NUMBER: _ClassVar[int]
        VALUE_FIELD_NUMBER: _ClassVar[int]
        key: str
        value: str
        def __init__(self, key: _Optional[str] = ..., value: _Optional[str] = ...) -> None: ...
    CONTENT_LENGTH_FIELD_NUMBER: _ClassVar[int]
    RESPONSE_HEADER_FIELD_NUMBER: _ClassVar[int]
    STATUS_CODE_FIELD_NUMBER: _ClassVar[int]
    ENTRIES_FIELD_NUMBER: _ClassVar[int]
    content_length: int
    response_header: _containers.ScalarMap[str, str]
    status_code: int
    entries: _containers.RepeatedCompositeFieldContainer[Entry]
    def __init__(self, content_length: _Optional[int] = ..., response_header: _Optional[_Mapping[str, str]] = ..., status_code: _Optional[int] = ..., entries: _Optional[_Iterable[_Union[Entry, _Mapping]]] = ...) -> None: ...

class Entry(_message.Message):
    __slots__ = ("url", "content_length", "is_dir")
    URL_FIELD_NUMBER: _ClassVar[int]
    CONTENT_LENGTH_FIELD_NUMBER: _ClassVar[int]
    IS_DIR_FIELD_NUMBER: _ClassVar[int]
    url: str
    content_length: int
    is_dir: bool
    def __init__(self, url: _Optional[str] = ..., content_length: _Optional[int] = ..., is_dir: bool = ...) -> None: ...

class DeleteTaskRequest(_message.Message):
    __slots__ = ("task_id", "remote_ip")
    TASK_ID_FIELD_NUMBER: _ClassVar[int]
    REMOTE_IP_FIELD_NUMBER: _ClassVar[int]
    task_id: str
    remote_ip: str
    def __init__(self, task_id: _Optional[str] = ..., remote_ip: _Optional[str] = ...) -> None: ...

class DeleteLocalTaskRequest(_message.Message):
    __slots__ = ("task_id", "remote_ip")
    TASK_ID_FIELD_NUMBER: _ClassVar[int]
    REMOTE_IP_FIELD_NUMBER: _ClassVar[int]
    task_id: str
    remote_ip: str
    def __init__(self, task_id: _Optional[str] = ..., remote_ip: _Optional[str] = ...) -> None: ...

class DownloadCacheTaskRequest(_message.Message):
    __slots__ = ("url", "digest", "range", "type", "tag", "application", "priority", "filtered_query_params", "request_header", "piece_length", "output_path", "timeout", "disable_back_to_source", "need_back_to_source", "certificate_chain", "prefetch", "object_storage", "hdfs", "is_prefetch", "need_piece_content", "content_for_calculating_task_id", "remote_ip", "overwrite")
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
    OVERWRITE_FIELD_NUMBER: _ClassVar[int]
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
    overwrite: bool
    def __init__(self, url: _Optional[str] = ..., digest: _Optional[str] = ..., range: _Optional[_Union[_common_pb2.Range, _Mapping]] = ..., type: _Optional[_Union[_common_pb2.TaskType, str]] = ..., tag: _Optional[str] = ..., application: _Optional[str] = ..., priority: _Optional[_Union[_common_pb2.Priority, str]] = ..., filtered_query_params: _Optional[_Iterable[str]] = ..., request_header: _Optional[_Mapping[str, str]] = ..., piece_length: _Optional[int] = ..., output_path: _Optional[str] = ..., timeout: _Optional[_Union[datetime.timedelta, _duration_pb2.Duration, _Mapping]] = ..., disable_back_to_source: bool = ..., need_back_to_source: bool = ..., certificate_chain: _Optional[_Iterable[bytes]] = ..., prefetch: bool = ..., object_storage: _Optional[_Union[_common_pb2.ObjectStorage, _Mapping]] = ..., hdfs: _Optional[_Union[_common_pb2.HDFS, _Mapping]] = ..., is_prefetch: bool = ..., need_piece_content: bool = ..., content_for_calculating_task_id: _Optional[str] = ..., remote_ip: _Optional[str] = ..., overwrite: bool = ...) -> None: ...

class DownloadCacheTaskStartedResponse(_message.Message):
    __slots__ = ("content_length", "range", "response_header", "pieces", "is_finished")
    class ResponseHeaderEntry(_message.Message):
        __slots__ = ("key", "value")
        KEY_FIELD_NUMBER: _ClassVar[int]
        VALUE_FIELD_NUMBER: _ClassVar[int]
        key: str
        value: str
        def __init__(self, key: _Optional[str] = ..., value: _Optional[str] = ...) -> None: ...
    CONTENT_LENGTH_FIELD_NUMBER: _ClassVar[int]
    RANGE_FIELD_NUMBER: _ClassVar[int]
    RESPONSE_HEADER_FIELD_NUMBER: _ClassVar[int]
    PIECES_FIELD_NUMBER: _ClassVar[int]
    IS_FINISHED_FIELD_NUMBER: _ClassVar[int]
    content_length: int
    range: _common_pb2.Range
    response_header: _containers.ScalarMap[str, str]
    pieces: _containers.RepeatedCompositeFieldContainer[_common_pb2.Piece]
    is_finished: bool
    def __init__(self, content_length: _Optional[int] = ..., range: _Optional[_Union[_common_pb2.Range, _Mapping]] = ..., response_header: _Optional[_Mapping[str, str]] = ..., pieces: _Optional[_Iterable[_Union[_common_pb2.Piece, _Mapping]]] = ..., is_finished: bool = ...) -> None: ...

class DownloadCacheTaskResponse(_message.Message):
    __slots__ = ("host_id", "task_id", "peer_id", "download_cache_task_started_response", "download_piece_finished_response")
    HOST_ID_FIELD_NUMBER: _ClassVar[int]
    TASK_ID_FIELD_NUMBER: _ClassVar[int]
    PEER_ID_FIELD_NUMBER: _ClassVar[int]
    DOWNLOAD_CACHE_TASK_STARTED_RESPONSE_FIELD_NUMBER: _ClassVar[int]
    DOWNLOAD_PIECE_FINISHED_RESPONSE_FIELD_NUMBER: _ClassVar[int]
    host_id: str
    task_id: str
    peer_id: str
    download_cache_task_started_response: DownloadCacheTaskStartedResponse
    download_piece_finished_response: DownloadPieceFinishedResponse
    def __init__(self, host_id: _Optional[str] = ..., task_id: _Optional[str] = ..., peer_id: _Optional[str] = ..., download_cache_task_started_response: _Optional[_Union[DownloadCacheTaskStartedResponse, _Mapping]] = ..., download_piece_finished_response: _Optional[_Union[DownloadPieceFinishedResponse, _Mapping]] = ...) -> None: ...

class SyncCachePiecesRequest(_message.Message):
    __slots__ = ("host_id", "task_id", "interested_cache_piece_numbers")
    HOST_ID_FIELD_NUMBER: _ClassVar[int]
    TASK_ID_FIELD_NUMBER: _ClassVar[int]
    INTERESTED_CACHE_PIECE_NUMBERS_FIELD_NUMBER: _ClassVar[int]
    host_id: str
    task_id: str
    interested_cache_piece_numbers: _containers.RepeatedScalarFieldContainer[int]
    def __init__(self, host_id: _Optional[str] = ..., task_id: _Optional[str] = ..., interested_cache_piece_numbers: _Optional[_Iterable[int]] = ...) -> None: ...

class SyncCachePiecesResponse(_message.Message):
    __slots__ = ("number", "offset", "length")
    NUMBER_FIELD_NUMBER: _ClassVar[int]
    OFFSET_FIELD_NUMBER: _ClassVar[int]
    LENGTH_FIELD_NUMBER: _ClassVar[int]
    number: int
    offset: int
    length: int
    def __init__(self, number: _Optional[int] = ..., offset: _Optional[int] = ..., length: _Optional[int] = ...) -> None: ...

class DownloadCachePieceRequest(_message.Message):
    __slots__ = ("host_id", "task_id", "piece_number")
    HOST_ID_FIELD_NUMBER: _ClassVar[int]
    TASK_ID_FIELD_NUMBER: _ClassVar[int]
    PIECE_NUMBER_FIELD_NUMBER: _ClassVar[int]
    host_id: str
    task_id: str
    piece_number: int
    def __init__(self, host_id: _Optional[str] = ..., task_id: _Optional[str] = ..., piece_number: _Optional[int] = ...) -> None: ...

class DownloadCachePieceResponse(_message.Message):
    __slots__ = ("piece", "digest")
    PIECE_FIELD_NUMBER: _ClassVar[int]
    DIGEST_FIELD_NUMBER: _ClassVar[int]
    piece: _common_pb2.Piece
    digest: str
    def __init__(self, piece: _Optional[_Union[_common_pb2.Piece, _Mapping]] = ..., digest: _Optional[str] = ...) -> None: ...

class StatCacheTaskRequest(_message.Message):
    __slots__ = ("task_id", "remote_ip")
    TASK_ID_FIELD_NUMBER: _ClassVar[int]
    REMOTE_IP_FIELD_NUMBER: _ClassVar[int]
    task_id: str
    remote_ip: str
    def __init__(self, task_id: _Optional[str] = ..., remote_ip: _Optional[str] = ...) -> None: ...

class DeleteCacheTaskRequest(_message.Message):
    __slots__ = ("task_id", "remote_ip")
    TASK_ID_FIELD_NUMBER: _ClassVar[int]
    REMOTE_IP_FIELD_NUMBER: _ClassVar[int]
    task_id: str
    remote_ip: str
    def __init__(self, task_id: _Optional[str] = ..., remote_ip: _Optional[str] = ...) -> None: ...

class DownloadPersistentTaskRequest(_message.Message):
    __slots__ = ("url", "object_storage", "persistent", "output_path", "timeout", "need_piece_content", "force_hard_link", "digest", "remote_ip", "overwrite")
    URL_FIELD_NUMBER: _ClassVar[int]
    OBJECT_STORAGE_FIELD_NUMBER: _ClassVar[int]
    PERSISTENT_FIELD_NUMBER: _ClassVar[int]
    OUTPUT_PATH_FIELD_NUMBER: _ClassVar[int]
    TIMEOUT_FIELD_NUMBER: _ClassVar[int]
    NEED_PIECE_CONTENT_FIELD_NUMBER: _ClassVar[int]
    FORCE_HARD_LINK_FIELD_NUMBER: _ClassVar[int]
    DIGEST_FIELD_NUMBER: _ClassVar[int]
    REMOTE_IP_FIELD_NUMBER: _ClassVar[int]
    OVERWRITE_FIELD_NUMBER: _ClassVar[int]
    url: str
    object_storage: _common_pb2.ObjectStorage
    persistent: bool
    output_path: str
    timeout: _duration_pb2.Duration
    need_piece_content: bool
    force_hard_link: bool
    digest: str
    remote_ip: str
    overwrite: bool
    def __init__(self, url: _Optional[str] = ..., object_storage: _Optional[_Union[_common_pb2.ObjectStorage, _Mapping]] = ..., persistent: bool = ..., output_path: _Optional[str] = ..., timeout: _Optional[_Union[datetime.timedelta, _duration_pb2.Duration, _Mapping]] = ..., need_piece_content: bool = ..., force_hard_link: bool = ..., digest: _Optional[str] = ..., remote_ip: _Optional[str] = ..., overwrite: bool = ...) -> None: ...

class DownloadPersistentTaskStartedResponse(_message.Message):
    __slots__ = ("content_length",)
    CONTENT_LENGTH_FIELD_NUMBER: _ClassVar[int]
    content_length: int
    def __init__(self, content_length: _Optional[int] = ...) -> None: ...

class DownloadPersistentTaskResponse(_message.Message):
    __slots__ = ("host_id", "task_id", "peer_id", "download_persistent_task_started_response", "download_piece_finished_response")
    HOST_ID_FIELD_NUMBER: _ClassVar[int]
    TASK_ID_FIELD_NUMBER: _ClassVar[int]
    PEER_ID_FIELD_NUMBER: _ClassVar[int]
    DOWNLOAD_PERSISTENT_TASK_STARTED_RESPONSE_FIELD_NUMBER: _ClassVar[int]
    DOWNLOAD_PIECE_FINISHED_RESPONSE_FIELD_NUMBER: _ClassVar[int]
    host_id: str
    task_id: str
    peer_id: str
    download_persistent_task_started_response: DownloadPersistentTaskStartedResponse
    download_piece_finished_response: DownloadPieceFinishedResponse
    def __init__(self, host_id: _Optional[str] = ..., task_id: _Optional[str] = ..., peer_id: _Optional[str] = ..., download_persistent_task_started_response: _Optional[_Union[DownloadPersistentTaskStartedResponse, _Mapping]] = ..., download_piece_finished_response: _Optional[_Union[DownloadPieceFinishedResponse, _Mapping]] = ...) -> None: ...

class UploadPersistentTaskRequest(_message.Message):
    __slots__ = ("url", "object_storage", "path", "persistent_replica_count", "ttl", "timeout", "remote_ip")
    URL_FIELD_NUMBER: _ClassVar[int]
    OBJECT_STORAGE_FIELD_NUMBER: _ClassVar[int]
    PATH_FIELD_NUMBER: _ClassVar[int]
    PERSISTENT_REPLICA_COUNT_FIELD_NUMBER: _ClassVar[int]
    TTL_FIELD_NUMBER: _ClassVar[int]
    TIMEOUT_FIELD_NUMBER: _ClassVar[int]
    REMOTE_IP_FIELD_NUMBER: _ClassVar[int]
    url: str
    object_storage: _common_pb2.ObjectStorage
    path: str
    persistent_replica_count: int
    ttl: _duration_pb2.Duration
    timeout: _duration_pb2.Duration
    remote_ip: str
    def __init__(self, url: _Optional[str] = ..., object_storage: _Optional[_Union[_common_pb2.ObjectStorage, _Mapping]] = ..., path: _Optional[str] = ..., persistent_replica_count: _Optional[int] = ..., ttl: _Optional[_Union[datetime.timedelta, _duration_pb2.Duration, _Mapping]] = ..., timeout: _Optional[_Union[datetime.timedelta, _duration_pb2.Duration, _Mapping]] = ..., remote_ip: _Optional[str] = ...) -> None: ...

class UpdatePersistentTaskRequest(_message.Message):
    __slots__ = ("task_id", "persistent", "remote_ip")
    TASK_ID_FIELD_NUMBER: _ClassVar[int]
    PERSISTENT_FIELD_NUMBER: _ClassVar[int]
    REMOTE_IP_FIELD_NUMBER: _ClassVar[int]
    task_id: str
    persistent: bool
    remote_ip: str
    def __init__(self, task_id: _Optional[str] = ..., persistent: bool = ..., remote_ip: _Optional[str] = ...) -> None: ...

class StatPersistentTaskRequest(_message.Message):
    __slots__ = ("task_id", "remote_ip")
    TASK_ID_FIELD_NUMBER: _ClassVar[int]
    REMOTE_IP_FIELD_NUMBER: _ClassVar[int]
    task_id: str
    remote_ip: str
    def __init__(self, task_id: _Optional[str] = ..., remote_ip: _Optional[str] = ...) -> None: ...

class StatLocalPersistentTaskRequest(_message.Message):
    __slots__ = ("task_id", "remote_ip")
    TASK_ID_FIELD_NUMBER: _ClassVar[int]
    REMOTE_IP_FIELD_NUMBER: _ClassVar[int]
    task_id: str
    remote_ip: str
    def __init__(self, task_id: _Optional[str] = ..., remote_ip: _Optional[str] = ...) -> None: ...

class StatLocalPersistentTaskResponse(_message.Message):
    __slots__ = ("task_id", "persistent", "ttl", "piece_length", "content_length", "uploading_count", "uploaded_count", "created_at", "updated_at", "failed_at", "finished_at")
    TASK_ID_FIELD_NUMBER: _ClassVar[int]
    PERSISTENT_FIELD_NUMBER: _ClassVar[int]
    TTL_FIELD_NUMBER: _ClassVar[int]
    PIECE_LENGTH_FIELD_NUMBER: _ClassVar[int]
    CONTENT_LENGTH_FIELD_NUMBER: _ClassVar[int]
    UPLOADING_COUNT_FIELD_NUMBER: _ClassVar[int]
    UPLOADED_COUNT_FIELD_NUMBER: _ClassVar[int]
    CREATED_AT_FIELD_NUMBER: _ClassVar[int]
    UPDATED_AT_FIELD_NUMBER: _ClassVar[int]
    FAILED_AT_FIELD_NUMBER: _ClassVar[int]
    FINISHED_AT_FIELD_NUMBER: _ClassVar[int]
    task_id: str
    persistent: bool
    ttl: _duration_pb2.Duration
    piece_length: int
    content_length: int
    uploading_count: int
    uploaded_count: int
    created_at: _timestamp_pb2.Timestamp
    updated_at: _timestamp_pb2.Timestamp
    failed_at: _timestamp_pb2.Timestamp
    finished_at: _timestamp_pb2.Timestamp
    def __init__(self, task_id: _Optional[str] = ..., persistent: bool = ..., ttl: _Optional[_Union[datetime.timedelta, _duration_pb2.Duration, _Mapping]] = ..., piece_length: _Optional[int] = ..., content_length: _Optional[int] = ..., uploading_count: _Optional[int] = ..., uploaded_count: _Optional[int] = ..., created_at: _Optional[_Union[datetime.datetime, _timestamp_pb2.Timestamp, _Mapping]] = ..., updated_at: _Optional[_Union[datetime.datetime, _timestamp_pb2.Timestamp, _Mapping]] = ..., failed_at: _Optional[_Union[datetime.datetime, _timestamp_pb2.Timestamp, _Mapping]] = ..., finished_at: _Optional[_Union[datetime.datetime, _timestamp_pb2.Timestamp, _Mapping]] = ...) -> None: ...

class ListLocalPersistentTasksRequest(_message.Message):
    __slots__ = ("remote_ip",)
    REMOTE_IP_FIELD_NUMBER: _ClassVar[int]
    remote_ip: str
    def __init__(self, remote_ip: _Optional[str] = ...) -> None: ...

class ListLocalPersistentTasksResponse(_message.Message):
    __slots__ = ("tasks",)
    TASKS_FIELD_NUMBER: _ClassVar[int]
    tasks: _containers.RepeatedCompositeFieldContainer[StatLocalPersistentTaskResponse]
    def __init__(self, tasks: _Optional[_Iterable[_Union[StatLocalPersistentTaskResponse, _Mapping]]] = ...) -> None: ...

class DeletePersistentTaskRequest(_message.Message):
    __slots__ = ("task_id", "remote_ip")
    TASK_ID_FIELD_NUMBER: _ClassVar[int]
    REMOTE_IP_FIELD_NUMBER: _ClassVar[int]
    task_id: str
    remote_ip: str
    def __init__(self, task_id: _Optional[str] = ..., remote_ip: _Optional[str] = ...) -> None: ...

class SyncPersistentPiecesRequest(_message.Message):
    __slots__ = ("host_id", "task_id", "interested_piece_numbers")
    HOST_ID_FIELD_NUMBER: _ClassVar[int]
    TASK_ID_FIELD_NUMBER: _ClassVar[int]
    INTERESTED_PIECE_NUMBERS_FIELD_NUMBER: _ClassVar[int]
    host_id: str
    task_id: str
    interested_piece_numbers: _containers.RepeatedScalarFieldContainer[int]
    def __init__(self, host_id: _Optional[str] = ..., task_id: _Optional[str] = ..., interested_piece_numbers: _Optional[_Iterable[int]] = ...) -> None: ...

class SyncPersistentPiecesResponse(_message.Message):
    __slots__ = ("number", "offset", "length", "ip", "tcp_port", "quic_port")
    NUMBER_FIELD_NUMBER: _ClassVar[int]
    OFFSET_FIELD_NUMBER: _ClassVar[int]
    LENGTH_FIELD_NUMBER: _ClassVar[int]
    IP_FIELD_NUMBER: _ClassVar[int]
    TCP_PORT_FIELD_NUMBER: _ClassVar[int]
    QUIC_PORT_FIELD_NUMBER: _ClassVar[int]
    number: int
    offset: int
    length: int
    ip: str
    tcp_port: int
    quic_port: int
    def __init__(self, number: _Optional[int] = ..., offset: _Optional[int] = ..., length: _Optional[int] = ..., ip: _Optional[str] = ..., tcp_port: _Optional[int] = ..., quic_port: _Optional[int] = ...) -> None: ...

class DownloadPersistentCacheTaskRequest(_message.Message):
    __slots__ = ("task_id", "persistent", "tag", "application", "output_path", "timeout", "need_piece_content", "force_hard_link", "digest", "remote_ip", "overwrite")
    TASK_ID_FIELD_NUMBER: _ClassVar[int]
    PERSISTENT_FIELD_NUMBER: _ClassVar[int]
    TAG_FIELD_NUMBER: _ClassVar[int]
    APPLICATION_FIELD_NUMBER: _ClassVar[int]
    OUTPUT_PATH_FIELD_NUMBER: _ClassVar[int]
    TIMEOUT_FIELD_NUMBER: _ClassVar[int]
    NEED_PIECE_CONTENT_FIELD_NUMBER: _ClassVar[int]
    FORCE_HARD_LINK_FIELD_NUMBER: _ClassVar[int]
    DIGEST_FIELD_NUMBER: _ClassVar[int]
    REMOTE_IP_FIELD_NUMBER: _ClassVar[int]
    OVERWRITE_FIELD_NUMBER: _ClassVar[int]
    task_id: str
    persistent: bool
    tag: str
    application: str
    output_path: str
    timeout: _duration_pb2.Duration
    need_piece_content: bool
    force_hard_link: bool
    digest: str
    remote_ip: str
    overwrite: bool
    def __init__(self, task_id: _Optional[str] = ..., persistent: bool = ..., tag: _Optional[str] = ..., application: _Optional[str] = ..., output_path: _Optional[str] = ..., timeout: _Optional[_Union[datetime.timedelta, _duration_pb2.Duration, _Mapping]] = ..., need_piece_content: bool = ..., force_hard_link: bool = ..., digest: _Optional[str] = ..., remote_ip: _Optional[str] = ..., overwrite: bool = ...) -> None: ...

class DownloadPersistentCacheTaskStartedResponse(_message.Message):
    __slots__ = ("content_length",)
    CONTENT_LENGTH_FIELD_NUMBER: _ClassVar[int]
    content_length: int
    def __init__(self, content_length: _Optional[int] = ...) -> None: ...

class DownloadPersistentCacheTaskResponse(_message.Message):
    __slots__ = ("host_id", "task_id", "peer_id", "download_persistent_cache_task_started_response", "download_piece_finished_response")
    HOST_ID_FIELD_NUMBER: _ClassVar[int]
    TASK_ID_FIELD_NUMBER: _ClassVar[int]
    PEER_ID_FIELD_NUMBER: _ClassVar[int]
    DOWNLOAD_PERSISTENT_CACHE_TASK_STARTED_RESPONSE_FIELD_NUMBER: _ClassVar[int]
    DOWNLOAD_PIECE_FINISHED_RESPONSE_FIELD_NUMBER: _ClassVar[int]
    host_id: str
    task_id: str
    peer_id: str
    download_persistent_cache_task_started_response: DownloadPersistentCacheTaskStartedResponse
    download_piece_finished_response: DownloadPieceFinishedResponse
    def __init__(self, host_id: _Optional[str] = ..., task_id: _Optional[str] = ..., peer_id: _Optional[str] = ..., download_persistent_cache_task_started_response: _Optional[_Union[DownloadPersistentCacheTaskStartedResponse, _Mapping]] = ..., download_piece_finished_response: _Optional[_Union[DownloadPieceFinishedResponse, _Mapping]] = ...) -> None: ...

class UploadPersistentCacheTaskRequest(_message.Message):
    __slots__ = ("content_for_calculating_task_id", "path", "persistent_replica_count", "tag", "application", "piece_length", "ttl", "timeout", "remote_ip")
    CONTENT_FOR_CALCULATING_TASK_ID_FIELD_NUMBER: _ClassVar[int]
    PATH_FIELD_NUMBER: _ClassVar[int]
    PERSISTENT_REPLICA_COUNT_FIELD_NUMBER: _ClassVar[int]
    TAG_FIELD_NUMBER: _ClassVar[int]
    APPLICATION_FIELD_NUMBER: _ClassVar[int]
    PIECE_LENGTH_FIELD_NUMBER: _ClassVar[int]
    TTL_FIELD_NUMBER: _ClassVar[int]
    TIMEOUT_FIELD_NUMBER: _ClassVar[int]
    REMOTE_IP_FIELD_NUMBER: _ClassVar[int]
    content_for_calculating_task_id: str
    path: str
    persistent_replica_count: int
    tag: str
    application: str
    piece_length: int
    ttl: _duration_pb2.Duration
    timeout: _duration_pb2.Duration
    remote_ip: str
    def __init__(self, content_for_calculating_task_id: _Optional[str] = ..., path: _Optional[str] = ..., persistent_replica_count: _Optional[int] = ..., tag: _Optional[str] = ..., application: _Optional[str] = ..., piece_length: _Optional[int] = ..., ttl: _Optional[_Union[datetime.timedelta, _duration_pb2.Duration, _Mapping]] = ..., timeout: _Optional[_Union[datetime.timedelta, _duration_pb2.Duration, _Mapping]] = ..., remote_ip: _Optional[str] = ...) -> None: ...

class UpdatePersistentCacheTaskRequest(_message.Message):
    __slots__ = ("task_id", "persistent", "remote_ip")
    TASK_ID_FIELD_NUMBER: _ClassVar[int]
    PERSISTENT_FIELD_NUMBER: _ClassVar[int]
    REMOTE_IP_FIELD_NUMBER: _ClassVar[int]
    task_id: str
    persistent: bool
    remote_ip: str
    def __init__(self, task_id: _Optional[str] = ..., persistent: bool = ..., remote_ip: _Optional[str] = ...) -> None: ...

class StatPersistentCacheTaskRequest(_message.Message):
    __slots__ = ("task_id", "remote_ip")
    TASK_ID_FIELD_NUMBER: _ClassVar[int]
    REMOTE_IP_FIELD_NUMBER: _ClassVar[int]
    task_id: str
    remote_ip: str
    def __init__(self, task_id: _Optional[str] = ..., remote_ip: _Optional[str] = ...) -> None: ...

class StatLocalPersistentCacheTaskRequest(_message.Message):
    __slots__ = ("task_id", "remote_ip")
    TASK_ID_FIELD_NUMBER: _ClassVar[int]
    REMOTE_IP_FIELD_NUMBER: _ClassVar[int]
    task_id: str
    remote_ip: str
    def __init__(self, task_id: _Optional[str] = ..., remote_ip: _Optional[str] = ...) -> None: ...

class StatLocalPersistentCacheTaskResponse(_message.Message):
    __slots__ = ("task_id", "persistent", "ttl", "piece_length", "content_length", "uploading_count", "uploaded_count", "created_at", "updated_at", "failed_at", "finished_at")
    TASK_ID_FIELD_NUMBER: _ClassVar[int]
    PERSISTENT_FIELD_NUMBER: _ClassVar[int]
    TTL_FIELD_NUMBER: _ClassVar[int]
    PIECE_LENGTH_FIELD_NUMBER: _ClassVar[int]
    CONTENT_LENGTH_FIELD_NUMBER: _ClassVar[int]
    UPLOADING_COUNT_FIELD_NUMBER: _ClassVar[int]
    UPLOADED_COUNT_FIELD_NUMBER: _ClassVar[int]
    CREATED_AT_FIELD_NUMBER: _ClassVar[int]
    UPDATED_AT_FIELD_NUMBER: _ClassVar[int]
    FAILED_AT_FIELD_NUMBER: _ClassVar[int]
    FINISHED_AT_FIELD_NUMBER: _ClassVar[int]
    task_id: str
    persistent: bool
    ttl: _duration_pb2.Duration
    piece_length: int
    content_length: int
    uploading_count: int
    uploaded_count: int
    created_at: _timestamp_pb2.Timestamp
    updated_at: _timestamp_pb2.Timestamp
    failed_at: _timestamp_pb2.Timestamp
    finished_at: _timestamp_pb2.Timestamp
    def __init__(self, task_id: _Optional[str] = ..., persistent: bool = ..., ttl: _Optional[_Union[datetime.timedelta, _duration_pb2.Duration, _Mapping]] = ..., piece_length: _Optional[int] = ..., content_length: _Optional[int] = ..., uploading_count: _Optional[int] = ..., uploaded_count: _Optional[int] = ..., created_at: _Optional[_Union[datetime.datetime, _timestamp_pb2.Timestamp, _Mapping]] = ..., updated_at: _Optional[_Union[datetime.datetime, _timestamp_pb2.Timestamp, _Mapping]] = ..., failed_at: _Optional[_Union[datetime.datetime, _timestamp_pb2.Timestamp, _Mapping]] = ..., finished_at: _Optional[_Union[datetime.datetime, _timestamp_pb2.Timestamp, _Mapping]] = ...) -> None: ...

class ListLocalPersistentCacheTasksRequest(_message.Message):
    __slots__ = ("remote_ip",)
    REMOTE_IP_FIELD_NUMBER: _ClassVar[int]
    remote_ip: str
    def __init__(self, remote_ip: _Optional[str] = ...) -> None: ...

class ListLocalPersistentCacheTasksResponse(_message.Message):
    __slots__ = ("tasks",)
    TASKS_FIELD_NUMBER: _ClassVar[int]
    tasks: _containers.RepeatedCompositeFieldContainer[StatLocalPersistentCacheTaskResponse]
    def __init__(self, tasks: _Optional[_Iterable[_Union[StatLocalPersistentCacheTaskResponse, _Mapping]]] = ...) -> None: ...

class DeletePersistentCacheTaskRequest(_message.Message):
    __slots__ = ("task_id", "remote_ip")
    TASK_ID_FIELD_NUMBER: _ClassVar[int]
    REMOTE_IP_FIELD_NUMBER: _ClassVar[int]
    task_id: str
    remote_ip: str
    def __init__(self, task_id: _Optional[str] = ..., remote_ip: _Optional[str] = ...) -> None: ...

class SyncPersistentCachePiecesRequest(_message.Message):
    __slots__ = ("host_id", "task_id", "interested_piece_numbers")
    HOST_ID_FIELD_NUMBER: _ClassVar[int]
    TASK_ID_FIELD_NUMBER: _ClassVar[int]
    INTERESTED_PIECE_NUMBERS_FIELD_NUMBER: _ClassVar[int]
    host_id: str
    task_id: str
    interested_piece_numbers: _containers.RepeatedScalarFieldContainer[int]
    def __init__(self, host_id: _Optional[str] = ..., task_id: _Optional[str] = ..., interested_piece_numbers: _Optional[_Iterable[int]] = ...) -> None: ...

class SyncPersistentCachePiecesResponse(_message.Message):
    __slots__ = ("number", "offset", "length", "ip", "tcp_port", "quic_port")
    NUMBER_FIELD_NUMBER: _ClassVar[int]
    OFFSET_FIELD_NUMBER: _ClassVar[int]
    LENGTH_FIELD_NUMBER: _ClassVar[int]
    IP_FIELD_NUMBER: _ClassVar[int]
    TCP_PORT_FIELD_NUMBER: _ClassVar[int]
    QUIC_PORT_FIELD_NUMBER: _ClassVar[int]
    number: int
    offset: int
    length: int
    ip: str
    tcp_port: int
    quic_port: int
    def __init__(self, number: _Optional[int] = ..., offset: _Optional[int] = ..., length: _Optional[int] = ..., ip: _Optional[str] = ..., tcp_port: _Optional[int] = ..., quic_port: _Optional[int] = ...) -> None: ...

class SyncHostRequest(_message.Message):
    __slots__ = ("host_id", "peer_id")
    HOST_ID_FIELD_NUMBER: _ClassVar[int]
    PEER_ID_FIELD_NUMBER: _ClassVar[int]
    host_id: str
    peer_id: str
    def __init__(self, host_id: _Optional[str] = ..., peer_id: _Optional[str] = ...) -> None: ...

class IBVerbsQueuePairEndpoint(_message.Message):
    __slots__ = ("num", "lid", "gid")
    NUM_FIELD_NUMBER: _ClassVar[int]
    LID_FIELD_NUMBER: _ClassVar[int]
    GID_FIELD_NUMBER: _ClassVar[int]
    num: int
    lid: int
    gid: bytes
    def __init__(self, num: _Optional[int] = ..., lid: _Optional[int] = ..., gid: _Optional[bytes] = ...) -> None: ...
