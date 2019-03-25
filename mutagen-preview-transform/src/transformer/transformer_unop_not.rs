use syn::spanned::Spanned;
use syn::{parse_quote, Expr, ExprUnary, UnOp};

use super::{ExprTransformerOutput, MutagenExprTransformer};
use crate::transform_info::SharedTransformInfo;

pub struct MutagenTransformerUnopNot {
    pub transform_info: SharedTransformInfo,
}

impl MutagenExprTransformer for MutagenTransformerUnopNot {
    fn map_expr(&mut self, e: Expr) -> ExprTransformerOutput {
        match e {
            Expr::Unary(ExprUnary {
                attrs: _,
                expr,
                op: UnOp::Not(op_not),
            }) => {
                let mutator_id = self
                    .transform_info
                    .add_mutation("LitUnopNot".to_string(), op_not.span());
                let expr = parse_quote! {
                    <::mutagen_preview::mutator::MutatorUnopNot<_>>
                        ::new(#mutator_id, #expr)
                        .run_mutator(
                            &mutagen_preview::MutagenRuntimeConfig::get_default()
                        )
                };
                ExprTransformerOutput::changed(expr, op_not.span())
            }
            _ => ExprTransformerOutput::unchanged(e),
        }
    }
}
