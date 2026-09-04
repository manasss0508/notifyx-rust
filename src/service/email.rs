use lettre::{
     message::header::ContentType, transport::smtp::authentication::Credentials, AsyncSmtpTransport,Message, Tokio1Executor, AsyncTransport};
use crate::error::AppError;

pub struct EmailService {
    mailer: AsyncSmtpTransport<Tokio1Executor>, // sends mail
    from: String, // mail is sent from , senders mail
}

impl EmailService {
    pub fn new(username: String, password: String, from: String, host: String) -> Self {
        // smtp credentials
        let credentials = Credentials::new(username,password);

        // creating AsyncSmtpTransport which is mailer and manages conn pool to smtp
        let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(host.as_str())
            .unwrap()
            .credentials(credentials)
            .build();

        EmailService{
            mailer,
            from,
        }

    }

    // sends mail
    pub async fn send(&self,to: &str, subject: &str, body: &str) -> Result<(), AppError> {
        println!("from : {:?}", &self.from);
        let mail = Message::builder()
            .from(self.from.parse().map_err(|e|{
                println!("sender mail parsing");
                AppError::SmtpMailParsing(e)
            })?)
            .to(to.parse().map_err(|e|{
                println!("recipient mail parsing");
                AppError::SmtpMailParsing(e)
            })?)
            .subject(subject)
            .header(ContentType::TEXT_PLAIN)
            .body(body.to_string())?;

        self.mailer.send(mail).await?;

        Ok(())

    }
}