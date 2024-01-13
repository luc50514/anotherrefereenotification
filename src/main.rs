pub mod validators;
pub mod referee;

fn main() {
    match
        validators::emailvalidator::emailvalidator::emailisvalid_in(referee::referee::Referee {
            name: "Rich".to_string(),
            email: None,
            phone: Some("5332432432".to_string()),
            isactivated: None,
        })
    {
        Ok(_) => println!("{}", "valid email"),
        Err(message) => println!("{}", message),
    }
    match
        validators::phonevalidator::phonevalidator::phoneisvalid_in(referee::referee::Referee {
            name: "Rich".to_string(),
            email: None,
            phone: Some("5332432432".to_string()),
            isactivated: None,
        })
    {
        Ok(_) => println!("{}", "valid phone number"),
        Err(message) => println!("{}", message),
    }
}
