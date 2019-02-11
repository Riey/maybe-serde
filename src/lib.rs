#![feature(specialization)]

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// This trait is impl for every types
///
/// If type implement `Serialize` then `maybe_serialize` return `Serialize::serialize` value with `Some`
///
/// Otherwise if type doesn't implement `Serialize` then `maybe_serialize` return just `None`
pub trait MaybeSer {
    fn maybe_serialize<S>(&self, serializer: S) -> Option<Result<S::Ok, S::Error>>
    where
        S: Serializer;
}

/// This trait is impl for every types
///
/// If type implement `Deserialize` then `maybe_deserialize` return `Deserialize::deserialize` value with `Some`
///
/// Otherwise if type doesn't implement `Deserialize` then `maybe_deserialize` return just `None`
pub trait MaybeDe<'de>: Sized {
    fn maybe_deserialize<D>(deserializer: D) -> Option<Result<Self, D::Error>>
    where
        D: Deserializer<'de>;
}

/// Always return `None`
impl<T> MaybeSer for T {
    default fn maybe_serialize<S>(
        &self,
        _serializer: S,
    ) -> Option<Result<<S as Serializer>::Ok, <S as Serializer>::Error>>
    where
        S: Serializer,
    {
        None
    }
}

/// Always return `None`
impl<'de, T> MaybeDe<'de> for T {
    default fn maybe_deserialize<D>(
        _deserializer: D,
    ) -> Option<Result<Self, <D as Deserializer<'de>>::Error>>
    where
        D: Deserializer<'de>,
    {
        None
    }
}

/// Always return `Some`
impl<T> MaybeSer for T
where
    T: Serialize,
{
    fn maybe_serialize<S>(
        &self,
        serializer: S,
    ) -> Option<Result<<S as Serializer>::Ok, <S as Serializer>::Error>>
    where
        S: Serializer,
    {
        Some(self.serialize(serializer))
    }
}

/// Always return `Some`
impl<'de, T> MaybeDe<'de> for T
where
    T: Deserialize<'de>,
{
    fn maybe_deserialize<D>(
        deserializer: D,
    ) -> Option<Result<Self, <D as Deserializer<'de>>::Error>>
    where
        D: Deserializer<'de>,
    {
        Some(T::deserialize(deserializer))
    }
}

#[cfg(test)]
mod tests {
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
}
