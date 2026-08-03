use serde::{Deserialize, Serialize, Serializer};
use crate::data::info::*;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ResponseEnum {
    License(License),
    OpenSubsonicExtensions(OpenSubsonicExtensionList),
    Unit,
}

impl Into<ResponseEnum> for () {
    fn into(self) -> ResponseEnum {
        ResponseEnum::Unit
    }
}

impl Into<ResponseEnum> for License {
    fn into(self) -> ResponseEnum {
        ResponseEnum::License(self)
    }
}

impl Into<ResponseEnum> for OpenSubsonicExtensionList{
    fn into(self) -> ResponseEnum {
        ResponseEnum::OpenSubsonicExtensions(self)
    }
}

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
