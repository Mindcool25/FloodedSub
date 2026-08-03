use std::borrow::Cow;

use serde::{Deserialize, Serialize, Serializer};
use crate::data::info::*;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ResponseEnum<'a> {
    License(Cow<'a, License>),
    OpenSubsonicExtensions(Cow<'a, OpenSubsonicExtensionList>),
    Unit,
}

pub trait ToResponseEnum: Clone {
    fn to_response_enum<'a>(&'a self) -> ResponseEnum<'a>;
}

macro_rules! impl_into_response_enum {
    {$resp:ident} => {
        impl ToResponseEnum for $resp {
            fn to_response_enum<'a>(&'a self) -> ResponseEnum<'a> {
                ResponseEnum::$resp(Cow::Borrowed(&self))
            }
        }
    };

    {$resp:ty as $tag:ident} => {
        impl ToResponseEnum for $resp {
            fn to_response_enum<'a>(&'a self) -> ResponseEnum<'a> {
                ResponseEnum::$tag(Cow::Borrowed(&self))
            }
        }
    };
}

impl ToResponseEnum for () {
    fn to_response_enum<'a>(&'a self) -> ResponseEnum<'a> {
        ResponseEnum::Unit
    }
}

impl_into_response_enum!{License}
impl_into_response_enum!{OpenSubsonicExtensionList as OpenSubsonicExtensions}

pub fn into_response_enum<'a, T, S>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
where
    T: ToResponseEnum,
    S: Serializer
{
    match value {
        None => serializer.serialize_unit(),
        Some(inner) => inner.to_response_enum().serialize(serializer)
    }
}
