/// This is an example trait that we want to be able to implement automatically.
trait HelloWorld {
    fn hello_world();
}

// This struct describes the parameters of the optional customization attribute
// that can sit on top of the struct being derived, which darling populates
// accordingly.
#[derive(darling::FromDeriveInput, Default)]
struct HelloWorldOpts {
    ident: Ident, // special field containing the identifier of the derived struct
    greeting: Option<String>,
}

// This function is the entry point for the derive macro, which is called
// automatically by the compiler when the `HelloWorld` trait is derived for type.
// It also defines an associated `hello_world` attribute that can be used for
// extra options.
#[proc_macro_derive(HelloWorld, attributes(hello_world))]
fn hello_world_derive(input: TokenStream) -> TokenStream {
    // Here syn and quote can be used to parse the input tokens stream into an
    // AST and to generate the output tokens stream directly from code.
    let derive_input = syn::parse_macro_input!(input as syn::DeriveInput);

    let opts = HelloWorldOpts::from_derive_input(&derive_input);
    let ident = opts.ident;
    let greeting = opts.greeting.unwrap_or("Hello".to_string());
    let message = format!("{greeting} world from {ident}!");

    quote! {
        impl HelloWorld for #ident {
            fn hello_world() {
                println!(#message);
            }
        }
    }
}

// The derive attribute calls the associated procedural macro to implement the
// trait automatically.
#[derive(HelloWorld)]
// The `hello_world` attribute can be also added to override the default
// behaviour.
#[hello_world(greeting = "Good morning")]
struct Foo;

Foo::hello_world(); // output: Good morning world from Foo!
