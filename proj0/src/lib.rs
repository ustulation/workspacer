extern crate futures;

use futures::Future;

#[doc(hidden)]
pub type Param = Box<Future<Item = (), Error = ()>>;

pub fn proj_0(_elt: Param) {
    println!("Called: proj_0.");
}

#[cfg(test)]
mod tests {
    use futures::{self, Future};

    #[test]
    fn proj_0() {
        let (_tx, rx) = futures::oneshot::<()>();
        let rx = Box::new(rx.map_err(|_e| ()));
        super::proj_0(rx);
    }
}
