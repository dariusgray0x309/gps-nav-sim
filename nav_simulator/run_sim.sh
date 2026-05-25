#!/usr/bin/env bash

# If any command fails, the script immediately exits
set -e

cargo build

echo "Starting the Subscriber Application..."
target/debug/algo_node -- &

# Get the process ID of the last process (i.e., server)
ALGO_PID=$!

# give the subscriber time to start up
# ideally, this should use retry logic instead
sleep 2 

echo "Starting the Satellites"
target/debug/sat_node --id 1 --phase-angle 0.0 --stop-time 20.0 &
SAT1_PID=$!

target/debug/sat_node --id 2 --phase-angle 30.0 --stop-time 20.0 &
SAT2_PID=$!

target/debug/sat_node --id 3 --phase-angle 60.0 --stop-time 20.0 &
SAT3_PID=$!

echo "Starting the Vehicle"
target/debug/vehicle_node --velocity 35.0 --efficiency 7.0 --fuel 55.0 --dt 0.01 &
VEHICLE_PID=$!

# Kill the apps
trap "kill $ALGO_PID $SAT1_PID $SAT2_PID $SAT3_PID $VEHICLE_PID" EXIT

wait