#!/bin/bash

# Wait for Vault pod to be running
#while [[ $(kubectl get pods -l app.kubernetes.io/name=vault -o 'jsonpath={..status.phase}') != "Running" ]]; do
while [[ $(kubectl get pod vault-server-0 -o 'jsonpath={..status.phase}') != "Running" ]]; do
    echo "Waiting for Vault pod to be running..."
    sleep 5
done

# Get Vault Pod name
#VAULT_POD_NAME=$(kubectl get pods -l app.kubernetes.io/name=vault -o jsonpath="{.items[0].metadata.name}")
VAULT_POD_NAME=$(kubectl get pods -l app.kubernetes.io/name=vault -o name | grep 'vault-server' | sed 's/pod\///')

# Initialize Vault
JSON=$(kubectl exec -i $VAULT_POD_NAME -- vault operator init -format=json)

if [ $? -ne 0 ]; then
    echo "Vault has already been initialized. Unseal keys and root token can be found in .secrets/vault.json."
    cat .secrets/vault.json
else
    echo $JSON | jq '.' > .secrets/vault.json
    echo $JSON
fi

# Unsealing Vault
cat .secrets/vault.json | jq -r ".unseal_keys_b64[]" | while read k; do
    echo "Unsealing Vault..."
    kubectl exec $VAULT_POD_NAME -- vault operator unseal $k
    echo "Unsealed"
done

# Check if vault is unsealed before attempting login
while [[ $(kubectl exec -i $VAULT_POD_NAME -- vault status -format=json | jq -r '.sealed') == "true" ]]; do
    echo "Waiting for vault to be fully unsealed..."
    sleep 5
done

# Logging in to Vault
ROOT_TOKEN=$(jq -r ".root_token" .secrets/vault.json)
kubectl exec -i $VAULT_POD_NAME -- vault login $ROOT_TOKEN

echo "Vault has been initialized and unsealed. Unseal keys and root token can be found in .secrets/vault.json."