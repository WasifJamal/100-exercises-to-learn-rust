// TODO: Implement the `From` trait for the `WrappingU32` type to make `example` compile.

#[allow(unused)]
pub struct WrappingU32 {
    value: u32,
}

impl From<i64> for WrappingU32 {
    fn from(value: i64) -> Self {
        Self { value: value as u32 }
    }
}

#[allow(unused)]
fn example() {
    let wrapping: WrappingU32 = 42.into();
    let wrapping = WrappingU32::from(42);
}
