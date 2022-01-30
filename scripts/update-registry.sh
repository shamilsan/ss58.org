#!/bin/sh

set -e

cd "$(dirname "$0")"/..
wget -O src/ss58-registry.json https://raw.githubusercontent.com/paritytech/ss58-registry/main/ss58-registry.json
