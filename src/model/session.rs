use crate::prelude::*;

mod raw {
    use crate::prelude::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum RawAcademicSessionKind {
        GradingPeriod,
        Semester,
        SchoolYear,
        Term,
    }

    impl Serialize for RawAcademicSessionKind {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serializer.serialize_str(match self {
                RawAcademicSessionKind::GradingPeriod => "gradingperiod",
                RawAcademicSessionKind::Semester => "semester",
                RawAcademicSessionKind::SchoolYear => "schoolyear",
                RawAcademicSessionKind::Term => "term",
            })
        }
    }

    impl<'de> Deserialize<'de> for RawAcademicSessionKind {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;

            match s.as_str() {
                "gradingPeriod" => Ok(RawAcademicSessionKind::GradingPeriod),
                "semester" => Ok(RawAcademicSessionKind::Semester),
                "schoolYear" => Ok(RawAcademicSessionKind::SchoolYear),
                "term" => Ok(RawAcademicSessionKind::Term),
                _ => Err(serde::de::Error::custom(format!(
                    "invalid academic session kind: {}",
                    s
                ))),
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct RawAcademicSession {
        #[serde(rename = "sourcedId")]
        pub id: model::SessionId,
        pub status: model::Status,

        #[serde(rename = "dateLastModified")]
        pub modified: RawDateTime,

        #[serde(rename = "startDate")]
        pub start: RawDate,

        #[serde(rename = "endDate")]
        pub end: RawDate,

        #[serde(rename = "type")]
        pub kind: RawAcademicSessionKind,

        #[serde(rename = "title")]
        pub name: String,

        #[serde(rename = "schoolYear")]
        pub year: u16,

        pub parent: Option<model::IdRef>,
        pub children: Vec<model::IdRef>,

        pub metadata: HashMap<String, String>,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SessionType {
    GradingPeriod,
    Semester,
    SchoolYear,
    Term,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Session {
    pub id: model::SessionId,
    pub status: model::Status,
    pub modified: DateTime<FixedOffset>,
    pub title: String,
    pub start: Date,
    pub end: Date,
    pub parent_id: Option<model::SessionId>,
    pub children: Vec<model::SessionId>,
    pub school_year: u16,
    pub kind: SessionType,
}

impl From<raw::RawAcademicSession> for Session {
    fn from(value: raw::RawAcademicSession) -> Self {
        Self {
            id: value.id,
            status: value.status,
            modified: value.modified.into_fixed_offset().unwrap(),
            title: value.name,
            start: value.start.into_naive().unwrap(),
            end: value.end.into_naive().unwrap(),
            parent_id: value.parent.map(|x| x.id).map(|n| model::SessionId(n.0)),
            children: value
                .children
                .into_iter()
                .map(|x| x.id)
                .map(|n| model::SessionId(n.0))
                .collect(),
            school_year: value.year,
            kind: match value.kind {
                raw::RawAcademicSessionKind::GradingPeriod => SessionType::GradingPeriod,
                raw::RawAcademicSessionKind::Semester => SessionType::Semester,
                raw::RawAcademicSessionKind::SchoolYear => SessionType::SchoolYear,
                raw::RawAcademicSessionKind::Term => SessionType::Term,
            },
        }
    }
}
