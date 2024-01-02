pub mod validators;

fn main() {
    match validators::emailvalidator::emailvalidator::emailisvalid("me@you.com") {
        Ok(_) => println!("{}", "valid email"),
        Err(message) => println!("{}", message),
    }
}
