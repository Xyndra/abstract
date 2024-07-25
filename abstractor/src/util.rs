use std::any::type_name;

pub(crate) fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}