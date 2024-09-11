mod from_accounts;

#[proc_macro_derive(FromAccounts)]
pub fn derive_from_accounts(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    from_accounts::do_derive_from_accounts(item)
}
