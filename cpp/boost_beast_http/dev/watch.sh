#!/bin/bash

WATCH_DIR=./src
TMP_DIR=./tmp

if [ ! -d $TMP_DIR ]; then
  mkdir $TMP_DIR
fi

scripts/build.sh

while inotifywait -r $WATCH_DIR -e create,delete,modify; do {
  echo "[Dev] ---- Starting Build"
  scripts/build.sh
  echo "[Dev] ---- Finished Build"
}; done
