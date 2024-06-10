#upload.sh <remote_user>@<remote_ip>
cargo build --release
scp target/release/$(basename $(pwd)) $1:~/sasbackend/$(basename $(pwd))
(cd admin_script; go build admin.go)
scp admin_script/admin $1:~/sasbackend/admin
