#!/bin/bash

rm -rf build
mkdir build

cmake .
cmake --build .
