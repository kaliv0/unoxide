- extract  'Err(err) => eprintln!("{command}: {filename}: {err}")' -> equal for all commands
<br>
- fix or remove failing tests from 'head'
- write custom_vs_actual tests and remove the other ones
- add tests for errors as well
<br>
- add support for negative line/byte count in head
- move cli/help_messages to utils module
- replace function_name with namefn (or write custom macro?)
- research for Optional<&str> and as_deref() (e.g. uniq)
- add return types in wc?
- should we pass &str instead of String unless necessary?
- refactor cat to use line_buffer instead of file.lines (e.g. uniq) or vice versa
- research about passing and returning  Box<dyn Write> between functions