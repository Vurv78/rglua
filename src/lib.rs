#[macro_use]
pub mod util;
#[cfg(feature = "interfaces")]
pub mod interface;

#[macro_use]
pub mod lua;

#[deprecated(since = "0.8.0", note = "Use rglua::lua instead")]
pub use lua as lua_shared;

#[deprecated(
	since = "0.8.0",
	note = "Use rglua::lua::* or rglua::lua::types instead"
)]
pub use lua::types;

pub mod prelude;

#[cfg(feature = "userdata")]
pub mod userdata;
