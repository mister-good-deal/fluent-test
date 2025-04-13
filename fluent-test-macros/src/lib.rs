use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

/// Registers a function to be run before each test in the current module
///
/// Example:
/// ```
/// use fluent_test::prelude::*;
///
/// #[setup]
/// fn setup() {
///     // Initialize test environment
/// }
/// ```
#[proc_macro_attribute]
pub fn setup(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = &input_fn.sig.ident;

    // Create a unique registration function name based on the function name
    let register_fn_name = syn::Ident::new(&format!("__register_setup_fixture_{}", fn_name), fn_name.span());

    let output = quote! {
        #input_fn

        // We use ctor to register the function at runtime
        #[ctor::ctor]
        fn #register_fn_name() {
            fluent_test::backend::fixtures::register_setup(
                module_path!(),
                Box::new(|| #fn_name())
            );
        }
    };

    TokenStream::from(output)
}

/// Registers a function to be run after each test in the current module
///
/// Example:
/// ```
/// use fluent_test::prelude::*;
///
/// #[tear_down]
/// fn tear_down() {
///     // Clean up test environment
/// }
/// ```
#[proc_macro_attribute]
pub fn tear_down(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = &input_fn.sig.ident;

    // Create a unique registration function name based on the function name
    let register_fn_name = syn::Ident::new(&format!("__register_teardown_fixture_{}", fn_name), fn_name.span());

    let output = quote! {
        #input_fn

        // We use ctor to register the function at runtime
        #[ctor::ctor]
        fn #register_fn_name() {
            fluent_test::backend::fixtures::register_teardown(
                module_path!(),
                Box::new(|| #fn_name())
            );
        }
    };

    TokenStream::from(output)
}

/// Runs a function with setup and teardown fixtures from the current module
///
/// Example:
/// ```
/// use fluent_test::prelude::*;
///
/// #[with_fixtures]
/// fn test_something() {
///     // Test code here
///     expect!(2 + 2).to_equal(4);
/// }
/// ```
#[proc_macro_attribute]
pub fn with_fixtures(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = &input_fn.sig.ident;
    let fn_body = &input_fn.block;
    let vis = &input_fn.vis; // Preserve visibility
    let attrs = &input_fn.attrs; // Preserve attributes
    let sig = &input_fn.sig; // Get function signature

    // Generate a unique internal name for the real implementation
    let impl_name = syn::Ident::new(&format!("__{}_impl", fn_name), fn_name.span());

    let output = quote! {
        // Define the implementation function with a private name
        fn #impl_name() #fn_body

        // Create the public function with fixtures
        #(#attrs)*
        #vis #sig {
            // Get the current module path - critical for finding the right fixtures
            let module_path = module_path!();

            fluent_test::backend::fixtures::run_test_with_fixtures(
                module_path,
                std::panic::AssertUnwindSafe(|| #impl_name())
            );
        }
    };

    TokenStream::from(output)
}
