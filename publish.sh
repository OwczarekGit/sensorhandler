#!/bin/bash

arch="x86_64-unknown-linux-gnu"

cargo build --release --target $arch
cp "./target/$arch/release/sensorhandler" "./sensorhandler-linux-x64"
