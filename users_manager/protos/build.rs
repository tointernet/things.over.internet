fn main() {
    let services = "../protofiles/users_service.proto";

    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .protoc_arg("--experimental_allow_proto3_optional")
        .out_dir("./src")
        .compile(&[services], &["../"])
        .unwrap_or_else(|e| panic!("protobuf compile error: {}", e));

    println!("cargo:rerun-if-changed={}", [services].join(","));
}
