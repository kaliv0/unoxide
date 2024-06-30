- fix or remove failing tests from 'head'
- write custom_vs_actual tests and remove the other ones
- add tests for errors as well
<br>
- should we pass &str instead of String unless necessary?
- research about passing and returning  Box<dyn Write> between functions
- cut -> research how to handle extract.copy() etc
- extract error messages to const file
- rename project and main command
- after refactoring tests -> remove sys-info (used for 'Windows' tests)
- remove lib.rs?
<br><br>
------------------
cargo clippy -- --allow clippy::too_many_arguments
