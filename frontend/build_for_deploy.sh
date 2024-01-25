#!/bin/bash
npm run build
mkdir -p ../ansible/to_deploy/frontend
rm -rf ../ansible/to_deploy/frontend/*
mv dist/* ../ansible/to_deploy/frontend
find ../ansible/to_deploy/frontend/ \( -name '*.js' -o -name '*.css' -o -name '*.html' \) -size +1400c -exec sh -c 'brotli --keep --best "$1"; gzip --keep --best "$1"' shell {} \;
