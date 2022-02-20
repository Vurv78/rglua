use super::prelude::*;
use crate::userdata::{Angle, Vector};

use crate::lua::LuaCFunction;

#[vtable]
pub struct LuaBase {
	pub state: crate::types::LuaState,

	/// Returns the amount of values on the stack
	pub Top: extern "C" fn() -> c_int,
	pub Push: extern "C" fn(iStackPos: c_int),
	pub Pop: extern "C" fn(iAmt: c_int),
	pub GetTable: extern "C" fn(iStackPos: c_int),
	pub GetField: extern "C" fn(iStackPos: c_int, strName: *const c_char),
	pub SetField: extern "C" fn(iStackPos: c_int, strName: *const c_char),
	pub CreateTable: extern "C" fn(),
	pub SetTable: extern "C" fn(iStackPos: c_int),
	pub SetMetaTable: extern "C" fn(iStackPos: c_int),
	pub GetMetaTable: extern "C" fn(iStackPos: c_int) -> bool,
	pub Call: extern "C" fn(iArgs: c_int, iResults: c_int),
	pub PCall: extern "C" fn(iArgs: c_int, iResults: c_int, iErrorFunc: c_int) -> c_int,
	pub Equal: extern "C" fn(iStackPos1: c_int, iStackPos2: c_int) -> c_int,
	pub RawEqual: extern "C" fn(iStackPos1: c_int, iStackPos2: c_int) -> c_int,
	pub Insert: extern "C" fn(iStackPos: c_int),
	pub Remove: extern "C" fn(iStackPos: c_int),
	pub Next: extern "C" fn(iStackPos: c_int),
	#[deprecated = "Use usertype functions"]
	pub NewUserdata: extern "C" fn(iSize: c_uint) -> *mut c_void,
	pub ThrowError: extern "C" fn(strError: *const c_char) -> !,
	pub CheckType: extern "C" fn(iStackPos: c_int, iType: c_int),
	pub ArgError: extern "C" fn(iArgNum: c_int, strMessage: *const c_char) -> !,
	pub RawGet: extern "C" fn(iStackPos: c_int),
	pub RawSet: extern "C" fn(iStackPos: c_int),
	pub GetString: extern "C" fn(iStackPos: c_int, iOutLen: *mut c_uint) -> *const c_char,
	pub GetNumber: extern "C" fn(iStackPos: c_int) -> c_double,
	pub GetBool: extern "C" fn(iStackPos: c_int) -> bool,
	pub GetCFunction: extern "C" fn(iStackPos: c_int) -> LuaCFunction,
	#[deprecated = "Use usertype functions"]
	pub GetUserdata: extern "C" fn(iStackPos: c_int) -> *mut c_void,

	pub PushNil: extern "C" fn(),
	pub PushString: extern "C" fn(val: *const c_char, ilen: c_uint),
	pub PushNumber: extern "C" fn(val: c_double),
	pub PushBool: extern "C" fn(val: bool),
	pub PushCFunction: extern "C" fn(val: LuaCFunction),
	pub PushCClosure: extern "C" fn(val: LuaCFunction, iUpValues: c_int),
	#[deprecated]
	pub PushUserdata: extern "C" fn(val: *mut c_void),

	pub ReferenceCreate: extern "C" fn(iType: c_int) -> c_int,
	pub ReferenceFree: extern "C" fn(iRef: c_int),
	pub ReferencePush: extern "C" fn(iRef: c_int),
	pub PushSpecial: extern "C" fn(iType: c_int),
	pub IsType: extern "C" fn(iStackPos: c_int, iType: c_int) -> bool,
	pub GetType: extern "C" fn(iStackPos: c_int) -> c_int,
	pub GetTypeName: extern "C" fn(iType: c_int) -> *const c_char,

	#[deprecated]
	pub CreateMetaTableType: extern "C" fn(strName: *const c_char, iType: c_int),
	pub CheckString: extern "C" fn(iStackPos: c_int) -> *const c_char,
	pub CheckNumber: extern "C" fn(iStackPos: c_int) -> c_double,
	pub ObjLen: extern "C" fn(iStackPos: c_int) -> c_int,

	pub GetAngle: extern "C" fn(iStackPos: c_int) -> &'static Angle,
	pub GetVector: extern "C" fn(iStackPos: c_int) -> &'static Vector,

	pub PushAngle: extern "C" fn(ang: &Angle),
	pub PushVector: extern "C" fn(vec: &Vector),

	pub SetState: extern "C" fn(pState: *mut c_void),
	pub CreateMetaTable: extern "C" fn(strName: *const c_char) -> bool,
	pub PushMetaTable: extern "C" fn(iType: c_int) -> bool,
	pub PushUserType: extern "C" fn(data: *mut c_void, iType: c_int),
	pub SetUserType: extern "C" fn(iStackPos: c_int, data: *mut c_void)
}
