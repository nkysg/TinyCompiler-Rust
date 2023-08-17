use crate::ast::ExpressionKind::Opk;
use crate::ast::StatementKind::{AssignK, ReadK, WriteK};
use crate::ast::{Attr, ExpressionKind, StatementKind, TreeNode};
use crate::token::Token;
use anyhow::Result;

pub struct Parser {
    tokens: Vec<Token>,
    cur_idx: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, cur_idx: 0 }
    }

    pub fn parse(&mut self) -> Result<TreeNode> {
        self.stmt_sequence()
    }

    fn get_token(&mut self) -> Option<&Token> {
        self.cur_idx += 1;
        self.tokens.get(self.cur_idx - 1)
    }

    fn token_ref(&self) -> &Token {
        &self.tokens[self.cur_idx]
    }

    // stmt-sequence -> statement {; statement }
    fn stmt_sequence(&mut self) -> Result<TreeNode> {
        let mut t: Option<Box<TreeNode>> = Some(Box::new(self.statement()?));
        let mut p = t.as_mut().unwrap();
        while self.token_ref() != &Token::EndFile
            && self.token_ref() != &Token::End
            && self.token_ref() != &Token::Else
            && self.token_ref() != &Token::Until
        {
            self.match_token(Token::Semi);
            let q = self.statement()?;
            p.sibling = Some(Box::new(q));
            p = p.sibling.as_mut().unwrap();
        }
        Ok(*(t.unwrap()))
    }

    // statement -> if-stmt | repeat-stmt | assign-stmt | read-stmt | write-stmt
    fn statement(&mut self) -> Result<TreeNode> {
        let token = self.token_ref();
        let t = match *token {
            Token::If => self.if_stmt()?,
            Token::Repeat => self.repeat_stmt()?,
            Token::Id(_) => self.assign_stmt()?,
            Token::Read => self.read_stmt()?,
            Token::Write => self.write_stmt()?,
            _ => return Err(anyhow::format_err!("{}", token)),
        };
        Ok(t)
    }

    // if-stmt -> if exp then stmt-sequence | else stmt-sequence | end
    fn if_stmt(&mut self) -> Result<TreeNode> {
        let mut t = TreeNode::new_statement_node(StatementKind::IfK);
        self.match_token(Token::If);
        t.child[0] = Some(Box::new(self.expr()?));
        self.match_token(Token::Then);
        t.child[1] = Some(Box::new(self.stmt_sequence()?));
        let token = self.token_ref();
        if token == &Token::Else {
            self.match_token(Token::Else);
            t.child[2] = Some(Box::new(self.stmt_sequence()?));
        }
        self.match_token(Token::End);
        Ok(t)
    }

    // repeat-smt -> repeat smt-sequence until expr
    fn repeat_stmt(&mut self) -> Result<TreeNode> {
        let mut t = TreeNode::new_statement_node(StatementKind::RepeatK);
        self.match_token(Token::Repeat);
        t.child[0] = Some(Box::new(self.stmt_sequence()?));
        self.match_token(Token::Until);
        t.child[1] = Some(Box::new(self.expr()?));
        Ok(t)
    }

    // assign_stmt -> id := expr
    fn assign_stmt(&mut self) -> Result<TreeNode> {
        let mut t = TreeNode::new_statement_node(AssignK);
        let token = self.token_ref();
        if let Token::Id(id) = token {
            t.attr = Attr::Name(id.clone());
        }
        self.match_token(token.clone());
        self.match_token(Token::Assign);
        t.child[0] = Some(Box::new(self.expr()?));
        Ok(t)
    }

    // read_smt = read id
    fn read_stmt(&mut self) -> Result<TreeNode> {
        let mut t = TreeNode::new_statement_node(ReadK);
        self.match_token(Token::Read);
        let token = self.get_token().cloned();
        if let Some(Token::Id(id)) = token {
            t.attr = Attr::Name(id);
        }
        Ok(t)
    }

    // write_smt = write expr
    fn write_stmt(&mut self) -> Result<TreeNode> {
        let mut t = TreeNode::new_statement_node(WriteK);
        self.match_token(Token::Write);
        t.child[0] = Some(Box::new(self.expr()?));
        Ok(t)
    }

    // expr -> simple_exp ["<" simple-exp | "=" simple-exp]
    fn expr(&mut self) -> Result<TreeNode> {
        let mut t = self.simple_expr()?;
        let token = self.token_ref();
        if token == &Token::Lt || token == &Token::Eq {
            let mut p = TreeNode::new_expression_node(Opk);
            p.child[0] = Some(Box::new(t));
            p.attr = Attr::Op(token.clone());
            t = p;
            self.match_token(token.clone());
            t.child[1] = Some(Box::new(self.simple_expr()?));
        }
        Ok(t)
    }

    // simple_expr = term { "+" term | "-" term }*
    fn simple_expr(&mut self) -> Result<TreeNode> {
        let mut t = self.term()?;
        while self.token_ref() == &Token::Plus || self.token_ref() == &Token::Minus {
            let token = self.token_ref();
            let mut p = TreeNode::new_expression_node(Opk);
            p.child[0] = Some(Box::new(t));
            p.attr = Attr::Op(token.clone());
            t = p;
            self.match_token(token.clone());
            t.child[0] = Some(Box::new(self.term()?));
        }
        Ok(t)
    }

    // term = factor { " * " factor | " / " factor)*
    fn term(&mut self) -> Result<TreeNode> {
        let mut t = self.factor()?;
        while self.token_ref() == &Token::Times || self.token_ref() == &Token::Over {
            let mut p = TreeNode::new_expression_node(ExpressionKind::Opk);
            p.child[0] = Some(Box::new(t));
            let token = self.token_ref();
            p.attr = Attr::Op(token.clone());
            t = p;
            self.match_token(token.clone());
            t.child[1] = Some(Box::new(self.factor()?));
        }
        Ok(t)
    }

    // factor = NUM | ID | (exp)
    fn factor(&mut self) -> Result<TreeNode> {
        let mut t: TreeNode;
        let token = self.token_ref().clone();
        match token {
            Token::Num(ref str) => {
                t = TreeNode::new_expression_node(ExpressionKind::ConstK);
                t.attr = Attr::Val(str.parse::<i32>()?);
                self.match_token(token.clone());
            }
            Token::Id(ref id) => {
                t = TreeNode::new_expression_node(ExpressionKind::IdK);
                t.attr = Attr::Name(id.clone());
                self.match_token(token.clone());
            }
            Token::Lparen => {
                self.match_token(Token::Lparen);
                t = self.expr()?;
                self.match_token(Token::Rparen);
            }
            _ => {
                self.get_token();
                return Err(anyhow::format_err!("{}", token));
            }
        }
        Ok(t)
    }

    fn match_token(&mut self, expected: Token) -> bool {
        let token = self.token_ref();
        if token.clone() == expected {
            self.get_token();
            return true;
        }
        println!("token {}, expected {}", token, expected);
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::Parser;
    use crate::token::Token;

    #[test]
    fn test_parse() {
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
        let node = parser.parse();
        assert!(node.is_ok());
    }
}
