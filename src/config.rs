use lettre::email::ToMailbox;
use email::Mailbox;

#[derive(RustcDecodable)]
pub struct EmailAccount {
    pub username: String,
    pub password: String,
    pub server: String,
    pub port: u16,
}

#[derive(RustcDecodable)]
pub struct EmailDestination {
    pub name: String,
    pub email: String,
}

impl ToMailbox for EmailDestination {
    fn to_mailbox(&self) -> Mailbox {
        (&self).to_mailbox()
    }
}

impl<'a> ToMailbox for &'a EmailDestination {
    fn to_mailbox(&self) -> Mailbox {
        Mailbox::new_with_name(self.name.to_owned(), self.email.to_owned())
    }
}

#[derive(RustcDecodable)]
pub struct Email {
    pub to: Vec<EmailDestination>,
    pub from: EmailDestination,
    pub account: EmailAccount,
}

#[derive(RustcDecodable)]
pub struct Config {
    pub email: Option<Email>,
}
