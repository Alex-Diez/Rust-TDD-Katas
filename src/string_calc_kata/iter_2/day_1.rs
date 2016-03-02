use std::result::Result;
use std::num::ParseFloatError;

pub fn evaluate(src: &str) -> Result<f32, ParseFloatError> {
    src.parse::<f32>()
}
