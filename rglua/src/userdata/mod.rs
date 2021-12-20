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
			derive_more::Add,
			derive_more::AddAssign,
			derive_more::Sub,
			derive_more::SubAssign,
			derive_more::Mul,
			derive_more::MulAssign,
			derive_more::Div,
			derive_more::DivAssign,
			derive_more::Rem,
			derive_more::RemAssign,
			derive_more::Into,
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
		udata!( $($rest)* );
	};
	() => ()
}

udata! {
	// https://github.com/danielga/sourcesdk-minimal/blob/cab3e07edc4a41e7e69ea645ea51c1e5c5d1be71/public/mathlib/vector.h#L66
	pub struct Vector {
		pub x: f32,
		pub y: f32,
		pub z: f32
	}

	// https://github.com/danielga/sourcesdk-minimal/blob/cab3e07edc4a41e7e69ea645ea51c1e5c5d1be71/public/mathlib/vector.h#L1765
	/// QAngle
	pub struct Angle {
		pub p: f32,
		pub y: f32,
		pub r: f32
	}
}
