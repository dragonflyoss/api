#!/bin/bash

PROTOC_ALL_IMAGE=${PROTOC_ALL_IMAGE:-"namely/protoc-all:1.47_2"}
PROTO_PATH=pkg/apis
LANGUAGE=go

proto_modules="common/v1 common/v2 cdnsystem/v1 dfdaemon/v1 errordetails/v1 errordetails/v2 manager/v1 manager/v2 scheduler/v1 scheduler/v2 security/v1"

echo "generate protos..."

for module in ${proto_modules}; do
  if docker run --rm -v $PWD:/defs ${PROTOC_ALL_IMAGE} \
    -d ${PROTO_PATH}/$module -i . \
    -l ${LANGUAGE} -o . \
    --go-source-relative \
    --with-validator \
    --validator-source-relative; then
    echo "generate protos ${module} successfully"
  else
    echo "generate protos ${module} failed"
  fi
done
