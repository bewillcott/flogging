#!/usr/bin/bash

#
# Remove all unwanted 'html' files
rm guide/src/guide/**.html
rm guide/src/guide/**/**.html

# Rebuild mdbook guide
mdbook build guide
