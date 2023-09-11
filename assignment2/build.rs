fn main() {
    protobuf_codegen::Codegen::new()
        .cargo_out_dir("protos")
        .include("src/protos")
        .input("src/protos/person.proto")
        .run_from_script();
}
