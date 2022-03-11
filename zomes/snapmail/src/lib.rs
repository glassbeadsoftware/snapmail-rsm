#![allow(non_upper_case_globals)]
#![allow(unused_doc_comments)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_attributes)]

extern crate strum;
#[macro_use]
extern crate strum_macros;
#[macro_use] extern crate shrinkwraprs;


#[macro_use]
extern crate snapmail_proc_macro;

#[cfg(not(target_arch = "wasm32"))]
pub mod api_error;

//mod utils;
mod constants;
mod link_kind;
mod entry_kind;
mod path_kind;

mod dm;
mod dm_protocol;

pub mod signal_protocol;

mod callbacks;

pub mod handle;
pub mod mail;
pub mod file;

pub mod pub_enc_key;

pub use dm::*;
pub use dm_protocol::*;
//pub use utils::*;
pub use constants::*;
pub use link_kind::*;
pub use entry_kind::*;
pub use path_kind::*;
pub use signal_protocol::*;
