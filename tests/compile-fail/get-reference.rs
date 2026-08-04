use gluon::new_vm;
use gluon::vm::Variants;
use gluon::vm::api::Getable;
use gluon::vm::internal::Value;

#[cfg_attr(rustfmt, rustfmt_skip)]
fn main() {
    let vm = new_vm();
    let value = Value::int(0);
    let value = Variants::new(&value); //~ ERROR: does not live long enough
    let _: &'static str = <&'static str>::from_value(&vm, value);
}
