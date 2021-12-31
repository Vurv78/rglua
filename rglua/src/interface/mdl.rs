use super::common::StudioHdr;
use super::prelude::*;

iface! {
	#[version("MDLCache004")]
	#[file("datacache.dll")]
	pub abstract struct IMdlCache {};

	#[version("")]
	#[file("")]
	pub abstract struct IMdlCacheNotify {};
}

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
	DecodedAnimBlock,
}

impl IMdlCacheNotify {
	#[virtual_index(0)]
	/// Called right after data is loaded
	pub fn OnDataLoaded(&self, ty: MDLCacheDataType, handle: MDLHandle) -> () {}

	#[virtual_index(1)]
	/// Called right before data is unloaded
	pub fn OnDataUnloaded(&self, ty: MDLCacheDataType, handle: MDLHandle) -> () {}
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
	tangentDataStart: c_int,
}

impl IMdlCache {
	#[virtual_index(0)]
	pub fn SetCacheNotify(&self, pNotify: *mut IMdlCacheNotify) -> () {}

	#[virtual_index(1)]
	pub fn FindMDL(&self, pMDLRelativePath: *const c_char) -> MDLHandle {}

	#[virtual_index(2)]
	pub fn AddRef(&self, handle: MDLHandle) -> c_int {}

	#[virtual_index(3)]
	pub fn Release(&self, handle: MDLHandle) -> c_int {}

	#[virtual_index(4)]
	pub fn GetRef(&self, handle: MDLHandle) -> c_int {}

	#[virtual_index(5)]
	pub fn GetStudioHdr(&self, handle: MDLHandle) -> *mut StudioHdr {}

	#[virtual_index(9)]
	pub fn GetVirtualModel(&self, handle: MDLHandle) -> *mut VirtualModel {}

	#[virtual_index(11)]
	pub fn GetVertexData(&self, handle: MDLHandle) -> *mut VertexFileHeader {}

	#[virtual_index(12)]
	pub fn TouchAllData(&self, handle: MDLHandle) -> () {}

	#[virtual_index(13)]
	pub fn SetUserData(&self, handle: MDLHandle, pData: *mut c_void) -> () {}

	#[virtual_index(14)]
	pub fn GetUserData(&self, handle: MDLHandle) -> *mut c_void {}

	#[virtual_index(15)]
	pub fn IsErrorModel(&self, handle: MDLHandle) -> bool {}

	#[virtual_index(18)]
	pub fn GetModelName(&self, handle: MDLHandle) -> *const c_char {}

	#[virtual_index(19)]
	pub fn GetVirtualModelFast(
		&self,
		pStudioHdr: *const StudioHdr,
		handle: MDLHandle,
	) -> *mut VirtualModel {
	}

	#[virtual_index(20)]
	pub fn BeginLock(&self) -> () {}

	#[virtual_index(21)]
	pub fn EndLock(&self) -> () {}
}
