use capnp::{message::ReaderOptions, serialize};

pub mod db_current;
pub mod db_next;
pub mod db_general;
pub mod utils;
pub mod tests;

pub mod clarity_capnp;
pub mod expressions_capnp;
pub mod functions_capnp;
pub mod std_capnp;
pub mod types_capnp;



fn test() {

    let mut message = ::capnp::message::Builder::new_default();
    let model = message.init_root::<clarity_capnp::value::Builder>();

    let data = serialize::write_message_to_words(&message);

    let reader = serialize::read_message(data.as_slice(), ReaderOptions::new()).unwrap();
    let model = reader.get_root::<clarity_capnp::value::Reader>().unwrap();
}