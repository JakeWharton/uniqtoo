#!/usr/bin/env bash

printf '\e[34m$\e[0m '
sleep 1
echo 'git ls-files | xargs -L1 sh -c '"'"'sleep .5 && echo "${0##*.}"'"'"' | uniqtoo'

git ls-files | xargs -L1 sh -c 'sleep .5 && echo "${0##*.}"' | ./target/debug/uniqtoo
sleep 2
echo
