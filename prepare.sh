#!/bin/bash
set -euo pipefail
SCRIPT_DIR=$(realpath "$(dirname "$0")")

if [[ $# != 2 ]]; then
  echo "Please provide a year and day number."
  echo "usage: $0 YEAR DAY"
  exit 1
fi

if [[ ! "$1" =~ ^(20(1[5-9]|[2-9][0-9]))$ ]]; then
  echo "Not a valid year : $1"
  exit 1
fi

if [[ ! "$2" =~ ^(0[1-9]|1[0-9]|2[0-5])$ ]]; then
  echo "Not a valid day: $2"
  exit 1
fi

if [[ -z "${AOC_SESSION-""}" ]]; then
  echo "\$AOC_SESSION not set"
  exit 1
fi

TMPFILE=$(mktemp)
trap 'rm -f "$TMPFILE"' EXIT

curl "https://adventofcode.com/$1/day/${2#0}/input"          \
  -s --fail --cookie "session=$AOC_SESSION"                    \
  -A "Bash script at $(git remote -v | awk 'NR==1{print $2}')" \
  | tee "$TMPFILE"

mkdir -p "$SCRIPT_DIR/inputs"
mv "$TMPFILE" "$SCRIPT_DIR/inputs/$1_$2.in"
mkdir -p "$SCRIPT_DIR/src/$1"
cp "$SCRIPT_DIR/src/template.rs" "$SCRIPT_DIR/src/$1/$2.rs"
sed -i "s/\$YEAR/$1/g" "$SCRIPT_DIR/src/$1/$2.rs"
sed -i "s/\$DAY/$2/g" "$SCRIPT_DIR/src/$1/$2.rs"
