use proc_macro::TokenStream;
use quote::{quote, ToTokens};

use syn::{parse_macro_input, parse_quote, FnArg, ItemFn, ReturnType, Type};

fn handle_gmod(item: TokenStream, export: Option<&str>) -> TokenStream {
	let mut returns_result: Option<&Box<Type>> = None;

	let mut ast = parse_macro_input!(item as ItemFn);

	assert!(ast.sig.asyncness.is_none(), "Cannot be asynchronous");
	assert!(ast.sig.constness.is_none(), "Cannot be const");
	assert!(
		ast.sig.inputs.len() == 1,
		"Must have one parameter, being the Lua state (rglua::lua::LuaState)"
	);

	if let ReturnType::Type(_, ty) = &ast.sig.output {
		let mut ret = ty.to_token_stream().to_string();
		if ret.starts_with("Result < i32") | ret.starts_with("std :: result :: Result < i32") {
			ret.retain(|c| !c.is_whitespace());
			returns_result = Some(ty);
		} else {
			assert!(
				ret.as_str() == "i32",
				"Exported function must return i32 or Result<i32, E>"
			);
		}
	} else {
		panic!("Exported function must return i32 or Result<i32, E>");
	}

	let lua_shared_param;
	let lua_shared_ty;
	// Make sure parameter is a LuaState
	match ast.sig.inputs.first().unwrap() {
		FnArg::Receiver(_) => panic!("Parameter cannot be self"),
		FnArg::Typed(arg) => {
			// In the future this could check if it is *c_void as well.
			match arg.ty.to_token_stream().to_string().as_str() {
				"LuaState" | "rglua :: lua :: LuaState" => (),
				a => panic!("Parameter must be rglua::lua::LuaState. Got {}", a),
			}

			lua_shared_ty = &arg.ty;

			match *arg.pat {
				syn::Pat::Ident(ref i) => {
					lua_shared_param = &i.ident;
				}
				syn::Pat::Wild(_) => {
					panic!("Parameter must be named. Try _foo");
				}
				_ => panic!("Parameter must be in ``ident: ty`` format"),
			}
		}
	}

	// Make sure abi is either omitted, "C", or "C-unwind"
	if let Some(abi) = &ast.sig.abi {
		match abi.name.as_ref().unwrap().value().as_str() {
			"C" | "C-unwind" => (),
			_ => panic!("Only C (or C-unwind) ABI is supported"),
		}
	} else {
		ast.sig.abi = Some(parse_quote!(extern "C"))
	}

	if let Some(ret) = returns_result {
		// We don't need to change the name of the innter function, because the outer function will compile to
		// gmod13_(open|close)
		let inner_fn = &ast.sig.ident;
		let inner_stmts = &ast.block.stmts;
		let attrs = &ast.attrs;

		let inner = quote! {
			#(#attrs)*
			fn #inner_fn(#lua_shared_param: #lua_shared_ty) -> #ret {
				#(#inner_stmts)*
			}
		};

		let resultant = quote! {
			match #inner_fn(#lua_shared_param) {
				Err(why) => {
					// Your error must implement display / .to_string().
					// I'd recommend ``thiserror``.
					let err = why.to_string();
					let err = cstr!(err);
					rglua::lua::luaL_error(#lua_shared_param, cstr!("%s"), err.as_ptr());
				},
				Ok(n) => { return n }
			}
		};

		ast.block
			.stmts
			.insert(0, syn::parse2(inner).expect("Error parsing inner fn"));
		ast.block
			.stmts
			.insert(1, syn::parse2(resultant).expect("Error parsing resultant"));
		ast.block.stmts.truncate(2);

		ast.sig.output = ReturnType::Type(Default::default(), Box::new(parse_quote!(i32)));

		// Prevent attributes from going onto the generated extern "C" function
		// They will be applied to the inner function.
		ast.attrs.clear();
	}

	if let Some(export) = export {
		ast.sig.ident = quote::format_ident!("{}", export);
	}

	for attr in &ast.attrs {
		if let Some(id) = attr.path.get_ident() {
			if id == "no_mangle" {
				panic!("Using no_mangle is unnecessary on exported functions");
			}
		}
	}

	// Attributes will go onto the inner function
	ast.attrs.push(parse_quote!(#[no_mangle]));

	ast.into_token_stream().into()
}

#[proc_macro_attribute]
/// Creates the entrypoint to garrysmod. Compiles down to gmod13_open.
///
/// Normally you would not be able to return types other than i32 through to gmod13_open,
/// this is still true, but this proc-macro allows it through unwrapping the result and containing attributes on a hidden generated function.
/// # Examples
/// ```rust
/// use rglua::prelude::*;
/// #[gmod_open]
/// fn entry(state: LuaState) -> Result<i32, std::io::Error> {
///     printgm!(state, "Hello, gmod!");
///     std::fs::write("foo.txt", "bar")?;
///
///     // We don't push objects to lua, so return 0 (# of returns)
///     Ok(0)
/// }
/// ```
pub fn gmod_open(_attr: TokenStream, item: TokenStream) -> TokenStream {
	handle_gmod(item, Some("gmod13_open"))
}

#[proc_macro_attribute]
/// Creates the exitpoint to garrysmod. Compiles down to gmod13_close.
///
/// Normally you would not be able to return types other than i32 through to gmod13_open,
/// this is still true, but this proc-macro allows it through unwrapping the result and containing attributes on a hidden generated function.
/// # Examples
/// ```rust
/// use rglua::prelude::*;
/// #[gmod_close]
/// fn exit(state: LuaState) -> Result<i32, std::io::Error> {
///     printgm!(state, "Goodbye, gmod!");
///     // Do your cleanup stuff here.
///     // We don't push objects to lua, so return 0 (# of returns)
///     Ok(0)
/// }
/// ```
pub fn gmod_close(_attr: TokenStream, item: TokenStream) -> TokenStream {
	handle_gmod(item, Some("gmod13_close"))
}

#[proc_macro_attribute]
/// Creates a valid function to be passed down to lua.
/// Note this function will not be registered automatically for you, you must use luaL_register or functions like lua_pushcfunction.
/// This may change in the future or allow for something like #[lua_function(name = "foo", auto = true)]
/// # Examples
/// ```rust
/// use rglua::prelude::*;
/// #[lua_function]
/// fn write(state: LuaState) -> Result<i32, std::io::Error> {
///     printgm!(state, "Hello, lua!");
///     std::fs::write("foo.txt", "bar")?;
///     Ok(0)
/// }
pub fn lua_function(_attr: TokenStream, item: TokenStream) -> TokenStream {
	handle_gmod(item, None)
}
