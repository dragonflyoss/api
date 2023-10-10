/// InterestedAllPiecesRequest represents interested all pieces request of SyncPiecesRequest.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InterestedAllPiecesRequest {}
/// InterestedPiecesRequest represents interested pieces request of SyncPiecesRequest.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InterestedPiecesRequest {
    /// Interested piece numbers.
    #[prost(uint32, repeated, tag = "1")]
    pub piece_numbers: ::prost::alloc::vec::Vec<u32>,
}
/// SyncPiecesRequest represents request of AnnouncePeer.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SyncPiecesRequest {
    /// Task id.
    #[prost(string, tag = "1")]
    pub task_id: ::prost::alloc::string::String,
    #[prost(oneof = "sync_pieces_request::Request", tags = "2, 3")]
    pub request: ::core::option::Option<sync_pieces_request::Request>,
}
/// Nested message and enum types in `SyncPiecesRequest`.
pub mod sync_pieces_request {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Request {
        #[prost(message, tag = "2")]
        InterestedAllPiecesRequest(super::InterestedAllPiecesRequest),
        #[prost(message, tag = "3")]
        InterestedPiecesRequest(super::InterestedPiecesRequest),
    }
}
/// InterestedPiecesResponse represents all pieces response of SyncPiecesResponse.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InterestedAllPiecesResponse {
    /// All of the pieces in task.
    #[prost(message, repeated, tag = "1")]
    pub pieces: ::prost::alloc::vec::Vec<super::super::common::v2::Piece>,
}
/// InterestedPiecesResponse represents interested pieces response of SyncPiecesResponse.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InterestedPiecesResponse {
    /// Interested pieces of task.
    #[prost(message, repeated, tag = "1")]
    pub pieces: ::prost::alloc::vec::Vec<super::super::common::v2::Piece>,
}
/// SyncPiecesResponse represents response of SyncPieces.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SyncPiecesResponse {
    #[prost(oneof = "sync_pieces_response::Response", tags = "1, 2")]
    pub response: ::core::option::Option<sync_pieces_response::Response>,
}
/// Nested message and enum types in `SyncPiecesResponse`.
pub mod sync_pieces_response {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag = "1")]
        InterestedAllPiecesResponse(super::InterestedAllPiecesResponse),
        #[prost(message, tag = "2")]
        InterestedPiecesResponse(super::InterestedPiecesResponse),
    }
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
pub mod dfdaemon_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Dfdaemon RPC Service.
    #[derive(Debug, Clone)]
    pub struct DfdaemonClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DfdaemonClient<tonic::transport::Channel> {
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
    impl<T> DfdaemonClient<T>
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
        ) -> DfdaemonClient<InterceptedService<T, F>>
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
            DfdaemonClient::new(InterceptedService::new(inner, interceptor))
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
        /// SyncPieces syncs pieces from the other peers.
        pub async fn sync_pieces(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::SyncPiecesRequest>,
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
                "/dfdaemon.v2.Dfdaemon/SyncPieces",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("dfdaemon.v2.Dfdaemon", "SyncPieces"));
            self.inner.streaming(req, path, codec).await
        }
        /// DownloadTask downloads task back-to-source.
        pub async fn download_task(
            &mut self,
            request: impl tonic::IntoRequest<super::DownloadTaskRequest>,
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
                "/dfdaemon.v2.Dfdaemon/DownloadTask",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("dfdaemon.v2.Dfdaemon", "DownloadTask"));
            self.inner.unary(req, path, codec).await
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
                "/dfdaemon.v2.Dfdaemon/UploadTask",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("dfdaemon.v2.Dfdaemon", "UploadTask"));
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
                "/dfdaemon.v2.Dfdaemon/StatTask",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("dfdaemon.v2.Dfdaemon", "StatTask"));
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
                "/dfdaemon.v2.Dfdaemon/DeleteTask",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("dfdaemon.v2.Dfdaemon", "DeleteTask"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod dfdaemon_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with DfdaemonServer.
    #[async_trait]
    pub trait Dfdaemon: Send + Sync + 'static {
        /// Server streaming response type for the SyncPieces method.
        type SyncPiecesStream: futures_core::Stream<
                Item = std::result::Result<super::SyncPiecesResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// SyncPieces syncs pieces from the other peers.
        async fn sync_pieces(
            &self,
            request: tonic::Request<tonic::Streaming<super::SyncPiecesRequest>>,
        ) -> std::result::Result<tonic::Response<Self::SyncPiecesStream>, tonic::Status>;
        /// DownloadTask downloads task back-to-source.
        async fn download_task(
            &self,
            request: tonic::Request<super::DownloadTaskRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
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
    /// Dfdaemon RPC Service.
    #[derive(Debug)]
    pub struct DfdaemonServer<T: Dfdaemon> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Dfdaemon> DfdaemonServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for DfdaemonServer<T>
    where
        T: Dfdaemon,
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
                "/dfdaemon.v2.Dfdaemon/SyncPieces" => {
                    #[allow(non_camel_case_types)]
                    struct SyncPiecesSvc<T: Dfdaemon>(pub Arc<T>);
                    impl<
                        T: Dfdaemon,
                    > tonic::server::StreamingService<super::SyncPiecesRequest>
                    for SyncPiecesSvc<T> {
                        type Response = super::SyncPiecesResponse;
                        type ResponseStream = T::SyncPiecesStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::SyncPiecesRequest>,
                            >,
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
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/dfdaemon.v2.Dfdaemon/DownloadTask" => {
                    #[allow(non_camel_case_types)]
                    struct DownloadTaskSvc<T: Dfdaemon>(pub Arc<T>);
                    impl<
                        T: Dfdaemon,
                    > tonic::server::UnaryService<super::DownloadTaskRequest>
                    for DownloadTaskSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
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
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/dfdaemon.v2.Dfdaemon/UploadTask" => {
                    #[allow(non_camel_case_types)]
                    struct UploadTaskSvc<T: Dfdaemon>(pub Arc<T>);
                    impl<
                        T: Dfdaemon,
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
                "/dfdaemon.v2.Dfdaemon/StatTask" => {
                    #[allow(non_camel_case_types)]
                    struct StatTaskSvc<T: Dfdaemon>(pub Arc<T>);
                    impl<T: Dfdaemon> tonic::server::UnaryService<super::StatTaskRequest>
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
                "/dfdaemon.v2.Dfdaemon/DeleteTask" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteTaskSvc<T: Dfdaemon>(pub Arc<T>);
                    impl<
                        T: Dfdaemon,
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
    impl<T: Dfdaemon> Clone for DfdaemonServer<T> {
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
    impl<T: Dfdaemon> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Dfdaemon> tonic::server::NamedService for DfdaemonServer<T> {
        const NAME: &'static str = "dfdaemon.v2.Dfdaemon";
    }
}
