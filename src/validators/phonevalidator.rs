pub mod phonevalidator {
    use std::str::FromStr;

    use crate::referee::referee::*;
    extern crate phonenumber;
    use phonenumber::PhoneNumber;

    pub fn phoneisvalid_in(referee: Referee) -> Result<bool, &'static str> {
        match referee.phone {
            None => {
                return Err("invalid phone number");
            }
            Some(phone) => {
                match PhoneNumber::from_str(&phone) {
                    Ok(phonenum) => {
                        if phonenum.is_valid() {
                            return Ok(true);
                        } else {
                            return Err("invalid phone number");
                        }
                    }
                    Err(_) => {
                        return Err("invalid phone number");
                    }
                }
            }
        }
    }
}
#[cfg(test)]
mod phone_validator_tests {
    use super::*;
    use crate::referee::referee::Referee;

    #[test]
    fn given_a_referee_record_when_phone_is_empty_in_referee_then_phoneisvalid_should_be_false() {
        assert_eq!(
            Err("invalid phone number"),
            phonevalidator::phoneisvalid_in(Referee {
                name: "Rich".to_string(),
                email: None,
                phone: None,
                isactivated: None,
            })
        );
    }

    #[test]
    fn given_a_referee_record_when_phone_number_is_not_a_number_then_phoneisvalid_should_be_false() {
        assert_eq!(
            Err("invalid phone number"),
            phonevalidator::phoneisvalid_in(Referee {
                name: "Rich".to_string(),
                email: Some("meyou.com".to_string()),
                phone: Some("notvalid".to_string()),
                isactivated: None,
            })
        );
    }

    #[test]
    fn given_a_referee_record_when_phone_number_is_too_short_referee_is_serialized_then_phoneisvalid_should_be_false() {
        let referee: Referee = Referee {
            name: "Rich".to_string(),
            email: Some("meyou.com".to_string()),
            phone: Some("+1533243243".to_string()),
            isactivated: None,
        };
        let referee_serialized = serde_json::to_string(&referee).unwrap();
        let referee_deserialized: Referee = serde_json
            ::from_str(&referee_serialized.to_string())
            .unwrap();
        assert_eq!(
            Err("invalid phone number"),
            phonevalidator::phoneisvalid_in(referee_deserialized)
        );
    }

    #[test]
    fn given_a_referee_record_when_phone_number_is_valid_in_referee_is_serialized_then_phoneisvalid_should_be_true() {
        let referee: Referee = Referee {
            name: "Rich".to_string(),
            email: Some("me@you.com".to_string()),
            phone: Some("+15029025292".to_string()),
            isactivated: None,
        };
        let referee_serialized = serde_json::to_string(&referee).unwrap();
        let referee_deserialized: Referee = serde_json
            ::from_str(&referee_serialized.to_string())
            .unwrap();
        assert_eq!(Ok(true), phonevalidator::phoneisvalid_in(referee_deserialized));
    }
}
