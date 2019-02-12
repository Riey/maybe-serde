# maybe-serde

![Doc.rs](https://docs.rs/maybe-serde/badge.svg)

Using `specialization` feature for present optional impl serde::{Serialize, Deserialize}

So it require `nightly` rust

All types impl MaydeSer and MeybeDe

If type impl Serialize then `MeybeSer::maybe_serialize` return `Serialize::serialize` with Some but not then return None

same as MaydeDe and Deserialize

## Example(test code in src)

```rust
#[cfg(test)]
mod tests {
    use super::{MaybeDe, MaybeSer, MaybeSerde};
    use serde_derive::{Deserialize, Serialize};
    use serde_json::{de::StrRead, Deserializer, Result, Serializer, to_string, from_str};

    fn ser_fn<V>(val: &V) -> Option<Result<()>>
    where
        V: MaybeSer,
    {
        let mut buf = Vec::new();
        val.maybe_serialize(&mut Serializer::new(&mut buf))
    }

    fn de_fn<'de, V>(text: &'de str) -> Option<Result<V>>
    where
        V: MaybeDe<'de>,
    {
        V::maybe_deserialize(&mut Deserializer::new(StrRead::new(text)))
    }

    struct NoneSerde(i32);

    #[derive(Serialize, Deserialize)]
    struct SomeSerde(i32);

    #[test]
    fn serde_none() {
        struct NoneSerde(i32);

        assert!(ser_fn(&NoneSerde(123)).is_none());
        assert!(de_fn::<NoneSerde>("123").is_none());
    }

    #[test]
    fn serde_some() {

        assert!(ser_fn(&SomeSerde(123)).is_some());
        assert_eq!(de_fn::<SomeSerde>("123").unwrap().unwrap().0, 123);
    }

    #[test]
    fn serde_none_maybe_serde() {
        let maybe_serde = MaybeSerde(Some(NoneSerde(123)));

        assert!(to_string(&maybe_serde).is_ok());
        assert!(from_str::<MaybeSerde<NoneSerde>>("").unwrap().0.is_none());
    }

    #[test]
    fn serde_some_maybe_serde() {
        let maybe_serde = MaybeSerde(Some(SomeSerde(123)));

        assert_eq!(to_string(&maybe_serde).unwrap(), "123");
        assert_eq!(from_str::<MaybeSerde<SomeSerde>>("123").unwrap().0.unwrap().0, 123);
    }
}
```
