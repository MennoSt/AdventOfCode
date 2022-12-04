#!/bin/bash
# source: https://github.com/wimglenn/advent-of-code-data

YEAR=2022
DIR=/home/menno/pythonProjects/AdventOfCode/2022/input

for value in {0..5}
do
    aocd $value $YEAR > $DIR/inputday$value
done
echo All done
