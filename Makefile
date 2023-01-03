go-proto-gen:
	@go install google.golang.org/protobuf/cmd/protoc-gen-go@latest
	@go install google.golang.org/grpc/cmd/protoc-gen-go-grpc@latest

protog: go-proto-gen
	protoc --experimental_allow_proto3_optional --go-grpc_out=require_unimplemented_servers=false:./protosg --go_out=./protosg -I . ./protofiles/*.proto