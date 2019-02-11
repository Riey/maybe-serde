# maybe-serde

Using `specialization` feature for present optional impl serde::{Serialize, Deserialize}

All type impl MaydeSer and with impl Serialize it return Some but not then return None

same as MaydeDe and Deserialize

## Example(test code in src)

```rust
    use super::{MaybeDe, MaybeSer};
    use serde_derive::{Deserialize, Serialize};
    use serde_json::{de::StrRead, Deserializer, Result, Serializer};

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

    #[test]
    fn serde_none() {
        struct NoneSerde(i32);

        assert!(ser_fn(&NoneSerde(123)).is_none());
        assert!(de_fn::<NoneSerde>("123").is_none());
    }

    #[test]
    fn serde_some() {
        #[derive(Serialize, Deserialize)]
        struct SomeSerde(i32);

        assert!(ser_fn(&SomeSerde(123)).is_some());
        assert_eq!(de_fn::<SomeSerde>("123").unwrap().unwrap().0, 123);
    }
```
