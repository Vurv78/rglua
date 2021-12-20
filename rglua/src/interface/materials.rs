use super::prelude::*;

iface! {
	pub abstract struct IMaterial {};
	pub abstract struct IMaterialSystem {};
}

impl IMaterial {
	#[virtual_index(0)]
	pub fn GetName(&self) -> *const c_char {}

	#[virtual_index(1)]
	pub fn GetTextureGroupName(&self) -> *const c_char {}

	#[virtual_index(17)]
	pub fn IsTranslucent(&self) -> bool {}

	#[virtual_index(18)]
	pub fn IsAlphaTested(&self) -> bool {}

	#[virtual_index(19)]
	pub fn IsVertexLit(&self) -> bool {}

	#[virtual_index(31)]
	#[cfg(feature = "userdata")]
	pub fn GetReflectivity(&self, reflect: &mut Vector) {}

	#[virtual_index(34)]
	pub fn SetShader(&self, shader: *const c_char) {}

	#[virtual_index(37)]
	pub fn Refresh(&self) {}

	#[virtual_index(42)]
	pub fn IsErrorMaterial(&self) -> bool {}
}

pub type MaterialHandle = c_ushort;

// Extends IAppSystem
impl IMaterialSystem {
	#[virtual_index(70)]
	pub fn ReloadTextures(&self) {}

	#[virtual_index(71)]
	pub fn ReloadMaterials(&self, pSubString: *const c_char) {}

	/// Create a procedural material. The keyvalues looks like a VMT file
	/// # Returns
	/// IMaterial pointer
	#[virtual_index(72)]
	pub fn CreateMaterial(
		&self,
		pMaterialName: *const c_char,
		pVMTKeyValues: *mut c_void,
	) -> *mut c_void {
	}

	#[virtual_index(73)]
	pub fn FindMaterial(
		&self,
		pMaterialName: *const c_char,
		pTextureGroupName: *const c_char,
		complain: bool,
		pComplainPrefix: *const c_char,
	) -> *mut c_void {
	}

	/// This is the interface for knowing what materials are available
	/// is to use the following functions to get a list of materials.  The
	/// material names will have the full path to the material, and that is the
	/// only way that the directory structure of the materials will be seen through this
	/// interface.
	/// NOTE: Mostly for WoW so may not apply here
	#[virtual_index(75)]
	pub fn FirstMaterial(&self) -> MaterialHandle {}

	/// If there is no other material, returns InvalidMaterial
	#[virtual_index(76)]
	pub fn NextMaterial(&self, h: MaterialHandle) -> MaterialHandle {}

	/// Invalid material 👍
	#[virtual_index(77)]
	pub fn InvalidMaterial(&self) -> MaterialHandle {}

	/// Returns a particular material
	#[virtual_index(78)]
	pub fn GetMaterial(&self, h: MaterialHandle) -> *const c_void {}
}
