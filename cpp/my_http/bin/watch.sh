#!/bin/bash

# Define the directory to watch
WATCH_DIR="$HOME/workspace/personal/cpp/my_http/src"

# Define the build and restart script
BUILD_SCRIPT="$HOME/workspace/personal/cpp/my_http/bin/build_and_restart.sh"

# Run an initial build and start the server
$BUILD_SCRIPT

# Use inotifywait to watch for changes in the directory
while true; do
    inotifywait -e modify,create,delete -r $WATCH_DIR
    $BUILD_SCRIPT
done
