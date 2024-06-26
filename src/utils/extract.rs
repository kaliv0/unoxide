use std::ops::Range;

#[derive(clap::Args, Debug, Clone)]
#[group(required = true, multiple = false)]
pub struct ArgsExtract {
    /// select only these fields
    #[arg(short, long, value_name = "FIELDS")]
    pub fields: Option<String>,

    /// select only these bytes
    #[arg(short, long, value_name = "BYTES")]
    pub bytes: Option<String>,

    /// select only these characters
    #[arg(short, long, value_name = "CHARS")]
    pub chars: Option<String>,
}

pub type PositionList = Vec<Range<usize>>;

#[derive(Debug)]
pub enum Extract {
    Fields(PositionList),
    Bytes(PositionList),
    Chars(PositionList),
}
