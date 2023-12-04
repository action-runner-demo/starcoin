```bash
curl -L https://git.io/get_helm.sh | bash -s -- --version v3.8.2
helm repo add jetstack https://charts.jetstack.io
helm repo update
kubectl apply -f https://github.com/cert-manager/cert-manager/releases/download/v1.10.0/cert-manager.crds.yaml
helm install \
  cert-manager jetstack/cert-manager \
  --namespace cert-manager \
  --create-namespace \
  --version v1.10.0
 
helm repo add actions-runner-controller https://actions-runner-controller.github.io/actions-runner-controller

kubectl create ns actions-runner-system

kubectl create secret generic controller-manager \
    -n actions-runner-system \
    --from-literal=github_token=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
    
helm upgrade --install --namespace actions-runner-system --create-namespace \
             --wait actions-runner-controller actions-runner-controller/actions-runner-controller
```