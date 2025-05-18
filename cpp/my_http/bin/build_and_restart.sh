#!/bin/bash

# Define the directory containing your project
PROJECT_DIR="$HOME/workspace/personal/cpp/my_http/"

# Define the server executable
SERVER_EXECUTABLE="$PROJECT_DIR/build/MyHttp"

# Define the process ID file
PID_FILE="$PROJECT_DIR/server.pid"

# Function to stop the server
stop_server() {
  echo "STOPPING"
    if [ -f $PID_FILE ]; then
        PID=$(cat $PID_FILE)
        if kill -0 $PID > /dev/null 2>&1; then
            echo "Stopping server (PID $PID)..."
            kill $PID
            while kill -0 $PID > /dev/null 2>&1; do
                sleep 1
            done
            echo "Server stopped."
        fi
        rm -f $PID_FILE
    fi
}

# Function to start the server
start_server() {
    echo "Starting server..."
    $SERVER_EXECUTABLE &
    echo $! > $PID_FILE
    echo "Server started (PID $(cat $PID_FILE))."
}

# Stop the server if it is running
stop_server

# Change to the project directory
cd $PROJECT_DIR

# Run CMake build commands
echo "Building project..."
mkdir -p build
cd build
cmake ..
make

# Start the server
start_server
