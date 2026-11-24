import datetime

from google.protobuf import empty_pb2 as _empty_pb2
from google.protobuf import timestamp_pb2 as _timestamp_pb2
from google.protobuf.internal import containers as _containers
from google.protobuf.internal import enum_type_wrapper as _enum_type_wrapper
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from collections.abc import Iterable as _Iterable, Mapping as _Mapping
from typing import ClassVar as _ClassVar, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class ImageState(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = ()
    PENDING: _ClassVar[ImageState]
    RUNNING: _ClassVar[ImageState]
    SUCCEEDED: _ClassVar[ImageState]
    FAILED: _ClassVar[ImageState]
PENDING: ImageState
RUNNING: ImageState
SUCCEEDED: ImageState
FAILED: ImageState

class PreheatImageRequest(_message.Message):
    __slots__ = ("image", "namespace", "basic_auth", "bearer_token")
    IMAGE_FIELD_NUMBER: _ClassVar[int]
    NAMESPACE_FIELD_NUMBER: _ClassVar[int]
    BASIC_AUTH_FIELD_NUMBER: _ClassVar[int]
    BEARER_TOKEN_FIELD_NUMBER: _ClassVar[int]
    image: str
    namespace: str
    basic_auth: BasicAuth
    bearer_token: str
    def __init__(self, image: _Optional[str] = ..., namespace: _Optional[str] = ..., basic_auth: _Optional[_Union[BasicAuth, _Mapping]] = ..., bearer_token: _Optional[str] = ...) -> None: ...

class BasicAuth(_message.Message):
    __slots__ = ("username", "password")
    USERNAME_FIELD_NUMBER: _ClassVar[int]
    PASSWORD_FIELD_NUMBER: _ClassVar[int]
    username: str
    password: str
    def __init__(self, username: _Optional[str] = ..., password: _Optional[str] = ...) -> None: ...

class StatImageRequest(_message.Message):
    __slots__ = ("image", "namespace")
    IMAGE_FIELD_NUMBER: _ClassVar[int]
    NAMESPACE_FIELD_NUMBER: _ClassVar[int]
    image: str
    namespace: str
    def __init__(self, image: _Optional[str] = ..., namespace: _Optional[str] = ...) -> None: ...

class StatImageResponse(_message.Message):
    __slots__ = ("state", "blobs", "description", "created_at", "updated_at")
    STATE_FIELD_NUMBER: _ClassVar[int]
    BLOBS_FIELD_NUMBER: _ClassVar[int]
    DESCRIPTION_FIELD_NUMBER: _ClassVar[int]
    CREATED_AT_FIELD_NUMBER: _ClassVar[int]
    UPDATED_AT_FIELD_NUMBER: _ClassVar[int]
    state: ImageState
    blobs: _containers.RepeatedCompositeFieldContainer[Blob]
    description: str
    created_at: _timestamp_pb2.Timestamp
    updated_at: _timestamp_pb2.Timestamp
    def __init__(self, state: _Optional[_Union[ImageState, str]] = ..., blobs: _Optional[_Iterable[_Union[Blob, _Mapping]]] = ..., description: _Optional[str] = ..., created_at: _Optional[_Union[datetime.datetime, _timestamp_pb2.Timestamp, _Mapping]] = ..., updated_at: _Optional[_Union[datetime.datetime, _timestamp_pb2.Timestamp, _Mapping]] = ...) -> None: ...

class Blob(_message.Message):
    __slots__ = ("digest", "content_length", "finished_content_length", "is_finished")
    DIGEST_FIELD_NUMBER: _ClassVar[int]
    CONTENT_LENGTH_FIELD_NUMBER: _ClassVar[int]
    FINISHED_CONTENT_LENGTH_FIELD_NUMBER: _ClassVar[int]
    IS_FINISHED_FIELD_NUMBER: _ClassVar[int]
    digest: str
    content_length: int
    finished_content_length: int
    is_finished: bool
    def __init__(self, digest: _Optional[str] = ..., content_length: _Optional[int] = ..., finished_content_length: _Optional[int] = ..., is_finished: bool = ...) -> None: ...
