#!/bin/bash

cargo test
sed -i '1d' zomes/snapmail_model/bindings/*.ts
cat zomes/snapmail_model/bindings/*.ts > ./bindings/snapmail.bindings.ts