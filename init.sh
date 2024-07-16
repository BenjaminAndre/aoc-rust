if [[ -z "${AOC_SESSION-""}" ]]; then
  echo "\$AOC_SESSION not set"
  exit 1
fi

TMPFILE=$(mktemp)
trap 'rm -f "$TMPFILE"' EXIT

curl "https://adventofcode.com/$1/day/${2#0}/input" --silent --fail --cookie "session=$AOC_SESSION" > $TMPFILE 


mkdir -p "./$1";
cp -r "day_xx" "./$1/day_$2";
sed -i "s/day_xx/day_$2/g" "./$1/day_$2/Cargo.toml"
mv "$TMPFILE" "$1/day_$2/input.txt"
