extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(DogTraits)]
pub fn dog_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let struct_name = &ast.ident;

    let trait_impl = quote! {
        impl DogTraits for #struct_name {
            fn barks(&self) {
                println!("Woof woof");
            }

            fn fetch_ball(&self) {
                println!("{} is fetching the ball", self.name);
            }
        }
    };
    trait_impl.into()
}
