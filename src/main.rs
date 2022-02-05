use aws_sdk_sesv2::Error;
use structopt::StructOpt;

mod mail;

#[derive(Debug, StructOpt)]
struct Opt {
    /// email address to send the message from and to.
    #[structopt(short, long)]
    email: String,

    /// The message of the email.
    #[structopt(short, long)]
    message: String,

    /// The subject of the email.
    #[structopt(short, long)]
    subject: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();

    let Opt {
        email,
        message,
        subject,
    } = Opt::from_args();

    mail::mail_support(&email, &subject, &message).await
}
