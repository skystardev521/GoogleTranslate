use ghuanye::{GoogleTrans, TransArgs};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Some(arg) = TransArgs::from(args) {
        GoogleTrans::trans(&arg);
    }
}
