use std::os::raw::c_char;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IType_api {
    _unused: [u8; 0],
}
pub type IType = *mut IType_api;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ITypeFactory_api {
    _unused: [u8; 0],
}
pub type ITypeFactory = *mut ITypeFactory_api;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ITypeResource_api {
    _unused: [u8; 0],
}
pub type ITypeResource = *mut ITypeResource_api;

extern "C" {
    pub fn IType_release(s: IType);
    pub fn IType_retain(s: IType);
    pub fn IType_get_kind(s: IType) -> Kind;

    pub fn IType_factory_release(f: ITypeFactory);
    pub fn IType_factory_retain(f: ITypeFactory);
    pub fn IType_factory_dump(
        f: ITypeFactory,
        typ: IType,
        depth: usize,
    ) -> *const c_char;
}

#[repr(i32)]
#[derive(Debug)]
pub enum Kind {
    // An alias for another type, aka typedef. See #mi::neuraylib::IType_alias.
    Alias,
    // The \c boolean type. See #mi::neuraylib::IType_bool.
    Bool,
    // The \c integer type. See #mi::neuraylib::IType_int.
    Int,
    // An \c enum type. See #mi::neuraylib::IType_enum.
    Enum,
    // The \c float type. See #mi::neuraylib::IType_float.
    Float,
    // The \c double type. See #mi::neuraylib::IType_double.
    Double,
    //  The \c string type. See #mi::neuraylib::IType_string.
    String,
    // A vector type. See #mi::neuraylib::IType_vector.
    Vector,
    // A matrix type. See #mi::neuraylib::IType_matrix.
    Matrix,
    // The color type. See #mi::neuraylib::IType_color.
    Color,
    // An array type. See #mi::neuraylib::IType_array.
    Array,
    // A struct type. See #mi::neuraylib::IType_struct.
    Struct,
    // A texture type. See #mi::neuraylib::IType_texture.
    Texture,
    // The \c light_profile type. See #mi::neuraylib::IType_light_profile.
    LightProfile,
    // The \c bsdf_measurement type. See #mi::neuraylib::IType_bsdf_measurement.
    BsdfMeasurement,
    // The \c bsdf type. See #mi::neuraylib::IType_bsdf.
    Bsdf,
    // The \c edf type. See #mi::neuraylib::IType_edf.
    Edf,
    // The \c vdf type. See #mi::neuraylib::IType_vdf.
    Vdf,
}
