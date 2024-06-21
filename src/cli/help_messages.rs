pub const ECHO: &str = "Write arguments to the standard output.
    
Display the ARGs, separated by a single space character and followed by a
newline, on the standard output.";

pub const CAT: &str = "Concatenate FILE(s) to standard output.

With no FILE, or when FILE is -, read standard input.";

pub const HEAD: &str = "Print the first 10 lines of each FILE to standard output.
With more than one FILE, precede each with a header giving the file name.

With no FILE, or when FILE is -, read standard input.";

pub const WC: &str = "Print newline, word, and byte counts for each FILE, and a total line if
more than one FILE is specified.  A word is a non-zero-length sequence of
characters delimited by white space.

With no FILE, or when FILE is -, read standard input.";

pub const UNIQ: &str = "Filter adjacent matching lines from INPUT (or standard input),
writing to OUTPUT (or standard output).

With no options, matching lines are merged to the first occurrence.

Note: 'uniq' does not detect repeated lines unless they are adjacent.
You may want to sort the input first, or use 'sort -u' without 'uniq'.";
