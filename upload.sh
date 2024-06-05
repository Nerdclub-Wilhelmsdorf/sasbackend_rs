#upload.sh <remote_user>@<remote_ip>
cargo build --release
scp target/release/$(basename $(pwd)) $1:~/sasbackend/$(basename $(pwd))
