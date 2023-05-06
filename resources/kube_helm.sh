#!/bin/zsh
kubectl apply -f otel-collector-config.yaml
helm upgrade kafka bitnami/kafka -f values.yaml
