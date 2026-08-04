//@normalize-stderr-test: "(/|\w:)[^ ]*/src/lib.rs" -> "../src/lib.rs"
//@normalize-stderr-test: "(/|\w:)[^ ]*/mod.rs" -> "../mod.rs"
extern crate gluon;

use gluon::{ThreadExt, new_vm};

fn main() {
    let vm = new_vm();

    let _ = vm.run_expr::<&str>("", r#" "test" "#);
    //~^ ERROR: the trait bound `for<'value> &str: Getable<'_, 'value>` is not satisfied
}
