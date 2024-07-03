- fix or remove failing tests from 'head'
- write custom_vs_actual tests and remove the other ones
- add tests for errors as well
<br>
- cut -> research how to handle extract.copy() etc
- extract error messages to const file
- rename project and main command
<br><br>
------------------
fix implemntation:
- cat -n/-b with multiple files -> don't start each from zero

- test_cat::skips_bad_file similar to test_find::skips_bad_dir
- generate_bad_file common to cat, comm, find, grep, ls, uniq
- after removing expected dirs -> remove inputs nested dirs, move files one level up

- extract - 

- NB -> comm orders output differently than linux version
     -> uniq count and wc are formatted differently