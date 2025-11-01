use crate::v2::objects::cubic_spline_parameter::CubicSplineParameter;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct CubicSpline {
    pub parameters: Vec<CubicSplineParameter>,

    pub path_definition_name: String,
}
