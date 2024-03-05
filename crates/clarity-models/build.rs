extern crate capnpc;

fn main () {
    ::capnpc::CompilerCommand::new()
      .file("capnp/std.capnp")
      .file("capnp/types.capnp")
      .file("capnp/expressions.capnp")
      .file("capnp/functions.capnp")
      .file("capnp/clarity.capnp")
      .src_prefix("capnp/")
      .run()
      .expect("failed to compile cap'n proto schemas");
}