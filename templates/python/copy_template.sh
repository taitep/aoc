#!/bin/bash

TEMPLATE_DIR=$(dirname "$(realpath "$0")")

cp $TEMPLATE_DIR/parse.py .
cp $TEMPLATE_DIR/solution.py ./part1.py
cp $TEMPLATE_DIR/solution.py ./part2.py
