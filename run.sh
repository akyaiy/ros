#!/usr/bin/env bash

IMAGE_PATH="/home/alex/Dev/rsmtg/ros/target/x86_64-ros/debug/bootimage-ros.bin"

if [[ ! -f "$IMAGE_PATH" ]]; then
	echo "Error: Boot image not found at $IMAGE_PATH"
	exit 1
fi

com="cargo build"
echo "Running: $com"
$com

com="qemu-system-x86_64 -serial stdio -drive format=raw,file="$IMAGE_PATH""
echo "Running: $com"
$com