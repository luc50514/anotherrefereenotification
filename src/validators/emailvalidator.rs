pub mod emailvalidator {
    use serde::{ Serialize, Deserialize };
    #[derive(Serialize)]
    pub struct Referee {
        pub name: String,
        pub email: Option<String>,
        pub phone: Option<String>,
    }
    pub fn emailisvalid(emailaddress: &str) -> Result<bool, &str> {
        if emailaddress.is_empty() {
            return Err("invalid email address");
        }
        if !emailaddress.contains("@") {
            return Err("invalid email address missing @");
        }
        Ok(true)
    }
    pub fn emailisvalidIn(referee: Referee) -> Result<bool, &'static str> {
        match referee.email {
            None => {
                return Err("invalid email address");
            }
            Some(emailaddress) => {
                if !emailaddress.contains("@") {
                    return Err("invalid email address missing @");
                }
                return Ok(true);
            }
        }
    }
}
#[cfg(test)]
mod email_validator_tests {
    use super::*;
    use crate::validators::emailvalidator::emailvalidator::Referee;
    use serde::{ Serialize, Deserialize };

    #[test]
    fn given_a_referee_record_when_email_address_is_empty_then_emailisvalid_should_be_false() {
        assert_eq!(Err("invalid email address"), emailvalidator::emailisvalid(""));
    }

    #[test]
    fn given_a_referee_record_when_email_address_is_empty_in_Referee_then_emailisvalid_should_be_false() {
        assert_eq!(
            Err("invalid email address"),
            emailvalidator::emailisvalidIn(Referee {
                name: "Rich".to_string(),
                email: None,
                phone: Some("5332432432".to_string()),
            })
        );
    }

    #[test]
    fn given_a_referee_record_when_email_address_has_no_at_in_Referee_then_emailisvalid_should_be_false() {
        assert_eq!(
            Err("invalid email address missing @"),
            emailvalidator::emailisvalidIn(Referee {
                name: "Rich".to_string(),
                email: Some("meyou.com".to_string()),
                phone: Some("5332432432".to_string()),
            })
        );
    }
    #[test]
    fn given_a_referee_record_when_email_address_is_valid_then_emailisvalid_should_be_true() {
        assert_eq!(Ok(true), emailvalidator::emailisvalid("me@you.com"));
    }

    #[test]
    fn given_a_referee_record_when_email_address_is_missing_at_then_emailisvalid_should_be_false() {
        assert_eq!(
            Err("invalid email address missing @"),
            emailvalidator::emailisvalid("meyou.com")
        );
    }
}
