mod acknowledge_mail;
mod check_incoming_ack;
mod check_incoming_mail;
mod get_all_arrived_mail;
mod get_mail;
mod get_all_mails;
mod has_ack_been_received;
mod has_mail_been_received;
mod send_mail;
mod delete_mail;

pub use self::{
    acknowledge_mail::*,
    check_incoming_ack::*,
    check_incoming_mail::*,
    get_all_arrived_mail::*,
    get_mail::*,
    get_all_mails::*,
    has_ack_been_received::*,
    has_mail_been_received::*,
    send_mail::*,
    delete_mail::*,
};
