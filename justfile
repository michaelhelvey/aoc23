day num:
  cargo run --release --bin day_{{num}}

build_release:
 cargo build --release

bench num: build_release
  hyperfine --warmup 5 --shell=none ./target/release/day_{{num}}
