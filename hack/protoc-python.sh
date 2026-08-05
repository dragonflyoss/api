#!/bin/bash

PYTHON=${PYTHON:-"python3"}
GRPCIO_TOOLS_VERSION=${GRPCIO_TOOLS_VERSION:-"1.76.0"}
VENV_PATH=.cache/protoc-python
PROTO_PATH=proto
OUTPUT_PATH=python/dragonfly_api

proto_modules="common errordetails dfdaemon manager scheduler"
grpc_proto_modules="dfdaemon manager scheduler"

if [ ! -x "${VENV_PATH}/bin/python" ]; then
  if ! ${PYTHON} -m venv ${VENV_PATH}; then
    echo "create virtualenv ${VENV_PATH} failed"
    exit 1
  fi
fi

if ! ${VENV_PATH}/bin/python -m pip install --quiet "grpcio-tools==${GRPCIO_TOOLS_VERSION}"; then
  echo "install grpcio-tools ${GRPCIO_TOOLS_VERSION} failed"
  exit 1
fi

echo "generate protos..."

for module in ${proto_modules}; do
  grpc_out=""
  if [[ " ${grpc_proto_modules} " == *" ${module} "* ]]; then
    grpc_out="--grpc_python_out=${OUTPUT_PATH}"
  fi

  if ${VENV_PATH}/bin/python -m grpc_tools.protoc \
    -I ${PROTO_PATH} \
    --python_out=${OUTPUT_PATH} \
    --pyi_out=${OUTPUT_PATH} \
    ${grpc_out} \
    ${PROTO_PATH}/${module}.proto; then
    echo "generate protos ${module} successfully"
  else
    echo "generate protos ${module} failed"
    exit 1
  fi
done

# protoc generates absolute imports for sibling modules, rewrite them to
# relative imports so the package works regardless of sys.path, refer to
# https://github.com/protocolbuffers/protobuf/issues/1491.
for file in ${OUTPUT_PATH}/*_pb2*.py ${OUTPUT_PATH}/*_pb2*.pyi; do
  sed -i.bak 's/^import \([a-z]*_pb2\) as/from . import \1 as/' ${file}
  rm -f ${file}.bak
done
