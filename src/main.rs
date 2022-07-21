use aversion::{self, FromVersion};

#[derive(Debug, Default, PartialEq, serde::Deserialize, aversion::Versioned)]
struct CharacterV1 {
    pub name: String,
    pub age: u32,
}

#[derive(Debug, Default, PartialEq, serde::Deserialize, aversion::Versioned)]
struct CharacterV2 {
    pub name: String,
    pub age: u32,
    pub height: f32,
}


impl FromVersion<CharacterV1> for CharacterV2 {
    fn from_version(old: CharacterV1) -> Self {
        Self {
            name: old.name,
            age: old.age,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, aversion::Versioned, aversion::UpgradeLatest)]
struct CharacterV3 {
    pub name: String,
    pub age: u32,
    pub height: f32,
    pub weight: f32,
}

impl FromVersion<CharacterV2> for CharacterV3 {
    fn from_version(old: CharacterV2) -> Self {
        Self {
            name: old.name,
            age: old.age,
            height: old.height,
            ..Default::default()
        }
    }
}

type Character = CharacterV3;

fn main() {}

// There must be a way to generate this or a simmilar parser automatically
fn character_parser(serialized_data: &String) -> Character {
    if let Ok(data) = serde_json::from_str::<CharacterV3>(&serialized_data) {
        return Character::from_version(data);
    }

    if let Ok(data) = serde_json::from_str::<CharacterV2>(&serialized_data) {
        return Character::from_version(data);
    }

    if let Ok(data) = serde_json::from_str::<CharacterV1>(&serialized_data) {
        return Character::from_version(data);
    }

    return Character::default();
}

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
