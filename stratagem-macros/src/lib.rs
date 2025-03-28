extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Commander)]
pub fn command_executor_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    impl_command_executor(&ast)
}

// Function to generate the implementation
fn impl_command_executor(ast: &DeriveInput) -> TokenStream {
    // Get the name of the struct
    let name = &ast.ident;
    let generics = &ast.generics;

    let gen = quote! {
        impl #generics Commander for #name #generics {
            fn execute(&mut self, mut cmd: impl Command<Self> + 'static) {
                cmd.execute(self);
            }
        }
    };
    gen.into()
}
