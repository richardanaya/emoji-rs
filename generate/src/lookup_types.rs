use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use crate::sanitize;
pub struct GlyphLookupEntry<'a> {
    pub glyph: &'a str,
    pub group: &'a str,
    pub subgroup: &'a str,
    pub name: &'a str,
}
impl<'a> GlyphLookupEntry<'a> {
    pub fn new(glyph: &'a str, group: &'a str, subgroup: &'a str, name: &'a str) -> Self {
	Self{glyph, group, subgroup, name}
    }
}
impl<'a> ToTokens for GlyphLookupEntry<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
	let glyph = &self.glyph;
	let group = Ident::new(&sanitize(&self.group.to_string()).to_lowercase(),
			       Span::call_site());
	let subgroup = Ident::new(&sanitize(&self.subgroup.to_string()).to_lowercase(),
				  Span::call_site());
	let name = Ident::new(&sanitize(&self.name.to_string()).to_uppercase(),
			      Span::call_site());
        (quote! {
	    #glyph => crate::#group::#subgroup::#name
        }).to_tokens(tokens);
    }
}
pub struct NameLookupEntry<'a> {
    pub group: &'a str,
    pub subgroup: &'a str,
    pub name: &'a str,
}
impl<'a> NameLookupEntry<'a> {
    pub fn new(group: &'a str, subgroup: &'a str, name: &'a str) -> Self {
	Self{group, subgroup, name}
    }
}
impl<'a> ToTokens for NameLookupEntry<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
	let group = Ident::new(&sanitize(&self.group.to_string()).to_lowercase(),
			       Span::call_site());
	let subgroup = Ident::new(&sanitize(&self.subgroup.to_string()).to_lowercase(),
				  Span::call_site());
	let name = Ident::new(&sanitize(&self.name.to_string()).to_uppercase(),
			      Span::call_site());
	let namestr = &self.name;
        (quote! {
	    #namestr => crate::#group::#subgroup::#name
        }).to_tokens(tokens);
    }
}
