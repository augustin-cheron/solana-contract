use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, spanned::Spanned, Data, DeriveInput, Fields};

pub fn do_derive_from_accounts(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    let span = input.span();
    let name = input.ident;

    let ty = generate_type(&input.data, span).unwrap_or_else(syn::Error::into_compile_error);
    let expanded = quote! {
        impl<'a> ::solana_contract::FromAccounts<'a> for #name<'a> {
            fn from_accounts(accounts: &[::solana_program::account_info::AccountInfo<'a>]) -> Result<Self, ::solana_program::program_error::ProgramError>
            where
                Self: Sized,
            {
                let ai = &mut accounts.iter();
                Ok(Self {
                    #ty
                })
            }
        }
    };

    expanded.into()
}

fn generate_type(data: &Data, span: Span) -> Result<TokenStream, syn::Error> {
    let ty = match data {
        Data::Struct(data) => generate_struct(&data.fields),
        Data::Enum(_) => return Err(syn::Error::new(span, "Enum are not supported")),
        Data::Union(_) => return Err(syn::Error::new(span, "unions are not supported")),
    };

    Ok(ty)
}

fn generate_struct(fields: &Fields) -> TokenStream {
    match fields {
        Fields::Named(fields) => {
            let fields = fields.named.iter().map(|f| {
                let name = f.ident.as_ref().unwrap();
                quote_spanned!(f.span() => #name: ::solana_program::account_info::next_account_info(ai)?.clone())
            });
            quote! {
                #( #fields ),*
            }
        }
        Fields::Unnamed(_) => todo!(),
        Fields::Unit => todo!(),
    }
}
