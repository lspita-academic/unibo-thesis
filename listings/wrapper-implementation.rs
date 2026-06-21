// Traits are defined to represent the memory management operations on the
// library side.
pub trait ZValue {
    // This is the internal C type that is being wrapped.
    // Since one struct should become a newtype for only one C type, it is
    // defined as an associated type instead of a generic parameter.
    type Value;

    // ...
}

// Since we need to modify the internal structure of the type to hold the
// corresponding wrapped value, we cannot implement the traits using just derive
// macros.
#[zwrap(
    // All type names in Zenoh pico follow a similar structure, so they are
    // choosen starting from a base.
    base(name = "string"),
    // For each trait that represents some operations in the emulated
    // ownership model of Zenoh pico there is a corresponding macro argument to
    // define if and how to implement it.
    zvalue,
    // By default, for each implemented trait the corresponding standard Rust
    // one is implemented.
    // Keeping the two as separate allows to still use the C implementations
    // behid a safe API when implementing the other trait manually. (*)
    zown(impl_drop = false),
    // If the type or function identifier choosen automatically from the base
    // for an implementation it's not correct, it can be set manually.
    zclone(clone_zfn = some_other_clone_function)
)]
// The struct is expanded into a newtype `ZString(_z_string_t)` that wraps the C
// type and implements all the traits defined in the `zwrap` attribute arguments.
pub struct ZString;

// (*) We can later implement the Drop trait manually to add extra functionality
// without the need to reach for the C raw functions.
impl Drop for ZString {
    fn drop(&mut self) {
        // ...
        <Self as ZOwn>::zdrop(self);
    }
}

// The generated type is at the same level as standard code, so we can
// further expand it's functionality like any other Rust type.
impl FromStr for ZString {
    // ...
}
