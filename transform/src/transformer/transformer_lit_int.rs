use syn::{parse_quote, Expr, ExprLit, Lit};

use super::{ExprTransformerOutput, MutagenExprTransformer};
use crate::transform_info::SharedTransformInfo;

pub struct MutagenTransformerLitInt {
    pub transform_info: SharedTransformInfo,
}

impl MutagenExprTransformer for MutagenTransformerLitInt {
    fn map_expr(&mut self, e: Expr) -> ExprTransformerOutput {
        match e {
            Expr::Lit(ExprLit {
                lit: Lit::Int(l),
                attrs: _,
            }) => {
                let mutator_id = self
                    .transform_info
                    .add_mutation(format!("LitInt {}", l.value()), l.span());
                let expr = parse_quote! {
                    <::mutagen::mutator::MutatorLitInt<_>>
                        ::new(#mutator_id, #l)
                        .run_mutator(
                            &mutagen::MutagenRuntimeConfig::get_default()
                        )
                };
                ExprTransformerOutput::changed(expr, l.span())
            }
            _ => ExprTransformerOutput::unchanged(e),
        }
    }
}
