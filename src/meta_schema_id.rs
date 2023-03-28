use std::fmt::Display;

use crate::schemas;
use clap::ValueEnum;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum MetaSchemaId {
    #[clap(name = schemas::draft_2020_12::meta::META_SCHEMA_ID)]
    Draft202012,

    #[clap(name = schemas::draft_2019_09::meta::META_SCHEMA_ID)]
    Draft201909,

    #[clap(name = schemas::draft_07::meta::META_SCHEMA_ID)]
    Draft07,

    #[clap(name = schemas::draft_06::meta::META_SCHEMA_ID)]
    Draft06,

    #[clap(name = schemas::draft_04::meta::META_SCHEMA_ID)]
    Draft04,
}

impl Display for MetaSchemaId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MetaSchemaId::Draft202012 => f.write_str(schemas::draft_2020_12::meta::META_SCHEMA_ID),
            MetaSchemaId::Draft201909 => f.write_str(schemas::draft_2019_09::meta::META_SCHEMA_ID),
            MetaSchemaId::Draft07 => f.write_str(schemas::draft_07::meta::META_SCHEMA_ID),
            MetaSchemaId::Draft06 => f.write_str(schemas::draft_06::meta::META_SCHEMA_ID),
            MetaSchemaId::Draft04 => f.write_str(schemas::draft_04::meta::META_SCHEMA_ID),
        }
    }
}
