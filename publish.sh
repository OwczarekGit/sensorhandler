#!/bin/bash

arch_linux="x86_64-unknown-linux-gnu"
arch_windows="x86_64-pc-windows-gnu"

cargo build --release --target $arch_linux
cargo build --release --target $arch_windows
cp "./target/$arch_linux/release/sensorhandler" "./sensorhandler-linux-x64"
cp "./target/$arch_windows/release/sensorhandler.exe" "./sensorhandler-windows-x64.exe"
