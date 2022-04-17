use std::path::PathBuf;

fn main() {
    let path = PathBuf::from("../proto/chat.proto");

    protoc_rust_grpc::Codegen::new()
    .out_dir("src")
    .input(path)
    .rust_protobuf(true)
    .run()
    .expect("Erro compiling protocol buffer");
}