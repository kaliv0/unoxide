use clap::Subcommand;

#[derive(Subcommand)]
pub enum Subcommands {
    #[clap(about = "Write arguments to the standard output.
    
Display the ARGs, separated by a single space character and followed by a
newline, on the standard output.")]
    Echo {
        #[arg(required(true))]
        text: Vec<String>,

        #[arg(short('n'))]
        omit_newline: bool,
    },

    #[clap(about = "Concatenate FILE(s) to standard output.

With no FILE, or when FILE is -, read standard input.")]
    Cat {
        #[arg(value_name = "FILE", default_value = "-")]
        files: Vec<String>,

        #[arg(short('n'), long("number"), conflicts_with("number_nonblank_lines"))]
        number_lines: bool,

        #[arg(short('b'), long("number-nonblank"))]
        number_nonblank_lines: bool,

        #[arg(short('s'), long("squeeze-blank"))]
        squeeze_blank_lines: bool,
    },
}
