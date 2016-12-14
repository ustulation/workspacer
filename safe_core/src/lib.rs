#[macro_use]
extern crate maidsafe_utilities;

#[doc(hidded)]
pub fn foo() {
    let _joiner = maidsafe_utilities::thread::named("Core", move || {
        println!("Called: foo()");
    });
}

#[doc(hidded)]
pub fn bar() {
    let _joiner = maidsafe_utilities::thread::named("Core", move || {
        println!("Called: bar()");
    });
}
