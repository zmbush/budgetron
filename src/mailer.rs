use budget::Budget;
use config::Email;
use handlebars::{Context, Handlebars, Helper, HelperDef, RenderContext, RenderError};
use lettre::email::EmailBuilder;
use lettre::transport::smtp;
use lettre::transport::EmailTransport;
use rustc_serialize;

pub fn email_budget(cfg: &Email, budget: &Budget) {
    let mut handlebars = Handlebars::new();
    handlebars.register_template_string("email", include_str!("email.html").to_owned())
        .expect("Couldn't register email");

    handlebars.register_helper("money", Box::new(MoneyHelper));

    println!("Constructing mail");
    let mut builder = EmailBuilder::new();
    builder = builder.from(&cfg.from)
        .header(("Content-Type", "text/html"))
        .subject(&format!("Budget report for date ending on {}", budget.end_date))
        .body(&handlebars.render("email", budget).unwrap());
    for to in &cfg.to {
        builder = builder.to(to);
    }
    let mail = builder.build().unwrap();

    let builder = smtp::SmtpTransportBuilder::new((cfg.account.server.as_ref(), cfg.account.port))
        .unwrap();
    let mut mailer =
        builder.credentials(cfg.account.username.as_ref(), cfg.account.password.as_ref())
            .security_level(smtp::SecurityLevel::AlwaysEncrypt)
            .smtp_utf8(true)
            .build();

    let _ = mailer.send(mail);
}

struct MoneyHelper;

impl HelperDef for MoneyHelper {
    fn call(&self,
            c: &Context,
            h: &Helper,
            _: &Handlebars,
            rc: &mut RenderContext)
            -> Result<(), RenderError> {
        let num = h.param(0).expect("BOD").value().to_string();
        let value = c.navigate(rc.get_path(), &num);

        if let rustc_serialize::json::Json::F64(float) = *value {
            try!(rc.writer
                .write(format!("${:.2}", float).into_bytes().as_ref()));
        } else {
            try!(rc.writer
                .write("NOT A NUMBER".to_owned().into_bytes().as_ref()));
        }

        Ok(())
    }
}
