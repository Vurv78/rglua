// Types are in camelcase + C prefix
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

pub mod globals;
#[macro_use]
pub mod util;
#[cfg(feature = "interfaces")]
pub mod interface;
pub mod types;

#[macro_use]
pub mod lua_shared;

pub mod prelude;
