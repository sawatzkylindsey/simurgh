use rand::{thread_rng, Rng};
use regex::Regex;

const GENERATED_NAME_CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const GENERATED_NAME_LENGTH: usize = 6;

#[derive(Debug, PartialEq, Eq)]
pub enum PlayerType {
    Manual(String),
    ManualWithAssistance(String),
    Automatic,
    AutomaticWithAssistance(String),
}

impl std::str::FromStr for PlayerType {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let enum_matcher = Regex::new(r"(\w+)(\-\w+)?").map_err(|e| e.to_string())?;

        if let Some(captures) = enum_matcher.captures(value) {
            let name: String = match captures.get(2) {
                Some(n) => n.as_str()[1..].to_string(),
                None => {
                    let mut rng = thread_rng();
                    (0..GENERATED_NAME_LENGTH)
                        .map(|_| {
                            let index = rng.gen_range(0..GENERATED_NAME_CHARSET.len());
                            GENERATED_NAME_CHARSET[index] as char
                        })
                        .collect()
                }
            };

            match &captures[1] {
                "Manual" => Ok(PlayerType::Manual(name)),
                "ManualWithAssistance" => Ok(PlayerType::ManualWithAssistance(name)),
                "Automatic" => Ok(PlayerType::Automatic),
                "AutomaticWithAssistance" => Ok(PlayerType::AutomaticWithAssistance(name)),
                _ => Err(format!("unknown variant {}", &captures[1])),
            }
        } else {
            Err(format!("not parseable {}", value))
        }
    }
}

impl std::fmt::Display for PlayerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlayerType::Manual(v) => write!(f, "Manual-{}", v),
            PlayerType::ManualWithAssistance(v) => write!(f, "ManualWithAssistance-{}", v),
            PlayerType::Automatic => write!(f, "Automatic"),
            PlayerType::AutomaticWithAssistance(v) => write!(f, "AutomaticWithAssistance-{}", v),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn to_string() {
        assert_eq!(
            PlayerType::Manual("bob".to_string()).to_string(),
            "Manual-bob".to_string()
        );
        assert_eq!(
            PlayerType::ManualWithAssistance("bob".to_string()).to_string(),
            "ManualWithAssistance-bob".to_string()
        );
        assert_eq!(PlayerType::Automatic.to_string(), "Automatic".to_string());
        assert_eq!(
            PlayerType::AutomaticWithAssistance("bob".to_string()).to_string(),
            "AutomaticWithAssistance-bob".to_string()
        );
    }

    #[test]
    fn from_string() {
        assert_eq!(
            PlayerType::from_str("Manual-bob").unwrap(),
            PlayerType::Manual("bob".to_string())
        );
        let name: String;

        if let PlayerType::Manual(generated) = PlayerType::from_str("Manual-").unwrap() {
            assert_eq!(generated.len(), GENERATED_NAME_LENGTH);
            name = generated.to_string();
        } else {
            panic!("assertion error");
        }

        if let PlayerType::Manual(generated) = PlayerType::from_str("Manual").unwrap() {
            assert_eq!(generated.len(), GENERATED_NAME_LENGTH);
            assert_ne!(name, generated);
        } else {
            panic!("assertion error");
        }

        assert_eq!(
            PlayerType::from_str("ManualWithAssistance-bob").unwrap(),
            PlayerType::ManualWithAssistance("bob".to_string())
        );

        assert_eq!(
            PlayerType::from_str("Automatic-bob").unwrap(),
            PlayerType::Automatic
        );
        assert_eq!(
            PlayerType::from_str("Automatic-").unwrap(),
            PlayerType::Automatic
        );
        assert_eq!(
            PlayerType::from_str("Automatic").unwrap(),
            PlayerType::Automatic
        );

        assert_eq!(
            PlayerType::from_str("AutomaticWithAssistance-bob").unwrap(),
            PlayerType::AutomaticWithAssistance("bob".to_string())
        );
    }
}
