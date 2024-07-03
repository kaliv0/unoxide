#!/usr/bin/env bash

# used for testing ls command
set -u
DIR=${1:-$PWD}

TEST_DIR=${DIR}/tests/resources/ls/inputs/

chmod 755 ${TEST_DIR}/dir
chmod 600 ${TEST_DIR}/fox.txt
chmod 644 ${TEST_DIR}/.hidden ${TEST_DIR}/empty.txt \
    ${TEST_DIR}/bustle.txt ${TEST_DIR}/dir/.gitkeep \
    ${TEST_DIR}/dir/spiders.txt

echo "Done, fixed files in \"$DIR\"."
