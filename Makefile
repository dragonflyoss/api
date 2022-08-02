# Copyright The Dragonfly Authors.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#      http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

all: help

# Run code lint
lint: markdownlint
	@echo "Begin to golangci-lint."
	@golangci-lint run
.PHONY: lint

# Run markdown lint
markdownlint:
	@echo "Begin to markdownlint."
	@./hack/markdownlint.sh
.PHONY: markdownlint

# Run go generate
generate: protoc
	@go generate ./...
.PHONY: generate

# Generate grpc protos
protoc:
	@./hack/protoc.sh
.PHONY: protoc

# Clear compiled files
clean:
	@go clean
	@rm -rf bin .go .cache
.PHONY: clean

help: 
	@echo "make lint                           run code lint"
	@echo "make markdownlint                   run markdown lint"
	@echo "make generate                       run go generate"
	@echo "make protoc                         generate grpc protos"
	@echo "make clean                          clean"
