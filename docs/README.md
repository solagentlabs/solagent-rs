cargo doc --open
cd docs && mdbook serve --open


cargo install cargo-release


运行格式化和检查：
bash

cargo fmt
cargo clippy --all-targets --all-features -- -D warnings

运行测试：
bash

solana-test-validator --quiet &
cargo test --all-features
cargo tarpaulin --out Html

生成文档：
bash

cargo doc --open
cd docs && mdbook serve --open

基准测试：
bash

cargo bench

发布：
bash

cargo release patch


