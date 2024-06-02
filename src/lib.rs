/// The trait SameType<T> is implemented on T
/// Might seem simple, but it may enable many forms of metaprogramming
pub trait SameType<T> {}

impl <T> SameType<T> for T {}

