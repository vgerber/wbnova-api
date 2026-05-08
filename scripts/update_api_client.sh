#!/usr/bin/env bash
set -e
curl -o public.openapi.yaml https://portal.wandelbots.io/docs/api/v2/ui/public.openapi.yaml
opage -s public.openapi.yaml -o . -c config.json
mkdir -p src/v2
rm -rf src/v2/objects src/v2/paths
mv src/objects src/v2/objects
mv src/paths src/v2/paths

find src/v2 -type f -exec sed -i 's/crate::objects/crate::v2::objects/g' {} +

echo "pub mod v2;" > src/lib.rs

cargo fmt
cargo build