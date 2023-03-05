set "RUSTFLAGS=-D warnings"
set "RUSTFMT_CI=1"

:: Print version information
rustc -Vv || exit /b 1
cargo -V || exit /b 1

:: Build and test main crate
cargo build --release || exit /b 1
dir || exit /b 1
tree || exit /b 1
ls /d/a/breed-enter-rust/breed-enter-rust || exit /b 1