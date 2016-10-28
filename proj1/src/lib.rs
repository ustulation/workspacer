extern crate proj0;

use proj0::{Param, proj_0};

#[no_mangle]
pub extern "C" fn ffi() {
    println!("C ffi");
}

pub fn proj_1(elt: Param) {
    proj_0(elt);
}

#[cfg(test)]
mod tests {
    use futures::{self, Future};

    #[test]
    fn proj_1() {
        let (_tx, rx) = futures::oneshot::<()>();
        let rx = Box::new(rx.map_err(|_e| ()));
        xsuper::proj_1(rx);
    }
}
