use cso_core::expression::ScalarExpression;

#[derive(Clone, Debug)]
pub struct IsNull {
    inner: Box<dyn ScalarExpression>,
}

impl IsNull {
    pub fn new(inner: Box<dyn ScalarExpression>) -> Self {
        Self { inner }
    }
}

impl ScalarExpression for IsNull {
    fn is_boolean_expression(&self) -> bool {
        true
    }

    fn equal(&self, other: &dyn ScalarExpression) -> bool {
        match other.downcast_ref::<IsNull>() {
            Some(other) => self.inner.eq(&other.inner),
            None => false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct IsNotNull {
    inner: Box<dyn ScalarExpression>,
}

impl IsNotNull {
    pub fn new(inner: Box<dyn ScalarExpression>) -> Self {
        Self { inner }
    }
}

impl ScalarExpression for IsNotNull {
    fn is_boolean_expression(&self) -> bool {
        true
    }

    fn equal(&self, other: &dyn ScalarExpression) -> bool {
        match other.downcast_ref::<IsNotNull>() {
            Some(other) => self.inner.eq(&other.inner),
            None => false,
        }
    }
}
