# Dragonfly API (Blaxel Fork)

This public fork of the [Dragonfly API](https://github.com/dragonflyoss/api)
can be used to stage features for upstream PRs or, if need be,
to maintain Blaxel-specific functionality (but consider visibility when doing so).

## Usage

Always rebase rather than merge when incorporating upstream changes back into the fork.

At any given time, there should be a minimal number of focused commits on top of the upstream head.
Much of the automated GitHub-specific infrastructure (dependabot, release workflow, templates, etc.)
has been removed from this fork.
The idea is to instead rely on the upstream repository to incorporate dependabot suggestions
and rebase on those changes here.

### Generating Go bindings on Apple Silicon

The normal `make go-protoc` command will not work on an Arm machine (even with Rosetta).
You can emulate that command locally with something like this:

```bash
# Prerequisite: Install this dependency.
go install github.com/envoyproxy/protoc-gen-validate@latest

# Assume protoc-gen-validate@latest is version 1.3.3.
VALIDATE_INCLUDE="$(go env GOPATH)/pkg/mod/github.com/envoyproxy/protoc-gen-validate@v1.3.3"
GOOGLE_INCLUDE="/opt/homebrew/opt/protobuf/include"
PROTO_DIR="pkg/apis/dfdaemon/v2"
PROTO_FILES="$(find "$PROTO_DIR" -maxdepth 1 -name "*.proto")
protoc -I "$GOOGLE_INCLUDE" -I "$VALIDATE_INCLUDE" -I . -I "$PROTO_DIR" \
  --go_out=. --go_opt=paths=source_relative \
  --go-grpc_out=. --go-grpc_opt=paths=source_relative,require_unimplemented_servers=false \
  --validate_out="lang=go,paths=source_relative:." \
  $PROTO_FILES
```

Note that it's easy to generate Rust bindings with `cargo build`.
