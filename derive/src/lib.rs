extern crate proc_macro;
use proc_macro::{TokenStream};
use quote::{quote, format_ident};

/// Proc macro that generates an easy to use api function to use directly in rust out of
/// a hdk_extern function.
/// "snapmail_*" is prepended to the function name
///
#[proc_macro_attribute]
pub fn snapmail_api(_metadata: TokenStream, item: TokenStream) -> TokenStream {
   let item_fn = syn::parse_macro_input!(item as syn::ItemFn);

   let external_fn_ident = item_fn.sig.ident.clone();

   let input_type = if let Some(syn::FnArg::Typed(pat_type)) = item_fn.sig.inputs.first() {
      pat_type.ty.clone()
   } else {
      unreachable!();
   };
   let output_type = if let syn::ReturnType::Type(_, ref ty) = item_fn.sig.output {
      ty.clone()
   } else {
      unreachable!();
   };

   //println!("\n\n input.external_fn_ident: \"{:?}\"\n\n", external_fn_ident);

   let output_fn = format_ident!("snapmail_{}", external_fn_ident);

   let output: TokenStream = (quote! {
      #item_fn
      pub fn #output_fn(conductor: holochain::conductor::ConductorHandle, arg: #input_type) -> snapmail_api::api_error::SnapmailApiResult<#output_type> {
         snapmail_api::snapmail!(conductor, #external_fn_ident, #output_type, arg)
      }
   }).into();

   //println!("\n\n output: \"{}\"\n\n", output.to_string());
   output
}
