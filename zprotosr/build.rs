fn main() {
    let models = "../zprotofiles/ex_models.proto";
    let services = "../zprotofiles/ex_services.proto";

    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir("./src")
        .compile(&[models, services], &["../zprotofiles"])
        .unwrap_or_else(|e| panic!("protobuf compile error: {}", e));

    println!("cargo:rerun-if-changed={}", [models, services].join(","));
}
