use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Status {
    Active,
    ToBeDeleted,
}

impl Serialize for Status {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(match self {
            Status::Active => "active",
            Status::ToBeDeleted => "tobedeleted",
        })
    }
}

impl<'de> Deserialize<'de> for Status {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Status, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "active" => Ok(Status::Active),
            "tobedeleted" => Ok(Status::ToBeDeleted),
            "inactive" => Ok(Status::ToBeDeleted), /* deprecated, mapping */
            _ => Err(serde::de::Error::custom(format!("invalid status: {}", s))),
        }
    }
}
