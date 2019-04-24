use syn::{parse_quote, Expr, ExprLit, Lit, LitBool};

use super::{ExprTransformerOutput, MutagenExprTransformer};
use crate::transform_info::SharedTransformInfo;

pub struct MutagenTransformerLitBool {
    pub transform_info: SharedTransformInfo,
}

// transforms bool literals to mutator expressions
impl MutagenExprTransformer for MutagenTransformerLitBool {
    fn map_expr(&mut self, e: Expr) -> ExprTransformerOutput {
        match e {
            Expr::Lit(ExprLit {
                lit: Lit::Bool(LitBool { value, span }),
                attrs: _,
            }) => {
                let mutator_id = self
                    .transform_info
                    .add_mutation(format!("LitBool {} -> {}", value, !value), span);
                let expr = parse_quote! {
                    ::mutagen::mutator::MutatorLitBool::new(#mutator_id, #value)
                        .run_mutator(
                            &mutagen::MutagenRuntimeConfig::get_default()
                        )
                };
                ExprTransformerOutput::changed(expr, span)
            }
            _ => ExprTransformerOutput::unchanged(e),
        }
    }
}
