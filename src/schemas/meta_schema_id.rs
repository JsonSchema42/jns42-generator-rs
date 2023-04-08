use crate::schemas;
use clap::ValueEnum;
use schemas::{draft_04, draft_06, draft_07, draft_2019_09, draft_2020_12};
use std::fmt::Display;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, ValueEnum)]
pub enum MetaSchemaId {
    Unknown,

    #[clap(name = draft_2020_12::META_SCHEMA_ID)]
    Draft202012,

    #[clap(name = draft_2019_09::META_SCHEMA_ID)]
    Draft201909,

    #[clap(name = draft_07::META_SCHEMA_ID)]
    Draft07,

    #[clap(name = draft_06::META_SCHEMA_ID)]
    Draft06,

    #[clap(name = draft_04::META_SCHEMA_ID)]
    Draft04,
}

impl Display for MetaSchemaId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.into())
    }
}

impl From<&MetaSchemaId> for &'static str {
    fn from(value: &MetaSchemaId) -> Self {
        match value {
            MetaSchemaId::Draft202012 => draft_2020_12::META_SCHEMA_ID,
            MetaSchemaId::Draft201909 => draft_2019_09::META_SCHEMA_ID,
            MetaSchemaId::Draft07 => draft_07::META_SCHEMA_ID,
            MetaSchemaId::Draft06 => draft_06::META_SCHEMA_ID,
            MetaSchemaId::Draft04 => draft_04::META_SCHEMA_ID,
            MetaSchemaId::Unknown => "",
        }
    }
}

impl From<&str> for MetaSchemaId {
    fn from(value: &str) -> Self {
        match value {
            draft_2020_12::META_SCHEMA_ID => MetaSchemaId::Draft202012,
            draft_2019_09::META_SCHEMA_ID => MetaSchemaId::Draft201909,
            draft_07::META_SCHEMA_ID => MetaSchemaId::Draft07,
            draft_06::META_SCHEMA_ID => MetaSchemaId::Draft06,
            draft_04::META_SCHEMA_ID => MetaSchemaId::Draft04,
            _ => MetaSchemaId::Unknown,
        }
    }
}
