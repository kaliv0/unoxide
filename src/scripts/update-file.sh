#!/usr/bin/env bash

# used for testing tail -f command
set -e 

seed_file(){
  dst="$1"
  if [ -z $dst ]; then
    echo "Output path missing"
    return 1
  fi

  > $dst
  val=1
  while : 
  do
    echo "Hello $val" | tee -a $dst 
    ((val=val+1))
    sleep 1
  done
}

seed_file $1
