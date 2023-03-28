use clap::ValueEnum;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum MetaSchemaId {
    Draft202012,
    Draft201909,
    Draft07,
    Draft06,
    Draft04,
}
