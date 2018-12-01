#!/bin/bash

echo 0 | cat - input | sed 's/^[1-9]/+\$1/g' | xargs | bc
