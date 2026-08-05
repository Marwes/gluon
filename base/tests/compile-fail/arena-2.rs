//@normalize-stderr-test: "[^ ]*?[\\/]([^\\/]+).rs" -> ".../$1.rs"
extern crate gluon_base;
//@no-rustfix

use gluon_base::{
    ast::{Arena, Expr, RootExpr},
    mk_ast_arena, pos,
};

fn main() {
    mk_ast_arena!(arena1);
    mk_ast_arena!(arena2);
    //~^ E0716
    //~| E0716
    // temporary value dropped while borrowed (twice)

    let arena2_expr = arena2.alloc(pos::spanned(
        Default::default(),
        Expr::<String>::Error(None),
    ));

    // Should fail
    RootExpr::new(arena1, arena2_expr);
}
