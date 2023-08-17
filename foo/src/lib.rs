// uncommnet mod baz to compile

// mod baz {
// 	pub struct Baz;

// 	impl ::baz::BazTrait for Baz {
// 		type BazType = u32;
// 	}

// }
pub struct Foo;

impl From<<baz::Baz as ::baz::BazTrait>::BazType> for Foo {
    fn from(_: <baz::Baz as ::baz::BazTrait>::BazType) -> Self {
        todo!()
    }
}
