fn main() {
    let models = "../protofiles/ex_models.proto";
    let services = "../protofiles/ex_services.proto";

    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir("./src")
        .compile(&[models, services], &["../protofiles"])
        .unwrap_or_else(|e| panic!("protobuf compile error: {}", e));

    println!("cargo:rerun-if-changed={}", [models, services].join(","));
}
