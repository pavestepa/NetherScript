mod fns;
use fns::hello::hello;

use fns::case1::case1;

fn main() {
    hello(String::from("Paul"));
    case1();
}
