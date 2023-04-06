use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    tonic_build::configure()
        // for use the optional protocol https://github.com/hyperium/tonic/issues/627
        .protoc_arg("--experimental_allow_proto3_optional")
        .out_dir("src/pb")
        .compile(&["protos/reservation.proto"], &["protos"])
        .unwrap();

    let google_rs = Path::new("src/pb/google.protobuf.rs");
    if google_rs.exists() {
        fs::remove_file(google_rs).unwrap();
    }

    Command::new("cargo").args(&["fmt"]).output().unwrap();

    println!("cargo::rerun-if-changed=protos/reservation.proto");
}
