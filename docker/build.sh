#!/bin/bash

docker build -t achillesss/btrader:latest -f ./docker/Dockerfile .
# docker buildx build --platform linux/amd64,linux/arm64 --compress -t achillesss/btrader:latest -f ./docker/Dockerfile .
