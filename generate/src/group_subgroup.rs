use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use crate::emoji::Emoji;
use crate::sanitize;
pub struct Group {
    pub name: String,
    pub subgroups: Vec<Subgroup>,
}
impl Group {
    pub fn new(name: String) -> Self {
        Self {
            name,
            subgroups: vec![],
        }
    }
}
impl ToTokens for Group {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let modname = Ident::new(&sanitize(&self.name).to_lowercase(), Span::call_site());
        let subgroups = &self.subgroups;
        (quote! {
            pub mod #modname {
		#(#subgroups)*
            }
        })
            .to_tokens(tokens);
    }
}
pub struct Subgroup {
    pub name: String,
    pub emojis: Vec<Emoji>,
}
impl Subgroup {
    pub fn new(name: String) -> Self {
        Self {
            name,
            emojis: vec![],
        }
    }
}
impl ToTokens for Subgroup {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let modname = Ident::new(&sanitize(&self.name).to_lowercase(), Span::call_site());
        (quote! {
            pub mod #modname;
        })
            .to_tokens(tokens);
    }
}
