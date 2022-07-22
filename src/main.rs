use obake;
use serde_json::json;

#[obake::versioned]
#[obake(version("0.1.0"))]
#[obake(version("0.2.0"))]
#[obake(version("0.3.0"))]
#[obake(derive(serde::Serialize, serde::Deserialize))]
#[obake(serde(tag = "version"))]
#[derive(Debug, Default, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
struct Character {
    #[obake(cfg(">=0.1.0"))]
    name: String,
    age: u32,
    #[serde(default)]
    #[obake(cfg(">=0.2.0"))]
    height: f32,
    #[serde(default)]
    #[obake(cfg(">=0.3.0"))]
    weight: f32,
}

impl From<Character!["0.1.0"]> for Character!["0.2.0"] {
    fn from(old: Character!["0.1.0"]) -> Self {
        let serialised = serde_json::to_string(&old).unwrap();
        serde_json::from_str(&serialised).unwrap()
    }
}

impl From<Character!["0.2.0"]> for Character!["0.3.0"] {
    fn from(old: Character!["0.2.0"]) -> Self {
        let serialised = serde_json::to_string(&old).unwrap();
        serde_json::from_str(&serialised).unwrap()
    }
}

fn main() {
    let character: VersionedCharacter = Character::default().into();
    dbg!(serde_json::to_string_pretty(&character));
}

#[cfg(test)]
mod tests {
    use crate::*;
    use serde_json::json;

    #[test]
    fn from_2_to_3() {
        let freeza_ser = serde_json::to_string_pretty(&json!({
            "version": "Character_v0_2_0",
            "name": "Freeza",
            "age": 32,
            "height": 1.53,
        }))
        .unwrap();
        dbg!(&freeza_ser);

        let freeza: VersionedCharacter = serde_json::from_str(&freeza_ser).unwrap();
        let freeza: Character = freeza.into();
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
            "version": "Character_v0_1_0",
            "name": "Goku",
            "age": 42,
        }))
        .unwrap();
        dbg!(&goku_ser);

        let goku: VersionedCharacter = serde_json::from_str(&goku_ser).unwrap();
        let goku: Character = goku.into();
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
