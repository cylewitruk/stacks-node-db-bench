pub mod db_current;
pub mod db_next;
pub mod db_general;
pub mod utils;
pub mod tests;

// #[test]
// fn test() {

//     //use capnp::{message::ReaderOptions, serialize};
//     use clarity_models::*;

//     let mut message = ::capnp::message::Builder::new_default();
//     let model = message.init_root::<clarity_capnp::value::Builder>();

//     let data = serialize::write_message_to_words(&message);

//     let reader = serialize::read_message(data.as_slice(), ReaderOptions::new()).unwrap();
//     let model = reader.get_root::<clarity_capnp::value::Reader>().unwrap();
// }