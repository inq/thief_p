#[macro_use]
extern crate quote;
extern crate proc_macro;
extern crate syn;

use proc_macro::TokenStream;

#[proc_macro_derive(UiView)]
pub fn is_view(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_macro_input(&s).unwrap();
    let name = ast.ident;
    let gen = quote! {
        impl ::ui::comp::View for #name {
            fn get_view_mut(&mut self) -> &mut ::ui::comp::ViewT { &mut self.view }
            fn get_view(&self) -> &::ui::comp::ViewT { &self.view }
        }
    };
    gen.parse().unwrap()
}
