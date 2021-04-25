extern crate proc_macro;
use proc_macro::{TokenStream};

#[cfg(not(target_arch = "wasm32"))]
use quote::{quote, format_ident};

#[cfg(target_arch = "wasm32")]
pub fn snapmail_api(_metadata: TokenStream, item: TokenStream) -> TokenStream {
   item
}

/// Proc macro that generates an easy to use api function to use directly in rust out of
/// a hdk_extern function.
/// "snapmail_*" is prepended to the function name
///
#[cfg(not(target_arch = "wasm32"))]
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
   //println!("\n\n input.output_type: \"{:?}\"\n\n", output_type);

   // -- Output api function
   let output_fn = format_ident!("snapmail_{}", external_fn_ident);
   // let output: TokenStream = (quote! {
   //    #item_fn
   //    pub fn #output_fn(conductor: holochain::conductor::ConductorHandle, arg: #input_type) -> snapmail_api::api_error::SnapmailApiResult<#output_type> {
   //       let res = snapmail_api::snapmail!(conductor, #external_fn_ident, #output_type, arg);
   //       println!("proc macro result = {:?}", res);
   //       res
   //    }
   // }).into();

   let output: TokenStream = (quote! {
      #item_fn
      pub fn #output_fn(conductor: holochain::conductor::ConductorHandle, arg: #input_type) -> snapmail_api::api_error::SnapmailApiResult<#output_type> {
         use snapmail_api::api_error::*;
         use snapmail_api::*;
         use holochain::core::workflow::ZomeCallResult;

         let payload: ExternIO = ExternIO::encode(arg).unwrap();
         let result: SnapmailApiResult<#output_type> = tokio_helper::block_on(async {
            let call_result: ZomeCallResult = call_zome(conductor, std::stringify!(#external_fn_ident), payload).await;
            println!("      call_result = {:?}", call_result);
            let api_result: SnapmailApiResult<#output_type> = match call_result.unwrap() {
               ZomeCallResponse::Ok(io) => {
                  println!("         macro io = {:?}", io);
                  let maybe_ret = io.decode();
                  println!("  macro maybe_ret = {:?}", maybe_ret);
                                 //Ok(io)
                  Ok(maybe_ret.unwrap())
               },
               ZomeCallResponse::Unauthorized(_, _, _, _) => Err(SnapmailApiError::Unauthorized),
               ZomeCallResponse::NetworkError(err) => Err(SnapmailApiError::NetworkError(err)),
            };
            println!("macro api_result = {:?}", api_result);
            api_result
         }, *DEFAULT_TIMEOUT).map_err(|_e| SnapmailApiError::Timeout).unwrap();
         println!("     macro result = {:?}", result);
         result
      }
   }).into();

   //println!("\n\n output: \"{}\"\n\n", output.to_string());
   output
}
