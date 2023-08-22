use crate::token::Token;
use std::fmt::{Display, Formatter};

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

impl Display for Attr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            Attr::Op(token) => write!(f, "{}", *token)?,
            Attr::Val(val) => write!(f, "{}", *val)?,
            Attr::Name(name) => write!(f, "{}", name)?,
        }
        Ok(())
    }
}

#[derive(PartialEq, Debug)]
pub struct TreeNode {
    pub child: Vec<Option<Box<TreeNode>>>,
    pub sibling: Option<Box<TreeNode>>,

    // XXX FIXME YSG
    pub line_number: i32,
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
            expression_type: ExpressionType::Void,
            kind: Kind::Expression(kind),
            attr: Attr::Val(0),
        }
    }

    // XXX FIXME YSG implement with loop
    fn print_tree(&self, f: &mut Formatter<'_>, indent_count: &mut usize) -> std::fmt::Result {
        *indent_count += 2;
        writeln!(f, "cnt = {} ", *indent_count)?;

        {
            let str = " ".repeat(*indent_count);

            match &self.kind {
                Kind::Statement(stmt) => match stmt {
                    StatementKind::IfK => writeln!(f, "{} If", str)?,
                    StatementKind::RepeatK => writeln!(f, "{} Repeat", str)?,
                    StatementKind::AssignK => writeln!(f, "{} Assign to: {}", str, self.attr)?,
                    StatementKind::ReadK => writeln!(f, "{} Read: {}", str, self.attr)?,
                    StatementKind::WriteK => writeln!(f, "{} Write", str)?,
                },
                Kind::Expression(expr) => match expr {
                    ExpressionKind::Opk => writeln!(f, "{} Op: {}", str, self.attr)?,
                    ExpressionKind::ConstK => writeln!(f, "{} const: {}", str, self.attr)?,
                    ExpressionKind::IdK => writeln!(f, "{} Id: {}", str, self.attr)?,
                },
            }

            for elem in self.child.iter().flatten() {
                elem.print_tree(f, indent_count)?;
            }

            if let Some(node) = &self.sibling {
                node.print_tree(f, indent_count)?;
            }
        }
        *indent_count -= 2;
        Ok(())
    }
}

impl Display for TreeNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut i = 0;
        self.print_tree(f, &mut i)?;
        Ok(())
    }
}
