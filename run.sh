#!/bin/bash

if [ "$1" = "server" ]; then
  echo Running server
  cargo run
elif [ "$1" = "integration_test" ]; then
  echo Runnig integration_tests
fi
