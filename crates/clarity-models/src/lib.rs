
pub mod std_capnp {
    include!(concat!(env!("OUT_DIR"), "/std_capnp.rs"));
}

pub mod types_capnp {
    include!(concat!(env!("OUT_DIR"), "/types_capnp.rs"));
}

pub mod expressions_capnp {
    include!(concat!(env!("OUT_DIR"), "/expressions_capnp.rs"));
}

pub mod functions_capnp {
    include!(concat!(env!("OUT_DIR"), "/functions_capnp.rs"));
}

pub mod clarity_capnp {
    include!(concat!(env!("OUT_DIR"), "/clarity_capnp.rs"));
}