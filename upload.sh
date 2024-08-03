#upload.sh <remote_user>@<remote_ip>
cargo build --release
ssh $1 "killall $(basename $(pwd))"
scp -r target/x86_64-unknown-linux-musl/release/$(basename $(pwd)) admin_script/ $1:~/sasbackend/
ssh 
