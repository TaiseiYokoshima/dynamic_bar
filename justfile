check:
   @RUSTFLAGS="-Awarnings" cargo check --quiet 

run *args:
   @RUSTFLAGS="-Awarnings" cargo run -- {{args}}


release-run *args:
   @RUSTFLAGS="-Awarnings" cargo run --release -- {{args}}

