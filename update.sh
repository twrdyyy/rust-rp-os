#!/usr/bin/env bash

echo "Updating kernel to new version"
make
cp kernel8.img ./demo_payload_rpi3.imb
echo "Successfully updated kernel!"
echo "Cleaning up"
rm kernel8.img