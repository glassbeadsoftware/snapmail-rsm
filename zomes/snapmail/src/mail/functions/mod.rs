mod get_mail;
mod send_mail;
mod get_all_mails;

mod acknowledge_mail;
mod check_ack_inbox;
mod check_mail_inbox;
mod get_all_unacknowledged_inmails;
mod has_ack_been_received;
mod has_mail_been_fully_acknowleged;
mod delete_mail;
mod request_acks;

pub use self::{
   acknowledge_mail::*,
   check_ack_inbox::*,
   check_mail_inbox::*,
   get_all_unacknowledged_inmails::*,
   get_mail::*,
   get_all_mails::*,
   has_ack_been_received::*,
   has_mail_been_fully_acknowleged::*,
   send_mail::*,
   delete_mail::*,
   request_acks::*,
};
