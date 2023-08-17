// mod baz {
// 	bar::define_parameters! {
// 		pub Parameters = {
// 			Test: u32 = 1,
// 		}
// 	}
// }

// this failed to compile if `baz::Parameters` is coming from a different crate
// if uncommon the above code, it will compile
bar::define_aggregrated_parameters! {
	pub Parameters = {
		Test: baz::Parameters = 1,
	}
}