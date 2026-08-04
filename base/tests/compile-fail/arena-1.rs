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
    // temporary value dropped while borrowed

    let arena1_expr = arena1.alloc(pos::spanned(
        Default::default(),
        Expr::<String>::Error(None),
    ));

    RootExpr::new(arena2, arena1_expr);
}
