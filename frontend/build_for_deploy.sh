#!/bin/bash
npm run build -- --outDir ../ansible/to_deploy/frontend --emptyOutDir
find ../ansible/to_deploy/frontend/ \( -name '*.js' -o -name '*.css' -o -name '*.html' \) -size +1400c -exec sh -c 'brotli --keep --best "$1"; gzip --keep --best "$1"' shell {} \;
