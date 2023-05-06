#!/usr/bin/env bash
helm repo add prometheus-community https://prometheus-community.github.io/helm-charts
helm repo add grafana https://grafana.github.io/helm-charts
helm repo update
helm upgrade prometheus prometheus-community/prometheus -f prometheus-values.yaml
helm upgrade grafana grafana/grafana -f grafana-values.yaml
helm install grafana grafana/grafana -f grafana-values.yaml && helm install prometheus prometheus-community/prometheus -f prometheus-values.yaml
helm uninstall grafana && helm uninstall prometheus && helm install grafana grafana/grafana -f grafana-values.yaml && helm install prometheus prometheus-community/prometheus -f prometheus-values.yaml
