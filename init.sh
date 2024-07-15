mkdir -p "./$1";
cp -r "day_xx" "./$1/day_$2";
sed -i "s/day_xx/day_$2/g" "./$1/day_$2/Cargo.toml"
