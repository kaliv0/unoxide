use clap::Subcommand;

#[derive(Subcommand)]
pub enum Subcommands {
    #[clap(about = "")] // add description
    Echo {
        #[arg(required(true))]
        text: Vec<String>,

        #[arg(short('n'))]
        omit_newline: bool,
    },
    #[clap(about = "")]
    Cat {
        #[arg(value_name = "FILE", default_value = "-")]
        files: Vec<String>,

        #[arg(short('n'), long("number"), conflicts_with("number_nonblank_lines"))]
        number_lines: bool,

        #[arg(short('b'), long("number-nonblank"))]
        number_nonblank_lines: bool,
    },
}
