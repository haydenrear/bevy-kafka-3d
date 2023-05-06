#!/bin/zsh
docker login
docker build . -t haydenrear/bitnami-kafka-telemetry
docker push haydenrear/bitnami-kafka-telemetry