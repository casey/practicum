#!/usr/bin/env bash

can() {
  command -v $1 > /dev/null
  return $?
}

if ! can cargo; then
  echo 'rust does not appear to be installed'
  echo 'please run the following command to install it:'
  echo
  echo 'curl https://sh.rustup.rs -sSf | sh'
  exit 1
fi

cd command

cargo build

if [[ ! -f ./target/debug/p ]]; then
  echo 'Failed to build "p" command!'
  exit 1
fi

exec ./target/debug/p "$@"
