use prost_wkt_build::*;
use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    let descriptor_file = out.join("descriptors.bin");
    let mut prost_config = prost_build::Config::new();
    prost_config.protoc_arg("--experimental_allow_proto3_optional");

    tonic_build::configure()
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute(
            ".openfga.v1.Userset",
            "#[serde(rename_all = \"camelCase\")]",
        )
        .type_attribute(
            ".openfga.v1.TupleToUserset",
            "#[serde(rename_all = \"camelCase\")]",
        )
        .field_attribute(
            ".openfga.v1.Userset.this",
            "#[serde(skip_serializing_if = \"Option::is_none\")]",
        )
        .field_attribute(
            ".openfga.v1.Userset.computed_userset",
            "#[serde(skip_serializing_if = \"Option::is_none\")]",
        )
        .field_attribute(
            ".openfga.v1.Userset.tuple_to_userset",
            "#[serde(skip_serializing_if = \"Option::is_none\")]",
        )
        .field_attribute(
            ".openfga.v1.Userset.union",
            "#[serde(skip_serializing_if = \"Option::is_none\")]",
        )
        .field_attribute(
            ".openfga.v1.Userset.intersection",
            "#[serde(skip_serializing_if = \"Option::is_none\")]",
        )
        .field_attribute(
            ".openfga.v1.Userset.difference",
            "#[serde(skip_serializing_if = \"Option::is_none\")]",
        )
        .field_attribute(
            ".openfga.v1.TupleToUserset.tupleset",
            "#[serde(skip_serializing_if = \"Option::is_none\")]",
        )
        .field_attribute(
            ".openfga.v1.TupleToUserset.computed_userset",
            "#[serde(skip_serializing_if = \"Option::is_none\")]",
        )
        .field_attribute(
            ".openfga.v1.CheckRequest.contextual_tuples",
            "#[serde(skip_serializing_if = \"Option::is_none\")]",
        )
        .field_attribute(
            ".openfga.v1.CheckRequest.trace",
            "#[serde(skip_serializing_if = \"Option::is_none\")]",
        )
        .field_attribute(
            ".openfga.v1.CheckRequest.store_id",
            "#[serde(skip_serializing_if = \"Option::is_none\")]",
        )
        .field_attribute(
            ".openfga.v1.CheckResponse.resolution",
            "#[serde(skip_serializing_if = \"Option::is_none\")]",
        )
        .field_attribute(
            ".openfga.v1.RelationReference.relation",
            "#[serde(skip_serializing_if = \"Option::is_none\")]",
        )
        .field_attribute(
            ".openfga.v1.RelationReference.wildcard",
            "#[serde(skip_serializing_if = \"Option::is_none\")]",
        )
        .extern_path(".google.protobuf.Any", "::prost_wkt_types::Any")
        .extern_path(".google.protobuf.Timestamp", "::prost_wkt_types::Timestamp")
        .extern_path(".google.protobuf.Value", "::prost_wkt_types::Value")
        .build_server(false)
        .file_descriptor_set_path(&descriptor_file)
        .compile_with_config(
            prost_config,
            &["proto/openfga/v1/openfga_service.proto"],
            &["proto"],
        )
        .unwrap();

    let descriptor_bytes = std::fs::read(descriptor_file).unwrap();

    let descriptor = FileDescriptorSet::decode(&descriptor_bytes[..]).unwrap();

    prost_wkt_build::add_serde(out, descriptor);

    Ok(())
}
