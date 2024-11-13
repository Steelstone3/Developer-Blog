cd business

cargo update
cargo clippy
cargo fmt

cd ..
cd client_web

cargo update
cargo clippy
cargo fmt

cd ..
cd server

cargo update
cargo clippy
cargo fmt