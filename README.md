# serde_token

<!-- [![GitHub tag](https://img.shields.io/github/tag/Naereen/StrapDown.js.svg)](https://GitHub.com/Naereen/StrapDown.js/tags/) -->
<!-- [![Build Status](https://semaphoreci.com/api/v1/sunny-g/xdr/branches/master/badge.svg)](https://semaphoreci.com/sunny-g/xdr) -->
[![Crates.io](https://img.shields.io/crates/v/serde_token.svg)](https://crates.io/crates/serde_token)
[![Documentation](https://docs.rs/serde_token/badge.svg)](https://docs.rs/serde_token)
[![MIT license](https://img.shields.io/badge/License-MIT-blue.svg)](https://lbesson.mit-license.org/)

`serde_token` provides a utility function `tokenize` for [transcoding](https://docs.serde.rs/serde_transcode/index.html) a [Serde](https://serde.rs) [deserializer](https://docs.serde.rs/serde/trait.Deserializer.html) into a [`futures::Sink`](https://docs.rs/futures/0.1.27/futures/sink/trait.Sink.html) of `Token`s.

## Installation

Install from [Crates.io](https://crates.io/crates/serde_token):

```toml
[dependencies]
serde_token = "0.0.2"
```

## Usage

```rust
use futures::{unsync::mpsc::unbounded, Future, Sink, Stream};
use serde_json::Deserializer;
use serde_token::{tokenize, Token};

let mut de = Deserializer::from_str(r#" [ {"a":false}, "hello", 3 ] "#);
let (token_sink, token_stream) = unbounded::<Token>();

tokenize(&mut de, token_sink).unwrap();

let expected = token_stream.collect().wait().unwrap();
assert_eq!(expected, vec![
    Token::Seq { len: None },
    Token::Map { len: None },
    Token::Str("a"),
    Token::Bool(false),
    Token::MapEnd,
    Token::Str("hello"),
    Token::U64(3),
    Token::SeqEnd,
])
```

## How It Works (and Limitations)

`tokenize` takes in a [`serde::Deserializer`](https://docs.serde.rs/serde/trait.Deserializer.html), which will walk thru the encoded input, and a [`futures::Sink`](https://docs.rs/futures/0.1.27/futures/sink/trait.Sink.html), which will be sent the deserialized tokens. Two things to note:

1. Under the hood, we use [`serde_transcode`](https://docs.serde.rs/serde_transcode/index.html) to directly map the given `Deserializer` to the `Token` `Serializer`. However, `serde_transcode` "drives" the transcoding process with [`deserialize_any`](https://docs.serde.rs/serde/trait.Deserializer.html#tymethod.deserialize_any), meaning that encoding formats that require type hints cannot be tokenized without an intermediary step (not provided by this library).
2. Because of the aforementioned limitation, the `Tokenizer` struct, which is private and can only be used with the provided `Deserializer`, uses `unsafe` twice to `serialize` tokens that borrow their underlying data from the `Deserializer` (`Token::Str(&'de str)` and `Token::Bytes(&'de [u8])`). However, this should be safe because the tokens share the same lifetime as the provided `Deserializer` and should thus expire no later than the `Deserializer` itself.

## TODO

- [ ] look into how we use the [`Sink`](https://docs.rs/futures/0.1.27/futures/sink/trait.Sink.html):
  - should probably periodically `poll_complete` in case the sink is bounded
  - add support for back-pressure
  - handle `Async::NotReady` appropriately
- [ ] provide better guarads around our use of `unsafe`

## Changelog

| Version | Change Summary |
| ------- | ---------------|
| [v0.0.2](https://crates.io/crates/serde_token/0.0.2) | adds README |
| [v0.0.1](https://crates.io/crates/serde_token/0.0.1) | initial release |

## Contributing

1. Fork it [https://github.com/your_username/serde_token/fork](https://github.com/sunny-g/serde_token/fork)
2. Create your feature branch (`git checkout -b feature/fooBar`)
3. Commit your changes (`git commit -am 'Add some fooBar'`)
4. Push to the branch (`git push origin feature/fooBar`)
5. Create a new Pull Request

## Maintainers

- Sunny G - [@sunny-g](https://github.com/sunny-g)

<!-- ## Contributors -->

## License

MIT
