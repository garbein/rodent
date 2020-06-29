use lettre_email::{ Email};
use lettre::{ smtp::authentication::Credentials, ClientSecurity, ClientTlsParameters, SmtpClient, Transport };
use native_tls::TlsConnector;

#[allow(dead_code)]
pub fn send_mail (
      to_addr: String,
      subject: String,
      body: String,
      smtp_port: String,
      smtp_host: String,
      smtp_passwd: String,
      smtp_user: String
   ) {

  let mail = Email::builder()
    .to(to_addr)
    .from("blog@qttc.net")
    .subject(subject)
    .html(&body)
    .build();

  let creds = Credentials::new(
    smtp_user,
    smtp_passwd,
  );

  let addr = String::from(format!("{}:{}", smtp_host, smtp_port));
  let connector = TlsConnector::new().unwrap();
  let tls_params = ClientTlsParameters::new(smtp_host, connector);
  let security = ClientSecurity::Required(tls_params);
  let mut mailer = match SmtpClient::new(addr, security) {
    Ok(v) => v.credentials(creds).transport(),
    Err(e) => {
        println!("Connecting to SMTP server was error: {}", e);
        return;
    }
  };

  match mailer.send(mail.unwrap().into()) {
    Ok(_) => println!("Sending email success!"),
    Err(e) => println!("Sending email was error: {}", e),
  }
}

pub fn send_mail_simply(email_receiver: &str, content: &str){
    let mine_email = "10000@qq.com";
    let smtp_server = "smtp.qq.com";
    let password = "888888888888";

    let email = Email::builder()
        .to(email_receiver)
        .from(mine_email)
        .subject("subject")
        .html(content)
        .text("Message send by WFSB")
        .build()
        .unwrap();

    let creds = Credentials::new(
        mine_email.to_string(),
        password.to_string(),
    );

    // Open connection to Gmail
    let mut mailer = SmtpClient::new_simple(smtp_server)
        .unwrap()
        .credentials(creds)
        .transport();

    // Send the email
    let result = mailer.send(email.into());

    if result.is_ok() {
        println!("Email sent");
    } else {
        println!("Could not send email: {:?}", result);
    }

    print!("{:?}", result);
    mailer.close();
}