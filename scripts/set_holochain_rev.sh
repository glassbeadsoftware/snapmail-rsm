#!/bin/sh

# Extract OLD REV from holochain_rev.txt
value=`cat holochain_rev.txt`
echo "OLD REV = '$value'"
echo "NEW REV = '$1'"

# Replace REV in sweettest/Cargo.toml
sed -i "s/$value/$1/g" sweettest/Cargo.toml

# Replace REV in zomes/snapmail/Cargo.toml
sed -i "s/$value/$1/g" zomes/snapmail/Cargo.toml

# Replace REV in holochain_rev.txt
sed -i "s/$value/$1/g" holochain_rev.txt
