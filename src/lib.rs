// Include the `items` module, which is generated from items.proto.
pub mod tensorflow {
    include!(concat!(env!("OUT_DIR"), "/tensorflow.rs"));
}

