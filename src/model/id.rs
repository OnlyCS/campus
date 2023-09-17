use std::str::FromStr;

use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Guid {
    pub group1: u32,
    pub group2: u16,
    pub group3: u16,
    pub group4: u16,
    pub group5: u64,
}

impl PartialOrd for Guid {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let chained: u128 = (self.group1 as u128)
            | ((self.group2 as u128) << 32)
            | ((self.group3 as u128) << 48)
            | ((self.group4 as u128) << 64)
            | ((self.group5 as u128) << 80);

        let chained_other: u128 = (other.group1 as u128)
            | ((other.group2 as u128) << 32)
            | ((other.group3 as u128) << 48)
            | ((other.group4 as u128) << 64)
            | ((other.group5 as u128) << 80);

        chained.partial_cmp(&chained_other)
    }
}

impl Ord for Guid {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl FromStr for Guid {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut groups = s.split('-');

        // all hexadecimal
        let group1s = groups.next().context("Could not parse group 1")?;
        let group2s = groups.next().context("Could not parse group 2")?;
        let group3s = groups.next().context("Could not parse group 3")?;
        let group4s = groups.next().context("Could not parse group 4")?;
        let group5s = groups.next().context("Could not parse group 5")?;

        let group1 = u32::from_str_radix(group1s, 16).context("Could not parse group 1")?;
        let group2 = u16::from_str_radix(group2s, 16).context("Could not parse group 2")?;
        let group3 = u16::from_str_radix(group3s, 16).context("Could not parse group 3")?;
        let group4 = u16::from_str_radix(group4s, 16).context("Could not parse group 4")?;
        let group5 = u64::from_str_radix(group5s, 16).context("Could not parse group 5")?;

        Ok(Guid {
            group1,
            group2,
            group3,
            group4,
            group5,
        })
    }
}

impl ToString for Guid {
    fn to_string(&self) -> std::string::String {
        format!(
            "{:08X}-{:04X}-{:04X}-{:04X}-{:012X}",
            self.group1, self.group2, self.group3, self.group4, self.group5
        )
    }
}

macro_rules! id {
	($($id:ident),+) => {
		$(
			#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
			pub struct $id(pub Guid);

			impl FromStr for $id {
				type Err = Error;

				fn from_str(s: &str) -> Result<Self> {
					Ok(Self(Guid::from_str(s)?))
				}
			}

			impl From<std::string::String> for $id {
				fn from(s: std::string::String) -> Self {
					Self(Guid::from_str(&s).unwrap())
				}
			}

			impl ToString for $id {
				fn to_string(&self) -> std::string::String {
					self.0.to_string()
				}
			}

			impl Serialize for $id {
				fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
				where
					S: serde::Serializer,
				{
					serializer.serialize_str(&self.to_string())
				}
			}

			impl<'de> Deserialize<'de> for $id {
				fn deserialize<D>(deserializer: D) -> std::result::Result<$id, D::Error>
				where
					D: serde::Deserializer<'de>,
				{
					let s = String::deserialize(deserializer)?;
					$id::from_str(&s).map_err(serde::de::Error::custom)
				}
			}
		)+

		pub(crate) mod prelude {
			$(
				pub use super::$id;
			)+
		}
	};
}

id!(AnyId, SessionId, ClassId, CourseId, SchoolId, ResourceId);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct IdRef {
    pub href: String,

    #[serde(rename = "sourcedId")]
    pub id: AnyId,

    #[serde(rename = "type")]
    pub kind: String,
}
