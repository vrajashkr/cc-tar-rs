#!/bin/sh
PROJECT_ROOT="$( git rev-parse --show-toplevel )"
BINARY="${PROJECT_ROOT}/target/release/cc-tar-rs"

echo "comparing archive listing performance"
echo "GNU tar"
/usr/bin/time --verbose tar -t -f ./bench-data-archive.tar
echo ""
echo "cc-tar-rs"
/usr/bin/time --verbose ${BINARY} -t -f ./bench-data-archive.tar
