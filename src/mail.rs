use aws_sdk_sesv2::model::{Body, Content, Destination, EmailContent, Message};
use aws_sdk_sesv2::{Client, Error};

pub async fn mail_support(mail: &str, serialnumber: &str, text: &str) -> Result<(), Error> {
    send_message(&[mail], mail, serialnumber, text).await
}

async fn send_message(
    list: &[&str],
    from: &str,
    subject: &str,
    message: &str,
) -> Result<(), Error> {
    let region_provider = "eu-central-1";

    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);

    let mut contacts: Vec<String> = Vec::new();
    for &mail in list {
        contacts.push(mail.to_string());
    }

    let dest = Destination::builder()
        .set_to_addresses(Some(contacts))
        .build();
    let subject_content = Content::builder().data(subject).charset("UTF-8").build();
    let body_content = Content::builder().data(message).charset("UTF-8").build();
    let body = Body::builder().text(body_content).build();

    let msg = Message::builder()
        .subject(subject_content)
        .body(body)
        .build();

    println!("{:?}", dest);

    let email_content = EmailContent::builder().simple(msg).build();
    client
        .send_email()
        .from_email_address(from)
        .destination(dest)
        .content(email_content)
        .send()
        .await?;

    println!("Email sent to list");

    Ok(())
}
