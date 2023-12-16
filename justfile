day num:
  cargo run --release --bin day_{{num}}

build_release:
 cargo build --release

bench num: build_release
  hyperfine --warmup 5 --shell=none ./target/release/day_{{num}}

new day:
  cp ./templates/day_lib src/day{{day}}.rs
  cat ./templates/day_bin | sd "\{\{(.*)\}\}" "{{day}}" > ./src/bin/day_{{day}}.rs
  touch input/day_{{day}}.txt
  echo "pub mod day{{day}};" >> src/lib.rs
