extern crate prost_build;

fn main() {
    let mut config = prost_build::Config::new();
    config.protoc_arg("--experimental_allow_proto3_optional");
    config.type_attribute(".", "#[derive(serde::Serialize,serde::Deserialize)]");
    config
        .compile_protos(&["proto/gpspoint.proto"], &["proto/"])
        .expect("Failed to compile gpspoint.proto");
}
