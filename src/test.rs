use crate::{tokenize, Token};
use futures::{unsync::mpsc, Future, Stream};

#[test]
fn simple() {
    let json_str = r#" [1, "hello", 3] "#;
    let expected = vec![
        Token::Seq { len: None },
        Token::U64(1),
        Token::Str("hello"),
        Token::U64(3),
        Token::SeqEnd,
    ];

    assert_eq!(expected, tokens(json_str))
}

#[test]
fn complex() {
    let actual = r#"{
        "name": "John Doe",
        "age": 43,
        "phones": [ "+44 1234567", "+44 2345678" ]
    }"#;

    let expected = vec![
        Token::Map { len: None },
        Token::Str("name"),
        Token::Str("John Doe"),
        Token::Str("age"),
        Token::U64(43),
        Token::Str("phones"),
        Token::Seq { len: None },
        Token::Str("+44 1234567"),
        Token::Str("+44 2345678"),
        Token::SeqEnd,
        Token::MapEnd,
    ];

    assert_eq!(expected, tokens(&actual))
}

fn tokens(json_str: &str) -> Vec<Token> {
    let (token_sink, token_stream) = mpsc::unbounded::<Token>();
    let mut de = serde_json::de::Deserializer::from_str(json_str);

    tokenize(&mut de, token_sink).unwrap();
    token_stream.collect().wait().unwrap()
}
