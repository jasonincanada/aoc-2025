#!/bin/bash
#
# get your session ID from Chrome's devtools and replace it below. then use like:
#
# $ ./pull.sh > input.txt

SESSION=53616c7465645f5f48f654d6c0f704c24c29c73b7842afdad8650a1b31944ddcafc1babda7f61629abf444a56687d758a6a94cc03c51e5d4bbbdf019

DAY=$(date +%-d)
YEAR=$(date +%Y)

URL=https://adventofcode.com/$YEAR/day/$DAY/input

curl --cookie "session=$SESSION" \
     -H "User-Agent: custom bash script with curl, by github.com/jasonincanada" \
     $URL
