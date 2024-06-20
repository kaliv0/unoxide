- extract  'Err(err) => eprintln!("{command}: {filename}: {err}")' -> equal for all commands

- fix or remove failing tests from 'head'
- write custom_vs_actual tests and remove the other ones
- add tests for errors as well

- add support for negative line/byte count in head
- move cli/help_messages to utils module
- replace function_name with namefn (or write custom macro?)