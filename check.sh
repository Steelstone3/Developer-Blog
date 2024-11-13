cd shared

cargo update
cargo clippy

cd ..
cd client_web

cargo update
cargo clippy

cd ..
cd server

cargo update
cargo clippy