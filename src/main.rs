
use serde_json::json;
use obake;

#[obake::versioned]
#[obake(version("0.1.0"))]
#[obake(version("0.2.0"))]
#[obake(version("0.3.0"))]
#[obake(derive(serde::Serialize, serde::Deserialize))]
#[obake(serde(untagged))]
#[derive(Debug, Default, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
struct Character {
    name: String,
    age: u32,
    #[obake(cfg(">=0.2.0"))]
    height: f32,
    #[obake(cfg("0.3.0"))]
    weight: f32,
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

fn main() {
    let freeza_ser = serde_json::to_string_pretty(&json!({
        "name": "Freeza",
        "age": 32,
        "height": 1.53,
    })).unwrap();
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

    let goku_ser = serde_json::to_string_pretty(&json!({
        "name": "Goku",
        "age": 42,
    })).unwrap();
    dbg!(&goku_ser);

    let goku: VersionedCharacter = serde_json::from_str(&goku_ser).unwrap();
    let goku: Character = goku.into();

    let goku_expected = Character {
        name: "Goku".into(),
        age: 42,
        height: 0.,
        weight: 0.,
    };
    assert_eq!(goku_expected, goku)

}
