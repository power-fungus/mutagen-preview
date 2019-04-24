use syn::spanned::Spanned;
use syn::{parse_quote, BinOp, Expr, ExprBinary};

use super::{ExprTransformerOutput, MutagenExprTransformer};
use crate::transform_info::SharedTransformInfo;

pub struct MutagenTransformerBinopAdd {
    pub transform_info: SharedTransformInfo,
}

impl MutagenExprTransformer for MutagenTransformerBinopAdd {
    fn map_expr(&mut self, e: Expr) -> ExprTransformerOutput {
        match e {
            Expr::Binary(ExprBinary {
                attrs: _,
                left,
                right,
                op: BinOp::Add(op_add),
            }) => {
                let mutator_id = self
                    .transform_info
                    .add_mutation("LitBinopAdd".to_string(), op_add.span());
                let expr = parse_quote! {
                    <::mutagen::mutator::MutatorBinopAdd<_, _>>
                        ::new(#mutator_id, #left, #right)
                        .run_mutator(
                            &mutagen::MutagenRuntimeConfig::get_default()
                        )
                };
                ExprTransformerOutput::changed(expr, op_add.span())
            }
            _ => ExprTransformerOutput::unchanged(e),
        }
    }
}

// TOOD
// Q1: set spans where?
// Q2: type for Expr  T conversion
