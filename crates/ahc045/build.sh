#!/bin/bash

cargo build --release --target-dir ./target
cp ./target/release/ahc045 ./tools/
