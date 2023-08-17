pub trait BazTrait {
    type BazType;
}

pub struct Baz;

impl BazTrait for Baz {
    type BazType = u32;
}
