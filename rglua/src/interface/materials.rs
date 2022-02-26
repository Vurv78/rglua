use super::prelude::*;
use viable::vtable;

#[vtable]
/// You do not get this through creating an interface, it is instead exported by other interface functions.
pub struct Material {
    pub GetName: extern "C" fn() -> *const c_char,
    pub GetTextureGroupName: extern "C" fn() -> *const c_char,

    #[offset(17)]
    pub IsTranslucent: extern "C" fn() -> bool,
    pub IsAlphaTested: extern "C" fn() -> bool,
    pub IsVertexLit: extern "C" fn() -> bool,

    #[cfg(feature = "userdata")]
    #[offset(31)]
    pub GetReflectivity: extern "C" fn(reflect: &mut Vector),

    #[offset(34)]
    pub SetShader: extern "C" fn(shader: *const c_char),

    #[offset(37)]
    pub Refresh: extern "C" fn(),

    #[offset(42)]
    pub IsErrorMaterial: extern "C" fn() -> bool,
}

pub type MaterialHandle = c_ushort;

/// "VMaterialSystem080"
/// "materialsystem.dll"
#[vtable]
pub struct MaterialSystem {
    #[offset(72)]
    pub ReloadTextures: extern "C" fn(),
    pub ReloadMaterials: extern "C" fn(pSubString: *const c_char),

    pub CreateMaterial:
        extern "C" fn(mat_name: *const c_char, vmt_kv: *const c_void) -> *mut c_void,
    pub FindMaterial: extern "C" fn(
        mat_name: *const c_char,
        texture_group_name: *const c_char,
        complain: bool,
        complain_prefix: *const c_char,
    ) -> *mut c_void,

    #[offset(77)]
    pub FirstMaterial: extern "C" fn() -> MaterialHandle,
    pub NextMaterial: extern "C" fn(handle: MaterialHandle) -> MaterialHandle,
    pub InvalidMaterial: extern "C" fn() -> MaterialHandle,
    pub GetMaterial: extern "C" fn(handle: MaterialHandle) -> *mut c_void,
}
