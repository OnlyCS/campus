use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ClassType {
    #[serde(rename = "homeroom")]
    Homeroom,

    #[serde(rename = "scheduled")]
    Scheduled,
}

mod raw {
    use crate::prelude::*;

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct RawClass {
        #[serde(rename = "sourcedId")]
        pub id: model::ClassId,

        pub status: model::Status,
        pub modified: RawDateTime,
        pub metadata: HashMap<String, String>,
        pub title: String,

        #[serde(rename = "classCode")]
        pub code: String,

        #[serde(rename = "classType")]
        pub kind: super::ClassType,
        pub location: String,

        // as in class years, 10th, 11th, 12th, etc
        pub grades: Vec<model::Grade>,

        pub subjects: Vec<String>,
        pub course: Option<model::IdRef>,
        pub school: Option<model::IdRef>,
        pub terms: Vec<model::IdRef>,

        #[serde(rename = "subjectCodes")]
        pub subject_codes: Vec<u16>,

        pub periods: Vec<String>,

        pub resources: Vec<model::IdRef>,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Class {
    pub id: model::ClassId,
    pub status: model::Status,
    pub modified: DateTime<FixedOffset>,
    pub metadata: HashMap<String, String>,
    pub title: String,
    pub code: String,
    pub kind: ClassType,
    pub location: String,
    pub grades: Vec<model::Grade>,
    pub subjects: Vec<String>,
    pub course_id: Option<model::CourseId>,
    pub school_id: Option<model::SchoolId>,
    pub terms: Vec<model::SessionId>,
    pub subject_codes: Vec<u16>,
    pub periods: Vec<String>,
    pub resources: Vec<model::ResourceId>,
}

impl From<raw::RawClass> for Class {
    fn from(raw: raw::RawClass) -> Self {
        Self {
            id: raw.id,
            status: raw.status,
            modified: raw.modified.into_fixed_offset().unwrap(),
            metadata: raw.metadata,
            title: raw.title,
            code: raw.code,
            kind: raw.kind,
            location: raw.location,
            grades: raw.grades,
            subjects: raw.subjects,
            course_id: raw.course.map(|r| r.id).map(|n| model::CourseId(n.0)),
            school_id: raw.school.map(|r| r.id).map(|n| model::SchoolId(n.0)),
            terms: raw
                .terms
                .into_iter()
                .map(|r| r.id)
                .map(|n| model::SessionId(n.0))
                .collect(),
            subject_codes: raw.subject_codes,
            periods: raw.periods,
            resources: raw
                .resources
                .into_iter()
                .map(|r| r.id)
                .map(|n| model::ResourceId(n.0))
                .collect(),
        }
    }
}

impl<'de> Deserialize<'de> for Class {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw = raw::RawClass::deserialize(deserializer)?;
        Ok(raw.into())
    }
}
