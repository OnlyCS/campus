use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Grade {
    PreK,
    Kindergarten,
    First,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Eighth,
    Ninth,
    Tenth,
    Eleventh,
    Twelfth,
}

impl Serialize for Grade {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let str = match self {
            Self::PreK => "PK",
            Self::Kindergarten => "KG",
            Self::First => "01",
            Self::Second => "02",
            Self::Third => "03",
            Self::Fourth => "04",
            Self::Fifth => "05",
            Self::Sixth => "06",
            Self::Seventh => "07",
            Self::Eighth => "08",
            Self::Ninth => "09",
            Self::Tenth => "10",
            Self::Eleventh => "11",
            Self::Twelfth => "12",
        };

        serializer.serialize_str(str)
    }
}

impl<'de> Deserialize<'de> for Grade {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        match s.as_str() {
            "PK" => Ok(Self::PreK),
            "KG" => Ok(Self::Kindergarten),
            "01" => Ok(Self::First),
            "02" => Ok(Self::Second),
            "03" => Ok(Self::Third),
            "04" => Ok(Self::Fourth),
            "05" => Ok(Self::Fifth),
            "06" => Ok(Self::Sixth),
            "07" => Ok(Self::Seventh),
            "08" => Ok(Self::Eighth),
            "09" => Ok(Self::Ninth),
            "10" => Ok(Self::Tenth),
            "11" => Ok(Self::Eleventh),
            "12" => Ok(Self::Twelfth),
            _ => Err(serde::de::Error::custom(format!("invalid grade: {}", s))),
        }
    }
}
