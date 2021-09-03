use crate::error::ParseError;
use drip_syntax::SyntaxKind;

#[derive(Debug, Clone, PartialEq)]
pub(super) enum Event {
    StartNode {
        kind: SyntaxKind,
        offset: Option<usize>,
    },
    AddToken,
    FinishNode,
    Error(ParseError),
    Placeholder,
}
