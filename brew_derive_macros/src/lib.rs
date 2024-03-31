use proc_macro::TokenStream;
use quote::quote;

/////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[proc_macro_derive(FromYml)]
pub fn from_yml_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_from_yml_macro(&ast)
}

fn impl_from_yml_macro(ast: &syn::DeriveInput) -> TokenStream {
    let impl_name = &ast.ident;
    let gen = quote! {
        impl FromYml<#impl_name> for #impl_name {
            fn from_yml(file_name: &str) -> Result<#impl_name, BrewError> {
                let mut data: #impl_name = from_str(&Self::read_yml(file_name))?;
                data.set_file_name(&file_name.to_string());
                Ok(data)
            }
        }
    };
    gen.into()
}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[proc_macro_derive(Raw)]
pub fn raw_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_raw_macro(&ast)
}

fn impl_raw_macro(ast: &syn::DeriveInput) -> TokenStream {
    let impl_name = &ast.ident;
    let gen = quote! {
        impl Raw for #impl_name {
            fn set_file_name(&mut self, file_name: &String) {
                self.file_name = Some(file_name.to_owned());
                self.set_inner_raws_file_name(file_name);
            }

            fn get_file_name(&self) -> String {
                match &self.file_name {
                    Some(file_name) => file_name.to_owned(),
                    None => String::from(""),
                }
            }
        }
    };
    gen.into()
}
