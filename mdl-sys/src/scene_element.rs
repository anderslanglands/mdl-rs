use std::os::raw::c_char;

use crate::base::Uuid;

#[derive(Debug)]
#[repr(u32)]
pub enum ElementType {
    Instance = 0,
    Group = 1,
    Options = 2,
    Camera = 3,
    Light = 4,
    LightProfile = 5,
    Texture = 7,
    Image = 8,
    TriangleMesh = 10,
    AttributeContainer = 16,
    PolygonMesh = 18,
    SubdivisionSurface = 23,
    FreeformSurface = 24,
    Module = 29,
    FunctionDefinition = 30,
    FunctionCall = 31,
    MaterialDefinition = 32,
    MaterialInstance = 33,
    CompiledMaterial = 34,
    BsdfMeasurement = 35,
    IrradianceProbes = 36,
    Decal = 37,
    OnDemandMesh = 38,
    Projector = 39,
    SectionObject = 40,
    Proxy = 41,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ISceneElement_api {
    _unused: [u8; 0],
}
pub type ISceneElement = *mut ISceneElement_api;

extern "C" {
    pub fn IScene_element_release(s: ISceneElement);
    pub fn IScene_element_retain(s: ISceneElement);
    pub fn IScene_element_compare_iid(id: Uuid) -> bool;
    pub fn IScene_element_type_get_iid() -> Uuid;
    pub fn IScene_element_get_element_type(se: ISceneElement) -> ElementType;
}