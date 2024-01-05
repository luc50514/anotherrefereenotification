pub mod emailvalidator {
    use serde::{ Serialize, Deserialize };
    use email_address::*;
    #[derive(Serialize, Deserialize)]
    pub struct Referee {
        pub name: String,
        #[serde(default)]
        pub email: Option<String>,
        #[serde(default)]
        pub phone: Option<String>,
    }

    pub fn emailisvalid_in(referee: Referee) -> Result<bool, &'static str> {
        match referee.email {
            None => {
                return Err("invalid email address");
            }
            Some(emailaddress) => {
                if EmailAddress::is_valid(&emailaddress) {
                    return Ok(true);
                }

                Err("invalid email address")
            }
        }
    }
}
#[cfg(test)]
mod email_validator_tests {
    use super::*;
    use crate::validators::emailvalidator::emailvalidator::Referee;

    #[test]
    fn given_a_referee_record_when_email_address_is_empty_in_referee_then_emailisvalid_should_be_false() {
        assert_eq!(
            Err("invalid email address"),
            emailvalidator::emailisvalid_in(Referee {
                name: "Rich".to_string(),
                email: None,
                phone: Some("5332432432".to_string()),
            })
        );
    }

    #[test]
    fn given_a_referee_record_when_email_address_has_no_at_in_referee_then_emailisvalid_should_be_false() {
        assert_eq!(
            Err("invalid email address"),
            emailvalidator::emailisvalid_in(Referee {
                name: "Rich".to_string(),
                email: Some("meyou.com".to_string()),
                phone: Some("5332432432".to_string()),
            })
        );
    }

    #[test]
    fn given_a_referee_record_when_email_address_has_no_at_in_referee_is_serialized_then_emailisvalid_should_be_false() {
        let referee: Referee = Referee {
            name: "Rich".to_string(),
            email: Some("meyou.com".to_string()),
            phone: Some("5332432432".to_string()),
        };
        let referee_serialized = serde_json::to_string(&referee).unwrap();
        let referee_deserialized: Referee = serde_json
            ::from_str(&referee_serialized.to_string())
            .unwrap();
        assert_eq!(
            Err("invalid email address"),
            emailvalidator::emailisvalid_in(referee_deserialized)
        );
    }

    #[test]
    fn given_a_referee_record_when_email_address_is_valid_in_referee_is_serialized_then_emailisvalid_should_be_false() {
        let referee: Referee = Referee {
            name: "Rich".to_string(),
            email: Some("me@you.com".to_string()),
            phone: Some("5332432432".to_string()),
        };
        let referee_serialized = serde_json::to_string(&referee).unwrap();
        let referee_deserialized: Referee = serde_json
            ::from_str(&referee_serialized.to_string())
            .unwrap();
        assert_eq!(Ok(true), emailvalidator::emailisvalid_in(referee_deserialized));
    }
}
