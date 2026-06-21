// Procedural macros must be defined in a separate crate from the one that uses
// them, but for the sake of this example everything is shown in a single file.

use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident};

/// This is an example trait that we want to be able to implement automatically.
trait HelloWorld {
    fn hello_world();
}

// This struct describes the parameters of the optional customization attribute
// that can sit on top of the struct being derived, which darling populates
// accordingly.
#[derive(FromDeriveInput, Default)]
#[darling(attributes(hello_world), default)]
struct HelloWorldOpts {
    // `ident` is a special field containing the identifier of the derived struct.
    ident: Ident,
    greeting: Option<String>,
}

// This function is the entry point for the derive macro, which is called
// automatically by the compiler when the `HelloWorld` trait is derived for type.
// It also defines an associated `hello_world` attribute that can be used for
// extra options.
#[proc_macro_derive(HelloWorld, attributes(hello_world))]
fn hello_world_derive(input: TokenStream) -> TokenStream {
    // Syn automatically parses the tokens stream into an
    // Abstract Syntax Tree (AST).
    let derive_input = parse_macro_input!(input as DeriveInput);

    // Darling extracts the options from the `hello_world` attribute to populate
    // the options struct.
    let opts = match HelloWorldOpts::from_derive_input(&derive_input) {
        Ok(opts) => opts,
        // If there is any error, make it appear at compile time.
        Err(err) => return TokenStream::from(err.write_errors()),
    };

    let ident = opts.ident;
    // Default greeting is "Hello" if omitted in the options attribute.
    let greeting = opts.greeting.unwrap_or("Hello".to_string());

    let message = format!("{greeting} world from {ident}!");

    // Quote creates the output tokens stream directly from code, with the ability
    // to interpolate values in it using the # syntax.
    let expanded = quote! {
        impl HelloWorld for #ident {
            fn hello_world() {
                println!(#message);
            }
        }
    };
    expanded.into()
}

// The derive attribute automatically calls the associated procedural macro to
// implement the trait automatically.
#[derive(HelloWorld)]
struct Foo;

#[derive(HelloWorld)]
// The `hello_world` attribute can be also added to override the default
// behaviour.
#[hello_world(greeting = "Good morning")]
struct Bar;

fn main() {
    Foo::hello_world(); // output: Hello world from Foo!
    Bar::hello_world(); // output: Good morning world from Bar!
}
