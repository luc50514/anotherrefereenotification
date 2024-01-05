pub mod validators;

fn main() {
    match
        validators::emailvalidator::emailvalidator::emailisvalid_in(
            validators::emailvalidator::emailvalidator::Referee {
                name: "Rich".to_string(),
                email: None,
                phone: Some("5332432432".to_string()),
            }
        )
    {
        Ok(_) => println!("{}", "valid email"),
        Err(message) => println!("{}", message),
    }
}
