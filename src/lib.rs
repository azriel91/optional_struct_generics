#[macro_use]
extern crate optional_struct;

#[derive(Debug, OptionalStruct)]
pub struct One<T> {
    data: T,
}

trait Trait {}

#[derive(Debug, OptionalStruct)]
pub struct Two<T: Trait> {
    data: T,
}
