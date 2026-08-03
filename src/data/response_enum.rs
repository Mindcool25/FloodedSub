use serde::{Deserialize, Serialize, Serializer};
use crate::data::info::*;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ResponseEnum {
    License(License),
    OpenSubsonicExtensions(OpenSubsonicExtensionList),
    Unit,
}

macro_rules! impl_into_response_enum {
    {$resp:ident} => {
        impl From<$resp> for ResponseEnum {
            fn from(value: $resp) -> Self {
                ResponseEnum::$resp(value)
            }
        }
    };

    {$resp:ty as $tag:ident} => {
        impl From<$resp> for ResponseEnum {
            fn from(value: $resp) -> Self {
                ResponseEnum::$tag(value)
            }
        }
    };
}

impl Into<ResponseEnum> for () {
    fn into(self) -> ResponseEnum {
        ResponseEnum::Unit
    }
}

impl_into_response_enum!{License}
impl_into_response_enum!{OpenSubsonicExtensionList as OpenSubsonicExtensions}

pub fn into_response_enum<T, S>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
where
    T: Into<ResponseEnum> + Clone,
    S: Serializer
{
    match value {
        None => serializer.serialize_unit(),
        Some(inner) => Into::<ResponseEnum>::into(inner.clone()).serialize(serializer)
    }
}
