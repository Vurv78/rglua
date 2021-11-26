// Types are in camelcase + C prefix
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

pub mod types;
pub mod globals;
mod helper;
#[cfg(feature = "interfaces")]
mod interfaces;
#[cfg(feature = "interfaces")]
pub mod interface;

#[macro_use]
pub mod lua_shared;

pub mod prelude;