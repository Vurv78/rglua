use super::common::StudioHdr;
use super::prelude::*;

#[repr(C)]
pub enum MDLCacheDataType {
	// Callbacks to get called when data is loaded or unloaded for these:
	StudioHDR = 0,
	StudioHWData,
	VCollide,

	// Callbacks NOT called when data is loaded or unloaded for these:
	AnimBlock,
	VirtualModel,
	Vertexes,
	DecodedAnimBlock
}

#[vtable]
pub struct MdlCacheNotify {
	/// Called right after data is loaded
	pub OnDataLoaded: extern "C" fn(ty: MDLCacheDataType, handle: MDLHandle),
	/// Called right before data is unloaded
	pub OnDataUnloaded: extern "C" fn(ty: MDLCacheDataType, handle: MDLHandle)
}

pub type MDLHandle = c_ushort;
pub type VirtualModel = c_void; // Todo?

const MAX_NUM_LODS: usize = 8;

#[repr(C)]
pub struct VertexFileHeader {
	id: c_int,
	version: c_int,
	checksum: c_int,
	numLODs: c_int,
	numLODVertexes: [c_int; MAX_NUM_LODS],
	numFixups: c_int,
	fixupTableStart: c_int,
	vertexDataStart: c_int,
	tangentDataStart: c_int
}

#[vtable]
/// "MDLCache004"
/// "datacache"
pub struct MdlCache {
	pub SetCacheNotify: extern "C" fn(pNotify: *mut MdlCacheNotify),
	pub FindMDL: extern "C" fn(pMDLRelativePath: *const c_char) -> MDLHandle,
	pub AddRef: extern "C" fn(handle: MDLHandle) -> c_int,
	pub Release: extern "C" fn(handle: MDLHandle) -> c_int,
	pub GetRef: extern "C" fn(handle: MDLHandle) -> c_int,
	pub GetStudioHdr: extern "C" fn(handle: MDLHandle) -> *mut StudioHdr,

	#[offset(9)]
	pub GetVirtualModel: extern "C" fn(handle: MDLHandle) -> *mut VirtualModel,

	#[offset(11)]
	pub GetVertexData: extern "C" fn(handle: MDLHandle) -> *mut VertexFileHeader,
	pub TouchAllData: extern "C" fn(handle: MDLHandle) -> (),
	pub SetUserData: extern "C" fn(handle: MDLHandle, pData: *mut c_void) -> (),
	pub GetUserData: extern "C" fn(handle: MDLHandle) -> *mut c_void,
	pub IsErrorModel: extern "C" fn(handle: MDLHandle) -> bool,

	#[offset(18)]
	pub GetModelName: extern "C" fn(handle: MDLHandle) -> *const c_char,
	pub GetVirtualModelFast:
		extern "C" fn(pStudioHdr: *const StudioHdr, handle: MDLHandle) -> *mut VirtualModel,
	pub BeginLock: extern "C" fn(),
	pub EndLock: extern "C" fn()
}
