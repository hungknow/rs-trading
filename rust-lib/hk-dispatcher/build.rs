use protobuf_codegen::Codegen;

fn main() {
    Codegen::new()
        .protoc()
        .include("protos")
        .inputs(&[
            "protos/ffi_event_type.proto",
            "protos/ffi_request.proto",
            "protos/ffi_response.proto",
            "protos/add_event.proto",
        ])
        .out_dir("src/protos")
        .run_from_script();
}