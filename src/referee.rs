pub mod referee {
    use serde::{ Serialize, Deserialize };

    #[derive(Serialize, Deserialize)]
    pub struct Referee {
        pub name: String,
        #[serde(default)]
        pub email: Option<String>,
        #[serde(default)]
        pub phone: Option<String>,
        #[serde(default)]
        pub isactivated: Option<bool>,
    }
    impl Referee {
        pub fn builder(name: &str, email: &str, phone: &str) -> Referee {
            if email.is_empty() {
                return Referee {
                    name: name.to_string(),
                    email: None,
                    phone: Some(phone.to_string()),
                    isactivated: None,
                };
            }
            Referee {
                name: name.to_string(),
                email: Some(email.to_string()),
                phone: Some(phone.to_string()),
                isactivated: None,
            }
        }
        pub fn activate(name: &str, email: &str, phone: &str) -> Referee {
            if email.is_empty() {
                return Referee {
                    name: name.to_string(),
                    email: None,
                    phone: Some(phone.to_string()),
                    isactivated: Some(true),
                };
            }
            Referee {
                name: name.to_string(),
                email: Some(email.to_string()),
                phone: Some(phone.to_string()),
                isactivated: Some(true),
            }
        }
        pub fn deactivate(name: &str, email: &str, phone: &str) -> Referee {
            if email.is_empty() {
                return Referee {
                    name: name.to_string(),
                    email: None,
                    phone: Some(phone.to_string()),
                    isactivated: Some(false),
                };
            }
            Referee {
                name: name.to_string(),
                email: Some(email.to_string()),
                phone: Some(phone.to_string()),
                isactivated: Some(false),
            }
        }
    }
}

#[cfg(test)]
mod referee_tests {
    use crate::referee::referee::Referee;

    #[test]
    fn given_a_referee_record_when_the_builder_is_used_then_the_names_match() {
        let referee = Referee {
            name: "Rich".to_string(),
            email: Some("me@you.com".to_string()),
            phone: Some("5332432432".to_string()),
            isactivated: None,
        };
        let refereebuilder = Referee::builder("Rich", "me@you.com", "5332432432");
        assert_eq!(referee.name, refereebuilder.name);
        assert_eq!(referee.email, refereebuilder.email);
    }

    #[test]
    fn given_a_referee_record_when_the_builder_is_used_then_the_emails_match() {
        let referee = Referee {
            name: "Rich".to_string(),
            email: None,
            phone: Some("5332432432".to_string()),
            isactivated: None,
        };
        let refereebuilder = Referee::builder("Rich", "", "5332432432");
        assert_eq!(referee.email, refereebuilder.email);
    }

    #[test]
    fn given_a_referee_record_when_the_activate_is_used_then_the_referee_is_activated() {
        let referee = Referee {
            name: "Rich".to_string(),
            email: None,
            phone: Some("5332432432".to_string()),
            isactivated: Some(true),
        };
        let refereebuilder = Referee::activate("Rich", "", "5332432432");
        assert_eq!(referee.email, refereebuilder.email);
        assert_eq!(referee.isactivated, refereebuilder.isactivated);
    }

    #[test]
    fn given_a_referee_record_when_the_deactivate_is_used_then_the_referee_is_not_activated() {
        let referee = Referee {
            name: "Rich".to_string(),
            email: None,
            phone: Some("5332432432".to_string()),
            isactivated: Some(false),
        };
        let refereebuilder = Referee::deactivate("Rich", "", "5332432432");
        assert_eq!(referee.email, refereebuilder.email);
        assert_eq!(referee.isactivated, refereebuilder.isactivated);
    }
}
