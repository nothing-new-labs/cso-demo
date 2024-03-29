use cso_core::expression::ScalarExpression;
use cso_core::ColumnRefSet;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct And {
    expressions: Vec<Rc<dyn ScalarExpression>>,
}

impl And {
    pub fn new(expressions: Vec<Rc<dyn ScalarExpression>>) -> And {
        assert!(expressions.iter().all(|expr| expr.is_boolean_expression()));
        And { expressions }
    }

    pub fn expressions(&self) -> &[Rc<dyn ScalarExpression>] {
        &self.expressions
    }
}

impl ScalarExpression for And {
    fn is_boolean_expression(&self) -> bool {
        true
    }

    fn equal(&self, other: &dyn ScalarExpression) -> bool {
        match other.downcast_ref::<And>() {
            Some(other) => self.expressions == other.expressions,
            None => false,
        }
    }

    fn derive_used_columns(&self, col_set: &mut ColumnRefSet) {
        self.expressions.iter().for_each(|e| e.derive_used_columns(col_set));
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
    fn is_boolean_expression(&self) -> bool {
        true
    }

    fn equal(&self, other: &dyn ScalarExpression) -> bool {
        match other.downcast_ref::<Or>() {
            Some(other) => self.expressions == other.expressions,
            None => false,
        }
    }

    fn derive_used_columns(&self, col_set: &mut ColumnRefSet) {
        self.expressions.iter().for_each(|e| e.derive_used_columns(col_set));
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
    fn is_boolean_expression(&self) -> bool {
        true
    }
    fn equal(&self, other: &dyn ScalarExpression) -> bool {
        match other.downcast_ref::<Not>() {
            Some(other) => self.expression.eq(&other.expression),
            None => false,
        }
    }

    fn derive_used_columns(&self, col_set: &mut ColumnRefSet) {
        self.expression.derive_used_columns(col_set);
    }
}
