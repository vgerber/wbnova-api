use crate::v2::objects::safety_geometry_capsule::SafetyGeometryCapsule;

use crate::v2::objects::safety_geometry_box::SafetyGeometryBox;

use crate::v2::objects::safety_geometry_lozenge::SafetyGeometryLozenge;

use crate::v2::objects::safety_geometry_plane::SafetyGeometryPlane;

use crate::v2::objects::safety_geometry_prism::SafetyGeometryPrism;

use crate::v2::objects::safety_geometry_sphere::SafetyGeometrySphere;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct SafetyGeometry {
    pub capsule: Option<SafetyGeometryCapsule>,

    #[serde(alias = "box")]
    pub safety_geometry_box: Option<SafetyGeometryBox>,

    pub id: String,

    pub lozenge: Option<SafetyGeometryLozenge>,

    pub plane: Option<SafetyGeometryPlane>,

    pub prism: Option<SafetyGeometryPrism>,

    pub sphere: Option<SafetyGeometrySphere>,

    pub inverted: bool,
}
