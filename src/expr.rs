use crate::parser::{Token, Object};
pub trait Expr<T> {
        fn accept(&self, visitor: Box<dyn Visitor<T>>) -> T;
}
pub struct Binary<T> {
pub left: Box<dyn Expr<T>>,
pub operator: Token,
pub right: Box<dyn Expr<T>>,
}
impl<T> Expr<T> for Binary<T> {
                fn accept(&self, visitor: Box<dyn Visitor<T>>) -> T {
                    return visitor.visit_Binary_Expr(self);
                }
}pub struct Grouping<T> {
pub expression: Box<dyn Expr<T>>,
}
impl<T> Expr<T> for Grouping<T> {
                fn accept(&self, visitor: Box<dyn Visitor<T>>) -> T {
                    return visitor.visit_Grouping_Expr(self);
                }
}pub struct Literal {
pub value: Object,
}
impl<T> Expr<T> for Literal {
                fn accept(&self, visitor: Box<dyn Visitor<T>>) -> T {
                    return visitor.visit_Literal_Expr(self);
                }
}pub struct Unary<T> {
pub operator: Token,
pub right: Box<dyn Expr<T>>,
}
impl<T> Expr<T> for Unary<T> {
                fn accept(&self, visitor: Box<dyn Visitor<T>>) -> T {
                    return visitor.visit_Unary_Expr(self);
                }
}pub trait Visitor<T>{
fn visit_Binary_Expr(&self, expr: &Binary<T>) -> T;
fn visit_Grouping_Expr(&self, expr: &Grouping<T>) -> T;
fn visit_Literal_Expr(&self, expr: &Literal) -> T;
fn visit_Unary_Expr(&self, expr: &Unary<T>) -> T;
}