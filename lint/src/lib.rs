#![feature(rustc_private)]
#![warn(unused_extern_crates)]

extern crate rustc_ast;
extern crate rustc_hir;
extern crate rustc_lint;
extern crate rustc_session;
extern crate rustc_span;

use rustc_ast::ast::{AttrKind, Attribute};
use rustc_hir::{self as hir, /*ExprKind, HirId*/};
use rustc_lint::{LateContext, LateLintPass, LintContext, LintPass};
use rustc_session::declare_lint;
//use rustc_span::symbol::sym;
use rustc_span::symbol::Symbol;

dylint_linting::dylint_library!();

#[allow(unsafe_code)]
#[no_mangle]
pub fn register_lints(sess: &rustc_session::Session, lint_store: &mut rustc_lint::LintStore) {
    dylint_linting::init_config(sess);
    let symbols = Symbols::new();
    lint_store.register_lints(&[&UNROOTED_MUST_ROOT]);
    lint_store
        .register_late_pass(move |_| Box::new(UnrootedPass::new(symbols.clone())));
}

declare_lint!(
    UNROOTED_MUST_ROOT,
    Deny,
    "Warn and report usage of unrooted jsmanaged objects"
);

pub(crate) struct UnrootedPass {
    symbols: Symbols,
}

impl UnrootedPass {
    pub fn new(symbols: Symbols) -> UnrootedPass {
        UnrootedPass { symbols }
    }
}


fn has_lint_attr(sym: &Symbols, attrs: &[Attribute]) -> bool {
    if attrs.is_empty() {
        return false;
    }
    match &attrs[0].kind {
        AttrKind::Normal(normal) => println!("{:?} {:?} {:?}", normal.item.path, sym.unrooted_must_root_lint, sym.must_root),
        _ => {},
    }
    attrs.iter().any(|attr| {
        matches!(
            &attr.kind,
            AttrKind::Normal(normal)
            if normal.item.path.segments.len() == 2 &&
            normal.item.path.segments[0].ident.name == sym.unrooted_must_root_lint &&
            normal.item.path.segments[1].ident.name == sym.must_root
        )
    })
}

impl LintPass for UnrootedPass {
    fn name(&self) -> &'static str {
        "ServoUnrootedPass"
    }
}

impl<'tcx> LateLintPass<'tcx> for UnrootedPass {
    fn check_item(&mut self, cx: &LateContext<'tcx>, item: &'tcx hir::Item) {
        let attrs = cx.tcx.hir().attrs(item.hir_id());
        println!("checking {:?} {:?} {:?}", item.ident, item.span, attrs);
        if has_lint_attr(&self.symbols, &attrs) {
            cx.lint(
                UNROOTED_MUST_ROOT,
                "Type must be rooted, use #[unrooted_must_root_lint::must_root] \
                 on the struct definition to propagate",
                |_lint| _lint,
            );
            return;
        }
        //if let hir::ItemKind::Struct(def, ..) = &item.kind {
        //    for ref field in def.fields() {
                //let def_id = cx.tcx.hir().local_def_id(field.hir_id);
                //if true 
                //if is_unrooted_ty(&self.symbols, cx, cx.tcx.type_of(def_id), false) {
                        //|lint| lint.set_span(field.span),
                //}
        //    }
        //}
    }
}

macro_rules! symbols {
    ($($s: ident)+) => {
        #[derive(Clone)]
        #[allow(non_snake_case)]
        struct Symbols {
            $( $s: Symbol, )+
        }

        impl Symbols {
            fn new() -> Self {
                Symbols {
                    $( $s: Symbol::intern(stringify!($s)), )+
                }
            }
        }
    }
}

symbols! {
    unrooted_must_root_lint
    must_root
}
