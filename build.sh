#!/bin/bash

set -e

wasm-pack build --target web --release
npx webpack --config webpack.config.js --mode production
