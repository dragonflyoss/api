#!/bin/bash

LINT_DIR=/data
GOLANGCI_IMAGE=golangci/golangci-lint:v1.54

docker run --rm \
    -w "${LINT_DIR}" \
    -v "$(pwd):${LINT_DIR}:ro" \
    ${GOLANGCI_IMAGE} \
    golangci-lint \
    run -v
