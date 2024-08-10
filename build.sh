cargo build --release --target x86_64-unknown-linux-musl
cargo build --release --target x86_64-pc-windows-gnu
rm -rf bin
mkdir bin
mkdir bin/linux
mkdir bin/windows
cp target/x86_64-unknown-linux-musl/release/bimble bin/linux/bimble
cp target/x86_64-pc-windows-gnu/release/bimble.exe bin/windows/bimble.exe
cp ./lb.bjb bin/linux
cp ./wb.bjb bin/windows

