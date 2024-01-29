/// TriggerDownloadTaskRequest represents request of TriggerDownloadTask.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TriggerDownloadTaskRequest {
    /// Download information.
    #[prost(message, optional, tag = "1")]
    pub download: ::core::option::Option<super::super::common::v2::Download>,
}
/// SyncPiecesRequest represents request of SyncPieces.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SyncPiecesRequest {
    /// Task id.
    #[prost(string, tag = "1")]
    pub task_id: ::prost::alloc::string::String,
    /// Interested piece numbers.
    #[prost(uint32, repeated, tag = "2")]
    pub interested_piece_numbers: ::prost::alloc::vec::Vec<u32>,
}
/// SyncPiecesResponse represents response of SyncPieces.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SyncPiecesResponse {
    /// Exist piece number.
    #[prost(uint32, tag = "1")]
    pub number: u32,
    /// Piece offset.
    #[prost(uint64, tag = "2")]
    pub offset: u64,
    /// Piece length.
    #[prost(uint64, tag = "3")]
    pub length: u64,
}
/// DownloadPieceRequest represents request of DownloadPiece.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DownloadPieceRequest {
    /// Task id.
    #[prost(string, tag = "1")]
    pub task_id: ::prost::alloc::string::String,
    /// Piece number.
    #[prost(uint32, tag = "2")]
    pub piece_number: u32,
}
/// DownloadPieceResponse represents response of DownloadPieces.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DownloadPieceResponse {
    /// Piece information.
    #[prost(message, optional, tag = "1")]
    pub piece: ::core::option::Option<super::super::common::v2::Piece>,
}
/// DownloadTaskRequest represents request of DownloadTask.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DownloadTaskRequest {
    /// Download information.
    #[prost(message, optional, tag = "1")]
    pub download: ::core::option::Option<super::super::common::v2::Download>,
}
/// DownloadTaskStartedRequest represents task download started request of DownloadTaskResponse.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DownloadTaskStartedRequest {
    /// Task response headers.
    #[prost(map = "string, string", tag = "1")]
    pub response_header: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// DownloadTaskFinishedRequest represents task download finished request of DownloadTaskResponse.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DownloadTaskFinishedRequest {}
