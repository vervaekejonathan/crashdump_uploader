extern crate notify;

use aws_sdk_sesv2::Error;
use structopt::StructOpt;

mod file;
mod mail;

#[derive(Debug, StructOpt)]
struct Opt {
    /// email address to send the message from and to.
    #[structopt(short, long)]
    email: String,

    /// The subject of the email.
    #[structopt(short, long)]
    subject: String,

    /// The folder to watch
    #[structopt(short, long)]
    path: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();

    let Opt {
        email,
        subject,
        path,
    } = Opt::from_args();

    file::handle_all_files_in_folder(&email, &subject, &path).await;
    file::watch_all_files_in_folder(&email, &subject, &path).await
}
