#!/usr/bin/env bash
# Vendor pinned sources needed to build the per-version JSON-RPC schemas.
# - alloy crates: downloaded from crates.io (.crate archives) for the exact
#   versions pinned by each Reth tag.
# - Reth rpc crates: sparse archives produced from the git tags (see fetch_reth).
#
# Idempotent. Run from repo root: tools/schema/fetch_sources.sh
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
VENDOR="$ROOT/vendor/crates"
mkdir -p "$VENDOR"

# crate:version pairs (alloy versions pinned by Reth v2.3.0 / v2.7.0)
CRATES=(
  "alloy-primitives:1.6.0"
  "alloy-sol-types:1.6.0"
  "alloy-consensus:2.0.5"
  "alloy-consensus:2.5.0"
  "alloy-eips:2.0.5"
  "alloy-eips:2.5.0"
  "alloy-serde:2.0.5"
  "alloy-serde:2.5.0"
  "alloy-network-primitives:2.0.5"
  "alloy-network-primitives:2.5.0"
  "alloy-consensus-any:2.0.5"
  "alloy-consensus-any:2.5.0"
  "alloy-rpc-types:2.0.5"
  "alloy-rpc-types:2.5.0"
  "alloy-rpc-types-eth:2.0.5"
  "alloy-rpc-types-eth:2.5.0"
  "alloy-rpc-types-any:2.0.5"
  "alloy-rpc-types-any:2.5.0"
  "alloy-rpc-types-txpool:2.0.5"
  "alloy-rpc-types-txpool:2.5.0"
  "alloy-rpc-types-mev:2.0.5"
  "alloy-rpc-types-mev:2.5.0"
  "alloy-dyn-abi:1.6.0"
)

for pair in "${CRATES[@]}"; do
  name="${pair%%:*}"; ver="${pair##*:}"
  crate="$name-$ver.crate"
  if [[ -f "$VENDOR/$crate" ]]; then
    echo "cached: $crate"
  else
    echo "fetch:  $crate"
    curl -fsSL -o "$VENDOR/$crate" "https://static.crates.io/crates/$name/$crate"
  fi
  if [[ ! -d "$VENDOR/$name-$ver" ]]; then
    tar xzf "$VENDOR/$crate" -C "$VENDOR"
  fi
done

# checksums
( cd "$VENDOR" && sha256sum ./*.crate > SHA256SUMS )
echo "wrote $VENDOR/SHA256SUMS"
