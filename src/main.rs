
use serde_json::json;
use serde::{Deserialize, Serialize};
use obake;

#[obake::versioned]
#[obake(version("0.1.0"))]
#[obake(version("0.2.0"))]
#[obake(version("0.3.0"))]
#[obake(derive(Debug, Clone, Serialize, Deserialize))]
#[obake(serde(untagged))]
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Character {
    pub name: String,
    pub age: u32,
    #[obake(cfg(">=0.2.0"))]
    pub height: f32,
    #[obake(cfg(">=0.3.0"))]
    pub weight: f32,
}

impl From<Character!["0.1.0"]> for Character!["0.2.0"] {
    fn from(old: Character!["0.1.0"]) -> Self {
        Self {
            name: old.name,
            age: old.age,
            ..Default::default()
        }
    }
}

impl From<Character!["0.2.0"]> for Character!["0.3.0"] {
    fn from(old: Character!["0.2.0"]) -> Self {
        Self {
            name: old.name,
            age: old.age,
            height: old.height,
            ..Default::default()
        }
    }
}

// There must be a way to generate this or a simmilar parser automatically
fn character_parser(serialized_data: &String) -> Character {
    if let Ok(data) = serde_json::from_str::<Character!["0.3.0"]>(&serialized_data) {
        return data;
    }

    if let Ok(data) = serde_json::from_str::<Character!["0.2.0"]>(&serialized_data) {
        let data: Character!["0.3.0"] = data.into();
        return data.into();
    }

    if let Ok(data) = serde_json::from_str::<Character!["0.1.0"]>(&serialized_data) {
        let data: Character!["0.2.0"] = data.into();
        let data: Character!["0.3.0"] = data.into();
        return data;
    }

    return Character::default();
}



fn main() {}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use crate::*;

    #[test]
    fn from_2_to_3() {
        let freeza_ser = serde_json::to_string_pretty(&json!({
            "name": "Freeza",
            "age": 32,
            "height": 1.53,
        })).unwrap();
        dbg!(&freeza_ser);

        let freeza: Character = character_parser(&freeza_ser);
        dbg!(&freeza);

        let freeza_expected = Character {
            name: "Freeza".into(),
            age: 32,
            height: 1.53,
            weight: 0.,
        };
        assert_eq!(&freeza_expected, &freeza);
    }

    #[test]
    fn from_1_to_2_to_3() {
        let goku_ser = serde_json::to_string_pretty(&json!({
            "name": "Goku",
            "age": 42,
        })).unwrap();
        dbg!(&goku_ser);

        let goku: Character = character_parser(&goku_ser);
        dbg!(&goku);

        let goku_expected = Character {
            name: "Goku".into(),
            age: 42,
            height: 0.,
            weight: 0.,
        };
        assert_eq!(goku_expected, goku)
    }

}
