use crate::expr::{Binary, Expr, Grouping, Literal, Unary, Visitor};
use std::clone::Clone;
pub trait OutlinePrint {}

#[derive(Clone)]
pub struct ASTPrinter {}

fn parenthesize(
    name: String,
    exprs: Vec<&Box<dyn Expr<String>>>,
    visitor: Vec<Box<dyn Visitor<String>>>,
) -> String {
    let mut string = String::new();

    string.push('(');
    string.push_str(&name);
    let bb = exprs.iter().zip(visitor);
    for (expr, visitor) in bb {
        string.push(' ');
        // let ss: Box<dyn Visitor<String>> = visitor;
        // string.push_str(&expr.accept(&Box::new(self)))
        string.push_str(&expr.accept(visitor))
    }

    string.push(')');
    string
}

impl ASTPrinter {
    //fn parenthesize(&self, name: String, exprs: Vec<Box<dyn Expr<String>>>) -> String {
    //    let mut string = String::new();

    //    string.push('(');
    //    string.push_str(&name);
    //    for expr in exprs {
    //        string.push(' ');
    //        let ss: Box<dyn Visitor<String>> = Box::new(self);
    //        // string.push_str(&expr.accept(&Box::new(self)))
    //        string.push_str(&expr.accept(&ss))
    //    }

    //    string.push(')');
    //    string
    //}

    pub fn print(&self, expr: Box<dyn Expr<String>>) -> String {
        let visitor: Box<dyn Visitor<String>> = Box::new(self.clone());
        expr.accept(visitor)
    }
}

impl Visitor<String> for ASTPrinter {
    fn visit_Binary_Expr(&self, expr: &Binary<String>) -> String {
        let visitors: Vec<Box<dyn Visitor<String>>> =
            vec![Box::new(self.clone()), Box::new(self.clone())];
        parenthesize(
            expr.operator.lexeme.clone(),
            vec![&expr.left, &expr.right],
            visitors,
        )
    }

    fn visit_Grouping_Expr(&self, expr: &Grouping<String>) -> String {
        parenthesize(
            String::from("group"),
            vec![&expr.expression],
            vec![Box::new(self.clone())],
        )
    }

    fn visit_Literal_Expr(&self, expr: &Literal) -> String {
        // if expr.value == null - in reference, but null doesn't exist in Rust
        expr.value.to_string()
    }

    fn visit_Unary_Expr(&self, expr: &Unary<String>) -> String {
        parenthesize(
            expr.operator.lexeme.to_string(),
            vec![&expr.right],
            vec![Box::new(self.clone())],
        )
    }
}

impl Visitor<String> for &ASTPrinter {
    fn visit_Binary_Expr(&self, expr: &Binary<String>) -> String {
        let cop = self.clone();

        let visitors: Vec<Box<dyn Visitor<String>>> =
            vec![Box::new(cop.clone()), Box::new(cop.clone())];

        parenthesize(
            expr.operator.lexeme.clone(),
            vec![&expr.left, &expr.right],
            visitors,
        )
    }

    fn visit_Grouping_Expr(&self, expr: &Grouping<String>) -> String {
        "".into()
    }

    fn visit_Literal_Expr(&self, expr: &Literal) -> String {
        "".into()
    }

    fn visit_Unary_Expr(&self, expr: &Unary<String>) -> String {
        "".into()
    }
}
