#!/usr/bin/bash

cargo build --release || exit 1

rm -rf bin
mkdir bin || exit 2

while read -r file; do
    echo "   Installing bin/$file"
    ln target/release/pywrapper bin/$file || exit 3
done < install_names.txt

cargo clean --release
