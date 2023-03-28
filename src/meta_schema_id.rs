use clap::ValueEnum;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum MetaSchemaId {
    #[clap(name = "https://json-schema.org/draft/2020-12/schema")]
    Draft202012,
    #[clap(name = "https://json-schema.org/draft/2019-09/schema")]
    Draft201909,
    #[clap(name = "http://json-schema.org/draft-07/schema#")]
    Draft07,
    #[clap(name = "http://json-schema.org/draft-06/schema#")]
    Draft06,
    #[clap(name = "http://json-schema.org/draft-04/schema#")]
    Draft04,
}
