#!/bin/bash

SCRIPT_DIR="$(dirname "$0")"

deno run --allow-read --allow-write "${SCRIPT_DIR}/scripts/update_mods.ts"

cargo fmt
