#upload.sh <remote_user>@<remote_ip>
cargo build --release
(cd admin_script; go build admin.go)
scp target/release/$(basename $(pwd)) admin_script/admin $1:~/sasbackend/
#scp admin_script/admin $1:~/sasbackend/admin
