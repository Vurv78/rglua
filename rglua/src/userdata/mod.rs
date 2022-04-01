macro_rules! udata {
	(
		$(#[$outer:meta])*
		$vis:vis struct $name:ident {
			$(
				$fieldvis:vis $field:ident: $ty:ty
			),*
		}
		$($rest:tt)*
	) => {
		#[repr(C)]
		#[derive(
			PartialEq,
			PartialOrd,
			Debug,
			Default,
			Copy,
			Clone
		)]
		$(#[$outer])*
		$vis struct $name {
			$(
				$fieldvis $field: $ty
			),*
		}

		impl $name {
			pub fn new( $($field: $ty),* ) -> $name {
				$name {
					$($field),*
				}
			}
		}
		udata!( $($rest)* );
	};
	() => ();
}

udata! {
	// https://github.com/danielga/sourcesdk-minimal/blob/cab3e07edc4a41e7e69ea645ea51c1e5c5d1be71/public/mathlib/vector.h#L66
	/// Floating point vector type created by the Vector() function in lua and Vector::new() in Rust.
	pub struct Vector {
		pub x: f32,
		pub y: f32,
		pub z: f32
	}

	// https://github.com/danielga/sourcesdk-minimal/blob/cab3e07edc4a41e7e69ea645ea51c1e5c5d1be71/public/mathlib/vector.h#L1765
	/// Euler angle type.
	/// This is a QAngle in the source engine.
	pub struct Angle {
		pub p: f32,
		pub y: f32,
		pub r: f32
	}
}
