macro_rules! include_proto {
    ($package:tt) => {
        include!(concat!(env!("OUT_DIR"), "/", $package, ".rs"));
    };
}

pub mod v1 {
    include_proto!("coin_shuffle.v1");
}
