//@normalize-stderr-test: "[^ ]*?[\\/]([^\\/]+).rs" -> ".../$1.rs"
#[macro_use]
extern crate gluon_vm;
extern crate gluon;
extern crate gluon_codegen;
//@no-rustfix

use gluon::{
    import::add_extern_module,
    new_vm,
    vm::{
        ExternModule,
        api::{Userdata, VmType},
    },
};

#[derive(Debug, gluon_codegen::Trace)]
struct Test;

impl Userdata for Test {}

impl VmType for Test {
    type Type = Test;
}

fn f(_: &'static Test) {}

#[cfg_attr(rustfmt, rustfmt_skip)]
fn main() {
    let vm = new_vm();
    add_extern_module(&vm, "test", |vm| {
        ExternModule::new(vm, primitive!(1, f))
        //~^ ERROR: lifetime may not live long enough
    });
}
