// Types are in camelcase + C prefix
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

pub mod globals;
mod helper;
#[cfg(feature = "interfaces")]
pub mod interface;
#[cfg(feature = "interfaces")]
mod interfaces;
pub mod types;

#[macro_use]
pub mod lua_shared;

pub mod prelude;
