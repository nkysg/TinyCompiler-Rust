use crate::token::Token;

#[derive(PartialEq, Debug)]
pub enum NodeKind {
    StatementK,
    ExpressionK,
}

#[derive(PartialEq, Debug)]
pub enum StatementKind {
    IfK,
    RepeatK,
    AssignK,
    ReadK,
    WriteK,
}

#[derive(PartialEq, Debug)]
pub enum ExpressionKind {
    Opk,
    ConstK,
    IdK,
}

#[derive(PartialEq, Debug)]
pub enum ExpressionType {
    Void,
    Integer,
    Boolean,
}

#[derive(PartialEq, Debug)]
pub enum Kind {
    Statement(StatementKind),
    Expression(ExpressionKind),
}

#[derive(PartialEq, Debug)]
pub enum Attr {
    Op(Token),
    Val(i32),
    Name(String),
}

#[derive(PartialEq, Debug)]
pub struct TreeNode {
    pub child: Vec<Option<Box<TreeNode>>>,
    pub sibling: Option<Box<TreeNode>>,

    // XXX FIXME YSG
    pub line_number: i32,
    node_kind: NodeKind,
    pub expression_type: ExpressionType,
    pub kind: Kind,
    pub attr: Attr,
}

impl TreeNode {
    pub fn new_statement_node(kind: StatementKind) -> Self {
        let child = vec![None, None, None];
        Self {
            child,
            sibling: None,
            line_number: 0,
            node_kind: NodeKind::StatementK,
            expression_type: ExpressionType::Void,
            kind: Kind::Statement(kind),
            attr: Attr::Val(0),
        }
    }

    pub fn new_expression_node(kind: ExpressionKind) -> Self {
        let child = vec![None, None, None];
        Self {
            child,
            sibling: None,
            line_number: 0,
            node_kind: NodeKind::ExpressionK,
            expression_type: ExpressionType::Void,
            kind: Kind::Expression(kind),
            attr: Attr::Val(0),
        }
    }
}
