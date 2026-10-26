//! Pure logic validators. No FLTK dependency.
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationError {
    Empty,
    OutOfRange { min: f64, max: f64 },
    Custom(String),
}

pub trait Validator: Send + Sync {
    fn validate(&self, input: &str) -> Result<(), ValidationError>;
}

pub struct Required;
impl Validator for Required {
    fn validate(&self, input: &str) -> Result<(), ValidationError> {
        if input.trim().is_empty() {
            Err(ValidationError::Empty)
        } else {
            Ok(())
        }
    }
}

pub struct NumberRange {
    pub min: f64,
    pub max: f64,
}
impl Validator for NumberRange {
    fn validate(&self, input: &str) -> Result<(), ValidationError> {
        if let Ok(num) = input.trim().parse::<f64>() {
            if num >= self.min && num <= self.max {
                Ok(())
            } else {
                Err(ValidationError::OutOfRange {
                    min: self.min,
                    max: self.max,
                })
            }
        } else {
            Err(ValidationError::Custom("Not a valid number".into()))
        }
    }
}
