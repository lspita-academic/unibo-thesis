// Traits are defined to represent the memory management operations on the
// library side.
trait ZValue, ZOwn, ZClone, ...

// Since we need to modify the internal structure of the type to hold the
// corresponding wrapped value, we cannot implement the traits using just derive
// macros.
// Instead, a standard procedural macro is used that modifies the internal
// structure of the tagget type to transform it into a wrapper around a C type
// from the Zenoh pico library.
// Using the provided arguments, the automatic implementations of the traits can
// be configured: a base name must be provided to choose automatically the
// internal types and functions, then all traits that should be implemented must
// be listed explicitly, with the possibility to override the identifiers to use
// and to disable the auto implementation of the corresponding standard Rust traits.
#[zwrap(
    base(name = "string"),
    zvalue,
    zown(impl_drop = false), // (*)
    zclone(clone_zfn = some_other_clone_function)
)]
// The struct is expanded into a newtype `ZString(_z_string_t)` that wraps the C
// type and implements all the traits defined in the `zwrap` attribute arguments.
struct ZString;

// (*) The corresponding Rust trait can be later implemented manually to add extra
// functionality on top of the Zenoh pico library, accessible through the defined
// traits functions.
impl Drop for ZString { ... }
