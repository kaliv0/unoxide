- fix or remove failing tests from 'head'
- write custom_vs_actual tests and remove the other ones
- add tests for errors as well
<br>
- add support for negative line/byte count in head?
- move cli/help_messages to utils module
- research for Optional<&str> and as_deref() (e.g. uniq)
- add return types in wc?
- should we pass &str instead of String unless necessary?
- refactor cat to use line_buffer instead of file.lines (e.g. uniq) or vice versa
- research about passing and returning  Box<dyn Write> between functions
- find -> add -size and -delete
- re-arrange mods and usings in all files
<br><br>
------------------
cargo clippy -- --allow clippy::too_many_arguments