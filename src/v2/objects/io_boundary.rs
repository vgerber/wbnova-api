use crate::v2::objects::integer_value::IntegerValue;

use crate::v2::objects::float_value::FloatValue;

use crate::v2::objects::boolean_value::BooleanValue;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub enum IoBoundary {
    IntegerValueValue(IntegerValue),

    FloatValueValue(FloatValue),

    BooleanValueValue(BooleanValue),
}
