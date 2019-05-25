#![cfg(feature = "rand")]

use std::{fs::File, io::Read};

use gluon::{vm::api::IO, ThreadExt};

fn main() {
    env_logger::init();

    if let Err(err) = main_() {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}

fn main_() -> Result<(), Box<dyn std::error::Error>> {
    let thread = gluon::new_vm();
    thread.get_database_mut().run_io(true);

    let mut source = String::new();
    let mut file = File::open("examples/24.glu")?;
    file.read_to_string(&mut source)?;

    gluon::Compiler::new_lock().run_expr::<IO<()>>(&thread, "24", &source)?;
    Ok(())
}
