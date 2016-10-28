extern crate proj1;
extern crate futures;

use futures::Future;

fn main() {
    let (_tx, rx) = futures::oneshot::<()>();
    let rx = Box::new(rx.map_err(|_e| ()));
    proj1::proj_1(rx);
}
