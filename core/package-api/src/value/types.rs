use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(TS, Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub enum PrimitiveType {
    Int,
    Float,
    String,
    Bool,
}

pub trait FromType<T> {
    fn from_type() -> Self;
}

pub trait IntoType<T> {
    fn into_type() -> T;
}

impl FromType<i32> for PrimitiveType {
    fn from_type() -> Self {
        PrimitiveType::Int
    }
}

impl FromType<f64> for PrimitiveType {
    fn from_type() -> Self {
        PrimitiveType::Float
    }
}

impl FromType<String> for PrimitiveType {
    fn from_type() -> Self {
        PrimitiveType::String
    }
}

impl FromType<&str> for PrimitiveType {
    fn from_type() -> Self {
        PrimitiveType::String
    }
}

impl FromType<bool> for PrimitiveType {
    fn from_type() -> Self {
        PrimitiveType::Bool
    }
}

impl<U: IntoType<PrimitiveType>> From<U> for PrimitiveType {
    fn from(_: U) -> PrimitiveType {
        U::into_type()
    }
}

impl<T, U> IntoType<U> for T
where
    U: FromType<T>,
{
    fn into_type() -> U {
        U::from_type()
    }
}

#[derive(TS, Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
#[ts(export)]
#[serde(rename_all = "camelCase", tag = "variant", content = "value")]
pub enum ListType {
    Primitive(PrimitiveType),
    // Struct(StructType),
}

impl From<PrimitiveType> for ListType {
    fn from(primitive_type: PrimitiveType) -> Self {
        ListType::Primitive(primitive_type)
    }
}

#[derive(TS, Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
#[ts(export)]
#[serde(rename_all = "camelCase", tag = "variant", content = "value")]
pub enum ValueType {
    Primitive(PrimitiveType),
    List(ListType),
}

impl From<PrimitiveType> for ValueType {
    fn from(primitive_type: PrimitiveType) -> Self {
        ValueType::Primitive(primitive_type)
    }
}

impl From<ListType> for ValueType {
    fn from(list_type: ListType) -> Self {
        ValueType::List(list_type)
    }
}
