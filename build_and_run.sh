#!/bin/sh

cd kernel
cargo build

# Check if the build was successful
if [ $? -ne 0 ]; then
    echo "Kernel build failed."
    exit 1
fi

cd ../bootloader
./run.sh

if [ $? -ne 0 ]; then
    echo "Bootloader run failed."
    exit 1
fi

echo "Build and run completed successfully."
