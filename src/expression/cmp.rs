use cso_core::expression::ScalarExpression;
use cso_core::ColumnRefSet;

#[derive(Debug, Clone)]
pub struct Equal {
    left: Box<dyn ScalarExpression>,
    right: Box<dyn ScalarExpression>,
}

impl Equal {
    pub fn new(left: Box<dyn ScalarExpression>, right: Box<dyn ScalarExpression>) -> Self {
        Equal { left, right }
    }

    pub fn left(&self) -> &dyn ScalarExpression {
        self.left.as_ref()
    }

    pub fn right(&self) -> &dyn ScalarExpression {
        self.right.as_ref()
    }
}

impl ScalarExpression for Equal {
    fn is_boolean_expression(&self) -> bool {
        true
    }

    fn equal(&self, other: &dyn ScalarExpression) -> bool {
        match other.downcast_ref::<Equal>() {
            Some(other) => self.left.eq(&other.left) && self.right.eq(&other.right),
            None => false,
        }
    }

    fn derive_used_columns(&self, col_set: &mut ColumnRefSet) {
        self.left.derive_used_columns(col_set);
        self.right.derive_used_columns(col_set);
    }
}

#[derive(Debug, Clone)]
pub struct NotEqual {
    left: Box<dyn ScalarExpression>,
    right: Box<dyn ScalarExpression>,
}

impl NotEqual {
    pub fn new(left: Box<dyn ScalarExpression>, right: Box<dyn ScalarExpression>) -> Self {
        NotEqual { left, right }
    }

    pub fn left(&self) -> &dyn ScalarExpression {
        self.left.as_ref()
    }

    pub fn right(&self) -> &dyn ScalarExpression {
        self.right.as_ref()
    }
}

impl ScalarExpression for NotEqual {
    fn is_boolean_expression(&self) -> bool {
        true
    }

    fn equal(&self, other: &dyn ScalarExpression) -> bool {
        match other.downcast_ref::<NotEqual>() {
            Some(other) => self.left.eq(&other.left) && self.right.eq(&other.right),
            None => false,
        }
    }

    fn derive_used_columns(&self, col_set: &mut ColumnRefSet) {
        self.left.derive_used_columns(col_set);
        self.right.derive_used_columns(col_set);
    }
}

#[derive(Debug, Clone)]
pub struct GreaterThan {
    left: Box<dyn ScalarExpression>,
    right: Box<dyn ScalarExpression>,
}

impl GreaterThan {
    pub fn new(left: Box<dyn ScalarExpression>, right: Box<dyn ScalarExpression>) -> Self {
        GreaterThan { left, right }
    }

    pub fn left(&self) -> &dyn ScalarExpression {
        self.left.as_ref()
    }

    pub fn right(&self) -> &dyn ScalarExpression {
        self.right.as_ref()
    }
}

impl ScalarExpression for GreaterThan {
    fn is_boolean_expression(&self) -> bool {
        true
    }

    fn equal(&self, other: &dyn ScalarExpression) -> bool {
        match other.downcast_ref::<GreaterThan>() {
            Some(other) => self.left.eq(&other.left) && self.right.eq(&other.right),
            None => false,
        }
    }

    fn derive_used_columns(&self, col_set: &mut ColumnRefSet) {
        self.left.derive_used_columns(col_set);
        self.right.derive_used_columns(col_set);
    }
}

#[derive(Debug, Clone)]
pub struct LessThan {
    left: Box<dyn ScalarExpression>,
    right: Box<dyn ScalarExpression>,
}

impl LessThan {
    pub fn new(left: Box<dyn ScalarExpression>, right: Box<dyn ScalarExpression>) -> Self {
        LessThan { left, right }
    }

    pub fn left(&self) -> &dyn ScalarExpression {
        self.left.as_ref()
    }

    pub fn right(&self) -> &dyn ScalarExpression {
        self.right.as_ref()
    }
}

impl ScalarExpression for LessThan {
    fn is_boolean_expression(&self) -> bool {
        true
    }

    fn equal(&self, other: &dyn ScalarExpression) -> bool {
        match other.downcast_ref::<LessThan>() {
            Some(other) => self.left.eq(&other.left) && self.right.eq(&other.right),
            None => false,
        }
    }

    fn derive_used_columns(&self, col_set: &mut ColumnRefSet) {
        self.left.derive_used_columns(col_set);
        self.right.derive_used_columns(col_set);
    }
}

#[derive(Debug, Clone)]
pub struct GreaterThanEqual {
    left: Box<dyn ScalarExpression>,
    right: Box<dyn ScalarExpression>,
}

impl GreaterThanEqual {
    pub fn new(left: Box<dyn ScalarExpression>, right: Box<dyn ScalarExpression>) -> Self {
        GreaterThanEqual { left, right }
    }

    pub fn left(&self) -> &dyn ScalarExpression {
        self.left.as_ref()
    }

    pub fn right(&self) -> &dyn ScalarExpression {
        self.right.as_ref()
    }
}

impl ScalarExpression for GreaterThanEqual {
    fn is_boolean_expression(&self) -> bool {
        true
    }

    fn equal(&self, other: &dyn ScalarExpression) -> bool {
        match other.downcast_ref::<GreaterThanEqual>() {
            Some(other) => self.left.eq(&other.left) && self.right.eq(&other.right),
            None => false,
        }
    }

    fn derive_used_columns(&self, col_set: &mut ColumnRefSet) {
        self.left.derive_used_columns(col_set);
        self.right.derive_used_columns(col_set);
    }
}

#[derive(Debug, Clone)]
pub struct LessThanEqual {
    left: Box<dyn ScalarExpression>,
    right: Box<dyn ScalarExpression>,
}

impl LessThanEqual {
    pub fn new(left: Box<dyn ScalarExpression>, right: Box<dyn ScalarExpression>) -> Self {
        LessThanEqual { left, right }
    }

    pub fn left(&self) -> &dyn ScalarExpression {
        self.left.as_ref()
    }

    pub fn right(&self) -> &dyn ScalarExpression {
        self.right.as_ref()
    }
}

impl ScalarExpression for LessThanEqual {
    fn is_boolean_expression(&self) -> bool {
        true
    }

    fn equal(&self, other: &dyn ScalarExpression) -> bool {
        match other.downcast_ref::<LessThanEqual>() {
            Some(other) => self.left.eq(&other.left) && self.right.eq(&other.right),
            None => false,
        }
    }

    fn derive_used_columns(&self, col_set: &mut ColumnRefSet) {
        self.left.derive_used_columns(col_set);
        self.right.derive_used_columns(col_set);
    }
}
