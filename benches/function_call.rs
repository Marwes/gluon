#[macro_use]
extern crate criterion;

extern crate gluon;

use criterion::{black_box, Bencher, Criterion};

use gluon::vm::api::{primitive, FunctionRef, Primitive};
use gluon::vm::thread::{Status, Thread};
use gluon::{new_vm, Compiler};

// Benchmarks function calls
fn factorial(b: &mut Bencher, &input: &i64) {
    let vm = new_vm();
    let text = r#"
    let factorial n =
        if n #Int< 2
        then 1
        else n #Int* factorial (n #Int- 1)
    factorial
    "#;
    let (mut factorial, _): (FunctionRef<fn(i64) -> i64>, _) =
        Compiler::new().run_expr(&vm, "factorial", text).unwrap();
    b.iter(|| {
        let result = factorial.call(input).unwrap();
        black_box(result)
    })
}

fn factorial_lua(b: &mut Bencher, &input: &i64) {
    let vm = rlua::Lua::new();
    let text = r#"
    local function factorial(n)
        if n < 2 then
            return 1
        else
            return n * factorial(n - 1)
        end
    end

    return factorial
    "#;
    vm.context(|context| {
        let factorial: rlua::Function = context.load(text).eval().unwrap();
        b.iter(|| {
            let result: i64 = factorial.call((input,)).unwrap();
            black_box(result)
        })
    });
}

fn factorial_tail_call(b: &mut Bencher) {
    let vm = new_vm();
    let text = r#"
    let factorial a n =
        if n < 2
        then a
        else factorial (a * n) (n - 1)
    factorial 1
    "#;
    let (mut factorial, _): (FunctionRef<fn(i64) -> i64>, _) =
        Compiler::new().run_expr(&vm, "factorial", text).unwrap();
    b.iter(|| {
        let result = factorial.call(20).unwrap();
        black_box(result)
    })
}

fn gluon_rust_boundary_overhead(b: &mut Bencher) {
    let vm = new_vm();

    extern "C" fn test_fn(_: &Thread) -> Status {
        Status::Ok
    }

    let text = r#"
    let for n f =
        if n #Int== 0 then
            ()
        else
            f n
            f n
            f n
            f n
            f n
            f n
            f n
            f n
            f n
            f n
            for (n #Int- 10) f
    for
    "#;
    Compiler::new().load_script(&vm, "test", text).unwrap();

    let mut test: FunctionRef<fn(i64, Primitive<fn(i64)>) -> ()> = vm.get_global("test").unwrap();
    b.iter(|| {
        let result = test
            .call(1000, primitive::<fn(i64)>("test_fn", test_fn))
            .unwrap();
        black_box(result)
    })
}

fn function_call_benchmark(c: &mut Criterion) {
    env_logger::init();

    c.bench(
        "factorial",
        criterion::ParameterizedBenchmark::new("gluon", factorial, vec![1, 10, 20])
            .with_function("lua", factorial_lua),
    );
    c.bench_function("factorial tail call", factorial_tail_call);
    c.bench_function("gluon rust boundary overhead", gluon_rust_boundary_overhead);
}

criterion_group!(function_call, function_call_benchmark);
criterion_main!(function_call);
