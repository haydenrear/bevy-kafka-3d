#!/usr/bin/env bash
# uncomment to add helm chart
#helm repo add bitnami https://charts.bitnami.com/bitnami
#helm repo update
helm uninstall kafka && helm install kafka bitnami/kafka -f values.yaml
kubectl delete configmap otel-collector-config && kubectl apply -f otel-collector-config.yaml && helm uninstall kafka && helm install kafka bitnami/kafka -f values.yaml

# Now you can connect on 127.0.0.1:9094 after using `minikube tunnel` using the LoadBalancer