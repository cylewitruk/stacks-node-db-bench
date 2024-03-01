fn main () {
    build_capnp("capnp/std.capnp");
    build_capnp("capnp/types.capnp");
    build_capnp("capnp/expressions.capnp");
    build_capnp("capnp/functions.capnp");
    build_capnp("capnp/clarity.capnp");
}

fn build_capnp(file: &str) {
    capnpc::CompilerCommand::new()
      .output_path("src/")
      .src_prefix("capnp/")
      .file(file)
      .run().unwrap();
}