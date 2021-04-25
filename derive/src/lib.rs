extern crate proc_macro;
use proc_macro::{TokenStream};

use quote::{quote, format_ident};

/// Proc macro that generates an easy to use api function to use directly in rust out of
/// a hdk_extern function.
/// "snapmail_*" is prepended to the function name
#[proc_macro_attribute]
pub fn snapmail_api(_metadata: TokenStream, item: TokenStream) -> TokenStream {
   // -- Parse input and retrieve function signature
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

   // Get the type within the ExternResult
   let path_type = if let syn::Type::Path(tp) = *output_type.clone() {
      tp.clone()
   } else {
      unreachable!();
   };
   let angle_type = if let syn::PathArguments::AngleBracketed(ab) = &path_type.path.segments[0].arguments{
      ab.clone()
   } else {
      unreachable!();
   };
   let type_type = if let syn::GenericArgument::Type(tt) = &angle_type.args[0] {
      tt.clone()
   } else {
      unreachable!();
   };
   let inner_path_type = if let syn::Type::Path(tp) = type_type {
      tp.clone()
   } else {
      unreachable!();
   };
   let inner_type = &inner_path_type.path.segments[0].ident;
   println!("\n\n input.inner_type: \"{:?}\"\n\n", inner_type);

   // -- Output api function
   let output_fn = format_ident!("snapmail_{}", external_fn_ident);

   // // Use snapmail! macro
   // let output: TokenStream = (quote! {
   //    #item_fn
   //    use snapmail_api::api_error::*;
   //    use snapmail_api::*;
   //    use holochain::conductor::ConductorHandle;
   //
   //    pub fn #output_fn(conductor: ConductorHandle, arg: #input_type) -> SnapmailApiResult<#inner_type> {
   //       snapmail!(conductor, #external_fn_ident, #inner_type, arg)
   //    }
   // }).into();

   // Output
   let output: TokenStream = (quote! {
      #item_fn
      pub fn #output_fn(conductor: holochain::conductor::ConductorHandle, arg: #input_type) -> snapmail_api::api_error::SnapmailApiResult<#inner_type> {
         use snapmail_api::api_error::*;
         use snapmail_api::*;
         use holochain::core::workflow::ZomeCallResult;

         let payload: ExternIO = ExternIO::encode(arg).unwrap();
         //println!("      payload = {:?}", payload);
         let fn_name = std::stringify!(#external_fn_ident);
         //println!("      fn_name = {:?}", fn_name);
         let result: SnapmailApiResult<#inner_type> = tokio_helper::block_on(async {
            let call_result = call_zome(conductor, fn_name, payload).await?;
            //println!("      call_result = {:?}", call_result);
            let api_result: SnapmailApiResult<#inner_type> = match call_result {
               ZomeCallResponse::Ok(io1) => {
                  let io: ExternIO = io1;
                  let maybe_ret: #inner_type = io.decode().unwrap();
                  Ok(maybe_ret)
               },
               ZomeCallResponse::Unauthorized(_, _, _, _) => Err(SnapmailApiError::Unauthorized),
               ZomeCallResponse::NetworkError(err) => Err(SnapmailApiError::NetworkError(err)),
            };
            api_result
         }, *DEFAULT_TIMEOUT).map_err(|_e| SnapmailApiError::Timeout).unwrap();
         //println!("     macro result = {:?}", result);
         result
      }
   }).into();

   //println!("\n\n output: \"{}\"\n\n", output.to_string());
   output
}
