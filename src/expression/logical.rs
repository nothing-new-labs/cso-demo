use cso_core::expression::ScalarExpression;

#[derive(Debug, Clone)]
pub struct And {
    expressions: Vec<Box<dyn ScalarExpression>>,
}

impl And {
    pub fn new(expressions: Vec<Box<dyn ScalarExpression>>) -> And {
        assert!(expressions.iter().all(|expr| expr.is_boolean_expression()));
        And { expressions }
    }
}

impl ScalarExpression for And {
    fn equal(&self, other: &dyn ScalarExpression) -> bool {
        match other.downcast_ref::<And>() {
            Some(other) => self.expressions == other.expressions,
            None => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Or {
    expressions: Vec<Box<dyn ScalarExpression>>,
}

impl Or {
    pub fn new(expressions: Vec<Box<dyn ScalarExpression>>) -> Or {
        assert!(expressions.iter().all(|expr| expr.is_boolean_expression()));
        Or { expressions }
    }
}

impl ScalarExpression for Or {
    fn equal(&self, other: &dyn ScalarExpression) -> bool {
        match other.downcast_ref::<Or>() {
            Some(other) => self.expressions == other.expressions,
            None => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Not {
    expression: Box<dyn ScalarExpression>,
}

impl Not {
    pub fn new(expression: Box<dyn ScalarExpression>) -> Not {
        assert!(expression.is_boolean_expression());
        Not { expression }
    }
}

impl ScalarExpression for Not {
    fn equal(&self, other: &dyn ScalarExpression) -> bool {
        match other.downcast_ref::<Not>() {
            Some(other) => self.expression.eq(&other.expression),
            None => false,
        }
    }
}