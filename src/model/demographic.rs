use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Gender {
    #[serde(rename = "male")]
    Male,

    #[serde(rename = "female")]
    Female,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ethnicity {
    NativeAmerican,
    Asian,
    Black,
    PacificIslander,
    White,
    Hispanic,
}

mod raw {
    use crate::prelude::*;

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct RawDemographic {
        #[serde(rename = "sourcedId")]
        pub id: model::ResourceId,
        pub status: model::Status,
        pub modified: RawDateTime,
        pub metadata: HashMap<String, String>,

        #[serde(rename = "birthDate")]
        pub dob: Option<RawDate>,

        #[serde(rename = "sex")]
        pub gender: Option<super::Gender>,

        #[serde(rename = "americanIndianOrAlaskaNative", default)]
        pub native_american: bool,
        pub asian: bool,

        #[serde(rename = "blackOrAfricanAmerican", default)]
        pub black: bool,

        #[serde(rename = "nativeHawaiianOrOtherPacificIslander", default)]
        pub pacific_islander: bool,

        #[serde(default)]
        pub white: bool,

        #[serde(rename = "demographicRaceTwoOrMoreRaces", default)]
        pub multi_race: bool,

        #[serde(rename = "hispanicOrLatinoEthnicity", default)]
        pub hispanic: bool,

        #[serde(rename = "countryOfBirthCode")]
        pub country_of_birth: Option<String>,

        #[serde(rename = "stateOfBirthAbbreviation")]
        pub state_of_birth: Option<String>,

        #[serde(rename = "cityOfBirth")]
        pub city_of_birth: Option<String>,

        #[serde(rename = "publicSchoolResidenceStatus")]
        pub residency: Option<String>,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Demographic {
    pub id: model::ResourceId,
    pub status: model::Status,
    pub modified: DateTime<FixedOffset>,
    pub metadata: HashMap<String, String>,

    pub dob: Option<Date>,
    pub gender: Option<Gender>,
    pub ethnicity: Vec<Ethnicity>,
    pub country_of_birth: Option<String>,
    pub state_of_birth: Option<String>,
    pub city_of_birth: Option<String>,
    pub residency: Option<String>,
}

impl From<raw::RawDemographic> for Demographic {
    fn from(value: raw::RawDemographic) -> Self {
        let mut ethnicity = Vec::new();

        if value.asian {
            ethnicity.push(Ethnicity::Asian);
        }

        if value.black {
            ethnicity.push(Ethnicity::Black);
        }

        if value.hispanic {
            ethnicity.push(Ethnicity::Hispanic);
        }

        if value.native_american {
            ethnicity.push(Ethnicity::NativeAmerican);
        }

        if value.pacific_islander {
            ethnicity.push(Ethnicity::PacificIslander);
        }

        if value.white {
            ethnicity.push(Ethnicity::White);
        }

        Self {
            id: value.id,
            status: value.status,
            modified: value.modified.into_fixed_offset().unwrap(),
            metadata: value.metadata,

            dob: value.dob.map(|n| n.into_naive()).flatten(),
            gender: value.gender,
            ethnicity: ethnicity,
            country_of_birth: value.country_of_birth,
            state_of_birth: value.state_of_birth,
            city_of_birth: value.city_of_birth,
            residency: value.residency,
        }
    }
}
