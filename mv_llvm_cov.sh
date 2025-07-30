#!/usr/bin/bash
#
# File Name:    mv_llvm_cov.sh
# Project Name: flogging
#
# Copyright (C) 2025 Bradley Willcott
#
# SPDX-License-Identifier: GPL-3.0-or-later
#
# This library (crate) is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This library (crate) is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this library (crate).  If not, see <https:#www.gnu.org/licenses/>.
#

mv target/llvm-cov/html/coverage/`pwd` target/llvm-cov/html/coverage/
rm -r target/llvm-cov/html/coverage/home
find target/llvm-cov/html/ -type f -name "*.html" -exec sed -i 's|/home/bwillcott/Rust-Projects||g' {} \;
find target/llvm-cov/html/ -type f -name "*.html" -exec sed -i "s|'../../../|'|g" {} \;
rm -r docs/*
mv target/llvm-cov/html/* docs/
