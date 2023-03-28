use std::fmt::Display;

use crate::schema;
use clap::ValueEnum;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum MetaSchemaId {
    #[clap(name = schema::draft_2020_12::meta::META_SCHEMA_ID)]
    Draft202012,

    #[clap(name = schema::draft_2019_09::meta::META_SCHEMA_ID)]
    Draft201909,

    #[clap(name = schema::draft_07::meta::META_SCHEMA_ID)]
    Draft07,

    #[clap(name = schema::draft_06::meta::META_SCHEMA_ID)]
    Draft06,

    #[clap(name = schema::draft_04::meta::META_SCHEMA_ID)]
    Draft04,
}

impl Display for MetaSchemaId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MetaSchemaId::Draft202012 => f.write_str(schema::draft_2020_12::meta::META_SCHEMA_ID),
            MetaSchemaId::Draft201909 => f.write_str(schema::draft_2019_09::meta::META_SCHEMA_ID),
            MetaSchemaId::Draft07 => f.write_str(schema::draft_07::meta::META_SCHEMA_ID),
            MetaSchemaId::Draft06 => f.write_str(schema::draft_06::meta::META_SCHEMA_ID),
            MetaSchemaId::Draft04 => f.write_str(schema::draft_04::meta::META_SCHEMA_ID),
        }
    }
}
