- fix or remove failing tests from 'head'
- write custom_vs_actual tests and remove the other ones
- add tests for errors as well
<br>
- add support for negative line/byte count in head?
- research for Optional<&str> and as_deref() (e.g. uniq)
- should we pass &str instead of String unless necessary?
- refactor cat to use line_buffer instead of file.lines (e.g. uniq) or vice versa
- research about passing and returning  Box<dyn Write> between functions
- find -> add -size and -delete
- cut -> research how to handle extract.copy() etc
- refactor chained closures in 'cut' and split into small separate functions
- extract error messages to const file
<br><br>
------------------
cargo clippy -- --allow clippy::too_many_arguments