/// DownloadPieceFinishedRequest represents piece download finished request of DownloadTaskResponse.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DownloadPieceFinishedRequest {
    /// Finished piece of task.
    #[prost(message, optional, tag = "1")]
    pub piece: ::core::option::Option<super::super::common::v2::Piece>,
}
/// DownloadTaskResponse represents response of DownloadTask.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DownloadTaskResponse {
    /// Host id.
    #[prost(string, tag = "1")]
    pub host_id: ::prost::alloc::string::String,
    /// Task id.
    #[prost(string, tag = "2")]
    pub task_id: ::prost::alloc::string::String,
    /// Peer id.
    #[prost(string, tag = "3")]
    pub peer_id: ::prost::alloc::string::String,
    #[prost(oneof = "download_task_response::Request", tags = "4, 5, 6")]
    pub request: ::core::option::Option<download_task_response::Request>,
}
/// Nested message and enum types in `DownloadTaskResponse`.
pub mod download_task_response {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Request {
        #[prost(message, tag = "4")]
        DownloadTaskStartedRequest(super::DownloadTaskStartedRequest),
        #[prost(message, tag = "5")]
        DownloadTaskFinishedRequest(super::DownloadTaskFinishedRequest),
        #[prost(message, tag = "6")]
        DownloadPieceFinishedRequest(super::DownloadPieceFinishedRequest),
    }
}
/// UploadTaskRequest represents request of UploadTask.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadTaskRequest {
    /// Task metadata.
    #[prost(message, optional, tag = "1")]
    pub task: ::core::option::Option<super::super::common::v2::Task>,
}
/// StatTaskRequest represents request of StatTask.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatTaskRequest {
    /// Task id.
    #[prost(string, tag = "1")]
    pub task_id: ::prost::alloc::string::String,
}
/// DeleteTaskRequest represents request of DeleteTask.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTaskRequest {
    /// Task id.
    #[prost(string, tag = "1")]
    pub task_id: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod dfdaemon_upload_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// DfdaemonUpload represents upload service of dfdaemon.
    #[derive(Debug, Clone)]
    pub struct DfdaemonUploadClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DfdaemonUploadClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> DfdaemonUploadClient<T>
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
        ) -> DfdaemonUploadClient<InterceptedService<T, F>>
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
            DfdaemonUploadClient::new(InterceptedService::new(inner, interceptor))
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
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// TriggerDownloadTask triggers download task.
        pub async fn trigger_download_task(
            &mut self,
            request: impl tonic::IntoRequest<super::TriggerDownloadTaskRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
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
                "/dfdaemon.v2.DfdaemonUpload/TriggerDownloadTask",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("dfdaemon.v2.DfdaemonUpload", "TriggerDownloadTask"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// SyncPieces syncs piece metadatas from remote peer.
        pub async fn sync_pieces(
            &mut self,
            request: impl tonic::IntoRequest<super::SyncPiecesRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::SyncPiecesResponse>>,
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
                "/dfdaemon.v2.DfdaemonUpload/SyncPieces",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("dfdaemon.v2.DfdaemonUpload", "SyncPieces"));
            self.inner.server_streaming(req, path, codec).await
        }
        /// DownloadPiece downloads piece from the remote peer.
        pub async fn download_piece(
            &mut self,
            request: impl tonic::IntoRequest<super::DownloadPieceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DownloadPieceResponse>,
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
                "/dfdaemon.v2.DfdaemonUpload/DownloadPiece",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("dfdaemon.v2.DfdaemonUpload", "DownloadPiece"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod dfdaemon_download_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// DfdaemonDownload represents download service of dfdaemon.
    #[derive(Debug, Clone)]
    pub struct DfdaemonDownloadClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DfdaemonDownloadClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> DfdaemonDownloadClient<T>
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
        ) -> DfdaemonDownloadClient<InterceptedService<T, F>>
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
            DfdaemonDownloadClient::new(InterceptedService::new(inner, interceptor))
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
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// DownloadTask downloads task back-to-source.
        pub async fn download_task(
            &mut self,
            request: impl tonic::IntoRequest<super::DownloadTaskRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::DownloadTaskResponse>>,
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
                "/dfdaemon.v2.DfdaemonDownload/DownloadTask",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("dfdaemon.v2.DfdaemonDownload", "DownloadTask"));
            self.inner.server_streaming(req, path, codec).await
        }
        /// UploadTask uploads task to p2p network.
        pub async fn upload_task(
            &mut self,
            request: impl tonic::IntoRequest<super::UploadTaskRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
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
                "/dfdaemon.v2.DfdaemonDownload/UploadTask",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("dfdaemon.v2.DfdaemonDownload", "UploadTask"));
            self.inner.unary(req, path, codec).await
        }
        /// StatTask stats task information.
        pub async fn stat_task(
            &mut self,
            request: impl tonic::IntoRequest<super::StatTaskRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v2::Task>,
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
                "/dfdaemon.v2.DfdaemonDownload/StatTask",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("dfdaemon.v2.DfdaemonDownload", "StatTask"));
            self.inner.unary(req, path, codec).await
        }
        /// DeleteTask deletes task from p2p network.
        pub async fn delete_task(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteTaskRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
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
                "/dfdaemon.v2.DfdaemonDownload/DeleteTask",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("dfdaemon.v2.DfdaemonDownload", "DeleteTask"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod dfdaemon_upload_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with DfdaemonUploadServer.
    #[async_trait]
    pub trait DfdaemonUpload: Send + Sync + 'static {
        /// TriggerDownloadTask triggers download task.
        async fn trigger_download_task(
            &self,
            request: tonic::Request<super::TriggerDownloadTaskRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /// Server streaming response type for the SyncPieces method.
        type SyncPiecesStream: futures_core::Stream<
                Item = std::result::Result<super::SyncPiecesResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// SyncPieces syncs piece metadatas from remote peer.
        async fn sync_pieces(
            &self,
            request: tonic::Request<super::SyncPiecesRequest>,
        ) -> std::result::Result<tonic::Response<Self::SyncPiecesStream>, tonic::Status>;
        /// DownloadPiece downloads piece from the remote peer.
        async fn download_piece(
            &self,
            request: tonic::Request<super::DownloadPieceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DownloadPieceResponse>,
            tonic::Status,
        >;
    }
    /// DfdaemonUpload represents upload service of dfdaemon.
    #[derive(Debug)]
    pub struct DfdaemonUploadServer<T: DfdaemonUpload> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: DfdaemonUpload> DfdaemonUploadServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
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
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for DfdaemonUploadServer<T>
    where
        T: DfdaemonUpload,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/dfdaemon.v2.DfdaemonUpload/TriggerDownloadTask" => {
                    #[allow(non_camel_case_types)]
                    struct TriggerDownloadTaskSvc<T: DfdaemonUpload>(pub Arc<T>);
                    impl<
                        T: DfdaemonUpload,
                    > tonic::server::UnaryService<super::TriggerDownloadTaskRequest>
                    for TriggerDownloadTaskSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TriggerDownloadTaskRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).trigger_download_task(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TriggerDownloadTaskSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/dfdaemon.v2.DfdaemonUpload/SyncPieces" => {
                    #[allow(non_camel_case_types)]
                    struct SyncPiecesSvc<T: DfdaemonUpload>(pub Arc<T>);
                    impl<
                        T: DfdaemonUpload,
                    > tonic::server::ServerStreamingService<super::SyncPiecesRequest>
                    for SyncPiecesSvc<T> {
                        type Response = super::SyncPiecesResponse;
                        type ResponseStream = T::SyncPiecesStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SyncPiecesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).sync_pieces(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SyncPiecesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/dfdaemon.v2.DfdaemonUpload/DownloadPiece" => {
                    #[allow(non_camel_case_types)]
                    struct DownloadPieceSvc<T: DfdaemonUpload>(pub Arc<T>);
                    impl<
                        T: DfdaemonUpload,
                    > tonic::server::UnaryService<super::DownloadPieceRequest>
                    for DownloadPieceSvc<T> {
                        type Response = super::DownloadPieceResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DownloadPieceRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).download_piece(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DownloadPieceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
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
    impl<T: DfdaemonUpload> Clone for DfdaemonUploadServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: DfdaemonUpload> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: DfdaemonUpload> tonic::server::NamedService for DfdaemonUploadServer<T> {
        const NAME: &'static str = "dfdaemon.v2.DfdaemonUpload";
    }
}
/// Generated server implementations.
pub mod dfdaemon_download_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with DfdaemonDownloadServer.
    #[async_trait]
    pub trait DfdaemonDownload: Send + Sync + 'static {
        /// Server streaming response type for the DownloadTask method.
        type DownloadTaskStream: futures_core::Stream<
                Item = std::result::Result<super::DownloadTaskResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// DownloadTask downloads task back-to-source.
        async fn download_task(
            &self,
            request: tonic::Request<super::DownloadTaskRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::DownloadTaskStream>,
            tonic::Status,
        >;
        /// UploadTask uploads task to p2p network.
        async fn upload_task(
            &self,
            request: tonic::Request<super::UploadTaskRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /// StatTask stats task information.
        async fn stat_task(
            &self,
            request: tonic::Request<super::StatTaskRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::common::v2::Task>,
            tonic::Status,
        >;
        /// DeleteTask deletes task from p2p network.
        async fn delete_task(
            &self,
            request: tonic::Request<super::DeleteTaskRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
    }
    /// DfdaemonDownload represents download service of dfdaemon.
    #[derive(Debug)]
    pub struct DfdaemonDownloadServer<T: DfdaemonDownload> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: DfdaemonDownload> DfdaemonDownloadServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
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
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for DfdaemonDownloadServer<T>
    where
        T: DfdaemonDownload,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/dfdaemon.v2.DfdaemonDownload/DownloadTask" => {
                    #[allow(non_camel_case_types)]
                    struct DownloadTaskSvc<T: DfdaemonDownload>(pub Arc<T>);
                    impl<
                        T: DfdaemonDownload,
                    > tonic::server::ServerStreamingService<super::DownloadTaskRequest>
                    for DownloadTaskSvc<T> {
                        type Response = super::DownloadTaskResponse;
                        type ResponseStream = T::DownloadTaskStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DownloadTaskRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).download_task(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DownloadTaskSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/dfdaemon.v2.DfdaemonDownload/UploadTask" => {
                    #[allow(non_camel_case_types)]
                    struct UploadTaskSvc<T: DfdaemonDownload>(pub Arc<T>);
                    impl<
                        T: DfdaemonDownload,
                    > tonic::server::UnaryService<super::UploadTaskRequest>
                    for UploadTaskSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UploadTaskRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).upload_task(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UploadTaskSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/dfdaemon.v2.DfdaemonDownload/StatTask" => {
                    #[allow(non_camel_case_types)]
                    struct StatTaskSvc<T: DfdaemonDownload>(pub Arc<T>);
                    impl<
                        T: DfdaemonDownload,
                    > tonic::server::UnaryService<super::StatTaskRequest>
                    for StatTaskSvc<T> {
                        type Response = super::super::super::common::v2::Task;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StatTaskRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).stat_task(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StatTaskSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/dfdaemon.v2.DfdaemonDownload/DeleteTask" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteTaskSvc<T: DfdaemonDownload>(pub Arc<T>);
                    impl<
                        T: DfdaemonDownload,
                    > tonic::server::UnaryService<super::DeleteTaskRequest>
                    for DeleteTaskSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteTaskRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).delete_task(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteTaskSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
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
    impl<T: DfdaemonDownload> Clone for DfdaemonDownloadServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: DfdaemonDownload> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: DfdaemonDownload> tonic::server::NamedService for DfdaemonDownloadServer<T> {
        const NAME: &'static str = "dfdaemon.v2.DfdaemonDownload";
    }
}
