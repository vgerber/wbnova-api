use crate::v2::objects::io_boolean_value::IoBooleanValue;

use crate::v2::objects::io_float_value::IoFloatValue;

use crate::v2::objects::io_integer_value::IoIntegerValue;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub enum IoValue {
    IoBooleanValueValue(IoBooleanValue),

    IoFloatValueValue(IoFloatValue),

    IoIntegerValueValue(IoIntegerValue),
}
