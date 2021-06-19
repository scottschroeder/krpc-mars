extern crate protoc_rust;

fn main() {
    println!("cargo:rerun-if-changed=protos/krpc.proto");
    println!("cargo:rerun-if-changed=build.rs");
    //println!("cargo:rerun-if-changed=path/to/Cargo.lock");
    //println!("cargo:rerun-if-changed=src/krpc.rs");
    protoc_rust::Codegen::new()
        .out_dir("src/")
        .inputs(&["protos/krpc.proto"])
        .run()
        .expect("protoc")
}
