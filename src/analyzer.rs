use crate::ast::{Attr, ExpressionKind, ExpressionType, Kind, StatementKind, TreeNode};
use crate::symtable::SymTable;
use crate::token::Token;
use anyhow::Result;

type InsertEmptyCallback = fn(&Option<Box<TreeNode>>);
type InsertCallback = fn(&mut Analyzer, &mut SymTable, &Option<Box<TreeNode>>);

type TypeEmptyCallback = fn(&mut Option<Box<TreeNode>>) -> Result<()>;
type TypeCheckCallback = fn(&mut Option<Box<TreeNode>>) -> Result<()>;

#[derive(Debug, Clone, Default)]
pub struct Analyzer {
    location: i32,
}

fn insert_empty(_node: &Option<Box<TreeNode>>) {}

fn insert_node(analyzer: &mut Analyzer, sym_table: &mut SymTable, node: &Option<Box<TreeNode>>) {
    if let Some(node) = node {
        match &node.kind {
            Kind::Statement(stmt) => match stmt {
                StatementKind::AssignK | StatementKind::ReadK => {
                    if let Attr::Name(str) = &node.attr {
                        if sym_table.st_lookup(str.as_str()).is_none() {
                            sym_table.st_insert(
                                str.as_str(),
                                node.line_number,
                                analyzer.add_location(),
                            );
                        } else {
                            sym_table.st_insert(str.as_str(), node.line_number, 0);
                        }
                    }
                }
                _ => {}
            },
            Kind::Expression(expr) => {
                if expr == &ExpressionKind::IdK {
                    if let Attr::Name(str) = &node.attr {
                        if sym_table.st_lookup(str.as_str()).is_none() {
                            sym_table.st_insert(
                                str.as_str(),
                                node.line_number,
                                analyzer.add_location(),
                            );
                        } else {
                            sym_table.st_insert(str.as_str(), node.line_number, 0);
                        }
                    }
                }
            }
        }
    }
}

fn type_empty(_node: &mut Option<Box<TreeNode>>) -> Result<()> {
    Ok(())
}
fn type_node(node: &mut Option<Box<TreeNode>>) -> Result<()> {
    if let Some(node) = node {
        match &node.kind {
            Kind::Statement(stmt) => match stmt {
                StatementKind::IfK | StatementKind::AssignK | StatementKind::WriteK => {
                    if node.child[0].is_none() {
                        return Err(anyhow::format_err!("{:#?} {:#?}", stmt, node));
                    }
                    if let Some(node2) = &node.child[0] {
                        if node2.expression_type != ExpressionType::Integer {
                            return Err(anyhow::format_err!("{:#?} {:#?}", stmt, node));
                        }
                    }
                }
                StatementKind::RepeatK => {
                    if node.child[1].is_none() {
                        return Err(anyhow::format_err!("{:#?} {:#?}", stmt, node));
                    }
                    if let Some(node2) = &node.child[1] {
                        if node2.expression_type != ExpressionType::Boolean {
                            return Err(anyhow::format_err!("{:#?} {:#?}", stmt, node));
                        }
                    }
                }
                _ => {}
            },
            Kind::Expression(expr) => match expr {
                ExpressionKind::Opk => {
                    if node.child[0].is_none() || node.child[1].is_none() {
                        return Err(anyhow::format_err!("{:#?} {:#?}", expr, node));
                    }

                    if let (Some(node2), Some(node3)) = (&node.child[0], &node.child[1]) {
                        if node2.expression_type != ExpressionType::Integer
                            || node3.expression_type != ExpressionType::Integer
                        {
                            return Err(anyhow::format_err!("{:#?} {:#?}", expr, node));
                        }
                    }

                    if let Attr::Op(token) = &node.attr {
                        match token {
                            &Token::Eq | &Token::Lt => {
                                node.expression_type = ExpressionType::Boolean
                            }
                            _ => node.expression_type = ExpressionType::Integer,
                        }
                    }
                }
                ExpressionKind::ConstK | ExpressionKind::IdK => {
                    node.expression_type = ExpressionType::Integer;
                }
            },
        }
    }
    Ok(())
}

impl Analyzer {
    pub fn new() -> Self {
        Self { location: 0 }
    }

    pub fn build_symbol_table(&mut self, node: &Option<Box<TreeNode>>) -> SymTable {
        let mut sym_table = SymTable::new();
        self.insert_sym_table(&mut sym_table, node, insert_node, insert_empty);
        sym_table
    }

    fn add_location(&mut self) -> i32 {
        let val = self.location;
        self.location += 1;
        val
    }

    fn insert_sym_table(
        &mut self,
        sym_table: &mut SymTable,
        node: &Option<Box<TreeNode>>,
        pre_order: InsertCallback,
        post_order: InsertEmptyCallback,
    ) {
        if node.is_some() {
            pre_order(self, sym_table, node);

            if let Some(ref node1) = node {
                for elem in node1.child.iter() {
                    self.insert_sym_table(sym_table, elem, pre_order, post_order);
                }
            }

            post_order(node);

            if let Some(ref node1) = node {
                self.insert_sym_table(sym_table, &node1.sibling, pre_order, post_order);
            }
        }
    }

    pub fn type_check(node: &mut Option<Box<TreeNode>>) -> Result<()> {
        Self::type_traverse(node, type_empty, type_node)
    }

    fn type_traverse(
        node: &mut Option<Box<TreeNode>>,
        pre_order: TypeEmptyCallback,
        post_order: TypeCheckCallback,
    ) -> Result<()> {
        match node {
            Some(_) => {
                pre_order(node)?;

                if let Some(node1) = node {
                    for elem in node1.child.iter_mut() {
                        Self::type_traverse(elem, pre_order, post_order)?;
                    }
                }

                post_order(node)?;
                if let Some(node1) = node {
                    Self::type_traverse(&mut node1.sibling, pre_order, post_order)?;
                }
            }
            None => {}
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::analyzer::Analyzer;
    use crate::parser::Parser;
    use crate::token::Token;
    use anyhow::Result;

    #[test]
    fn test_analyze() -> Result<()> {
        let tokens = vec![
            Token::Read,
            Token::Id("x".into()),
            Token::Semi,
            Token::If,
            Token::Num("0".into()),
            Token::Lt,
            Token::Id("x".into()),
            Token::Then,
            Token::Id("fact".into()),
            Token::Assign,
            Token::Num("1".into()),
            Token::Semi,
            Token::Repeat,
            Token::Id("fact".into()),
            Token::Assign,
            Token::Id("fact".into()),
            Token::Times,
            Token::Id("x".into()),
            Token::Semi,
            Token::Id("x".into()),
            Token::Assign,
            Token::Id("x".into()),
            Token::Minus,
            Token::Num("1".into()),
            Token::Until,
            Token::Id("x".into()),
            Token::Eq,
            Token::Num("0".into()),
            Token::Semi,
            Token::Write,
            Token::Id("fact".into()),
            Token::End,
            Token::EndFile,
        ];
        let mut parser = Parser::new(tokens);
        let node = parser.parse()?;
        let mut analyzer = Analyzer::new();
        let sym_table = analyzer.build_symbol_table(&Some(Box::new(node)));
        println!("{:#?}", sym_table);
        Ok(())
    }
}
