use proto_builder_trait::tonic::BuilderAttributes;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    tonic_build::configure()
        // for use the optional protocol https://github.com/hyperium/tonic/issues/627
        .protoc_arg("--experimental_allow_proto3_optional")
        .out_dir("src/pb")
        .with_sqlx_type(&["reservation.ReservationStatus"])
        .with_derive_builder(&[
            "reservation.ReservationQuery",
            "reservation.ReservationFilter",
        ])
        .with_derive_builder_into(
            "reservation.ReservationQuery",
            &["resource_id", "user_id", "status", "page", "desc"],
        )
        .with_derive_builder_into(
            "reservation.ReservationFilter",
            &["resource_id", "user_id", "status", "desc"],
        )
        .with_derive_builder_option("reservation.ReservationFilter", &["cursor"])
        .with_derive_builder_option("reservation.ReservationQuery", &["start", "end"])
        .with_type_attributes(
            &["reservation.ReservationFilter"],
            &[r#"#[builder(build_fn(name = "private_build"))]"#],
        )
        .with_field_attributes(
            &["page_size"],
            &["#[builder(setter(into), default = \"10\")]"],
        )
        .compile(&["protos/reservation.proto"], &["protos"])
        .unwrap();

    let google_rs = Path::new("src/pb/google.protobuf.rs");
    if google_rs.exists() {
        fs::remove_file(google_rs).unwrap();
    }

    Command::new("cargo").args(["fmt"]).output().unwrap();

    println!("cargo:rerun-if-changed=protos/reservation.proto");
}
