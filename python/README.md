# Dragonfly API

Python gRPC bindings for the [Dragonfly](https://d7y.io) API, generated from the
proto definitions in [dragonflyoss/api](https://github.com/dragonflyoss/api).

## Installation

```shell
pip install dragonfly-api
```

## Usage

```python
import grpc

from dragonfly_api import common_pb2, dfdaemon_pb2, dfdaemon_pb2_grpc

channel = grpc.insecure_channel("unix:/var/run/dragonfly/dfdaemon.sock")
stub = dfdaemon_pb2_grpc.DfdaemonDownloadStub(channel)
```

## Development

Regenerate the code from the proto files in the repository root:

```shell
make python-protoc
```

## LICENSE

Apache 2.0 License. Please see [LICENSE](https://github.com/dragonflyoss/api/blob/main/LICENSE) for more information.
