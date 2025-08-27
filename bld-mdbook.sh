#!/usr/bin/bash

#
# Remove all unwanted 'html' files
rm -r ./guide/src/*.html
find ./guide/src/guide/ -depth -name "*.html" -delete

# Rebuild mdbook guide
mdbook build guide
sed -i 's|<a href="api/flogging/index.html">|<a href="api/flogging/index.html" title="Opens in new tab" target="_blank">|' docs/toc.js
sed -i 's|<a href="coverage/index.html">|<a href="coverage/index.html" title="Opens in new tab" target="_blank">|' docs/toc.js
