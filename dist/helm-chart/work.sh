
#!/bin/bash
kubectl delete namespace dev
kubectl create namespace dev
helm uninstall pd
helm package pd
helm install pd ./pd-0.1.0.tgz 
helm uninstall range-server
helm package range-server
helm install range-server ./range-server-0.1.0.tgz
kubectl get pod -n dev







