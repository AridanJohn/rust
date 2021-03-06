use syntax::ast;
use syntax::ptr::P;
use syntax::token::{self, Token};
use syntax::tokenstream::{TokenStream, TokenTree};
use syntax_expand::base::{self, *};
use syntax_pos::symbol::Symbol;
use syntax_pos::Span;

pub fn expand_concat_idents<'cx>(
    cx: &'cx mut ExtCtxt<'_>,
    sp: Span,
    tts: TokenStream,
) -> Box<dyn base::MacResult + 'cx> {
    if tts.is_empty() {
        cx.span_err(sp, "concat_idents! takes 1 or more arguments.");
        return DummyResult::any(sp);
    }

    let mut res_str = String::new();
    for (i, e) in tts.into_trees().enumerate() {
        if i & 1 == 1 {
            match e {
                TokenTree::Token(Token { kind: token::Comma, .. }) => {}
                _ => {
                    cx.span_err(sp, "concat_idents! expecting comma.");
                    return DummyResult::any(sp);
                }
            }
        } else {
            match e {
                TokenTree::Token(Token { kind: token::Ident(name, _), .. }) => {
                    res_str.push_str(&name.as_str())
                }
                _ => {
                    cx.span_err(sp, "concat_idents! requires ident args.");
                    return DummyResult::any(sp);
                }
            }
        }
    }

    let ident = ast::Ident::new(Symbol::intern(&res_str), cx.with_call_site_ctxt(sp));

    struct ConcatIdentsResult {
        ident: ast::Ident,
    }

    impl base::MacResult for ConcatIdentsResult {
        fn make_expr(self: Box<Self>) -> Option<P<ast::Expr>> {
            Some(P(ast::Expr {
                id: ast::DUMMY_NODE_ID,
                kind: ast::ExprKind::Path(None, ast::Path::from_ident(self.ident)),
                span: self.ident.span,
                attrs: ast::AttrVec::new(),
            }))
        }

        fn make_ty(self: Box<Self>) -> Option<P<ast::Ty>> {
            Some(P(ast::Ty {
                id: ast::DUMMY_NODE_ID,
                kind: ast::TyKind::Path(None, ast::Path::from_ident(self.ident)),
                span: self.ident.span,
            }))
        }
    }

    Box::new(ConcatIdentsResult { ident })
}
