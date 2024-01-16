use referee::referee::Referee;
use mailjet_rs::common::Recipient;
use mailjet_rs::v3::Message;
use mailjet_rs::{ Client, SendAPIVersion };
pub mod validators;
pub mod referee;

fn main() {
    validateemail(referee::referee::Referee {
        name: "Rich".to_string(),
        email: None,
        phone: Some("+15332432432".to_string()),
        isactivated: None,
    });

    validatephone(referee::referee::Referee {
        name: "Rich".to_string(),
        email: None,
        phone: Some("+15332432432".to_string()),
        isactivated: None,
    });
    sendmessage();
}

#[tokio::main]
async fn sendmessage() {
    let client = Client::new(
        SendAPIVersion::V3,
        "0b6259bc7daad0e5300bf7f7b5631fa3",
        "ee0e185388e324dea90d08219b7080ba"
    );
    let mut message = Message::new(
        "mailjet_sender@company.com",
        "Mailjet Rust",
        Some("Your email flight plan!".to_string()),
        Some("Dear passenger, welcome to Mailjet! May the delivery force be with you!".to_string())
    );

    message.push_recipient(Recipient::new("luc50514@gmail.com"));

    // Finally send the message using the `Client`
    let response = client.send(message).await;
    match response {
        Ok(goodresponse) => println!("{:?}", goodresponse),
        Err(message) => println!("{:?}", message),
    }
}
fn validatephone(referee: Referee) {
    match validators::phonevalidator::phonevalidator::phoneisvalid_in(referee) {
        Ok(_) => println!("{}", "valid phone number"),
        Err(message) => println!("{}", message),
    }
}

fn validateemail(referee: Referee) {
    match validators::emailvalidator::emailvalidator::emailisvalid_in(referee) {
        Ok(_) => println!("{}", "valid email"),
        Err(message) => println!("{}", message),
    }
}
