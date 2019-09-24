#!/bin/bash

mypod=$(kubectl get pods -l app=rustapp --output=jsonpath={.items..metadata.name})
kubectl port-forward $mypod 8080:8080