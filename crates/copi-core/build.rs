fn main() {
    let mut config = prost_build::Config::new();
    config.type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]");
    config.type_attribute(".", "#[serde(rename_all = \"camelCase\")]");
    // config.field_attribute("skip_response", "#[serde(default)]");
    config
        .compile_protos(
            &["../../../copi-proto/host_to_mcu.proto"],
            &["../../../copi-proto"],
        )
        .unwrap();
}
