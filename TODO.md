- fix or remove failing tests from 'head'
- write custom_vs_actual tests and remove the other ones
- add tests for errors as well
<br>
- research for Optional<&str> and as_deref() (e.g. uniq)
- should we pass &str instead of String unless necessary?
- research about passing and returning  Box<dyn Write> between functions
- find -> add -size and -delete?
- cut -> research how to handle extract.copy() etc
- extract error messages to const file
- rename poject and main command
- rearrange and combine utils (most of them enums)
- remove empty lines from help messages
- after refactoring tests -> remove sys-info (used for 'Windows' tests)
<br><br>
------------------
cargo clippy -- --allow clippy::too_many_argumentssys-info