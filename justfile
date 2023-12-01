day num:
  cargo run --bin day_{{num}}

test num:
  cargo test --bin day_{{num}}

build_release:
 cargo build --release

bench num:
  hyperfine --warmup 5 ./target/release/day_{{num}}
