fn main() {
    tonic_build::configure()
        .protoc_arg("--experimental_allow_proto3_optional")
        .compile(&["user.proto"], &["proto/"])
        .expect("Error while generating proto schemas");
}
