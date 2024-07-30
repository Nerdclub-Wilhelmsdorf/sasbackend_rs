#upload.sh <remote_user>@<remote_ip>
cargo build --release
scp -r target/release/$(basename $(pwd)) admin_script/ $1:~/sasbackend/
#scp admin_script/admin $1:~/sasbackend/admin
