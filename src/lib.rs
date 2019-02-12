#![feature(specialization)]

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Maybe implement Serialize
///
/// This trait is impl for all types
///
/// If type implement `Serialize` then `maybe_serialize` return `Serialize::serialize` value with `Some`
///
/// Otherwise if type doesn't implement `Serialize` then `maybe_serialize` return just `None`
pub trait MaybeSer {
    /// true if implement `serde::Serialize`
    const IMPL_SERIALIZE: bool;

    fn maybe_serialize<S>(&self, serializer: S) -> Option<Result<S::Ok, S::Error>>
    where
        S: Serializer;
}

/// Maybe implement Deserialize
///
/// This trait is impl for all types
///
/// If type implement `Deserialize` then `maybe_deserialize` return `Deserialize::deserialize` value with `Some`
///
/// Otherwise if type doesn't implement `Deserialize` then `maybe_deserialize` return just `None`
pub trait MaybeDe<'de>: Sized {
    /// true if implement `serde::Deserialize`
    const IMPL_DESERIALIZE: bool;

    fn maybe_deserialize<D>(deserializer: D) -> Option<Result<Self, D::Error>>
    where
        D: Deserializer<'de>;
}

/// Always return `None`
impl<T> MaybeSer for T {

    /// Always return `false`
    default const IMPL_SERIALIZE: bool = false;

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
    /// Always return `false`
    default const IMPL_DESERIALIZE: bool = false;

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
    /// Always return `true`
    const IMPL_SERIALIZE: bool = true;

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
    /// Always return `true`
    const IMPL_DESERIALIZE: bool = true;

    fn maybe_deserialize<D>(
        deserializer: D,
    ) -> Option<Result<Self, <D as Deserializer<'de>>::Error>>
    where
        D: Deserializer<'de>,
    {
        Some(T::deserialize(deserializer))
    }
}

/// Helper type for connect serde and maybe-serde
///
/// This type implement both `serde::{Serialize, Deserialize}` for all T
///
/// It's just helper type and not necessary
///
/// you could define your own type using MaybeSer and MaybeDe
///
/// ### When Serialize
/// if T: Serialize and Option is Some then just serialize T itself (doesn't serialize as Option!)
///
/// and T: !Serialize or Option is None then serialize None
///
/// maybe `#[serde(skip_serializing_if = "T::IMPL_SERIALIZE")]` attribute could be help
///
/// ### When Deserialize
/// if T: Deserialize then get Some<T>
///
/// and T: !Deserialize just None
pub struct MaybeSerde<T>(pub Option<T>);

impl<T> Into<Option<T>> for MaybeSerde<T> {
    fn into(self) -> Option<T> { self.0 }
}

impl<T: MaybeSer> Serialize for MaybeSerde<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
        S: Serializer {

        if T::IMPL_SERIALIZE {
            match &self.0 {
                Some(dat) => dat.maybe_serialize(serializer).unwrap(),
                None => serializer.serialize_none(),
            }
        } else {
            serializer.serialize_none()
        }
    }
}

impl<'de, T: MaybeDe<'de>> Deserialize<'de> for MaybeSerde<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
        D: Deserializer<'de> {

        if T::IMPL_DESERIALIZE {
            Ok(MaybeSerde(Some(T::maybe_deserialize(deserializer).unwrap()?)))
        } else {
            Ok(MaybeSerde(None))
        }
    }
}

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
