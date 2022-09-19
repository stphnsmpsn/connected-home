# Tiltfile
update_settings(max_parallel_updates=8)

###################################################################################################
#
###################################################################################################

#local_resource(
#    'cargo-build',
#    'make cargo-build',
#    labels=['Build']
#)

local_resource(
    'make-builder',
    'docker build -f dockerfiles/Build . -t rustbuilder',
    labels=['Build']
)

local_resource(
    'docker-build',
    'docker run -u $(id -u ${USER}):$(id -g ${USER}) --mount=target=/app,type=bind,source=$PWD rustbuilder',
    labels=['Build'],
    allow_parallel = True
)

###################################################################################################
#
###################################################################################################

local_resource(
    'update-helm',
    'helm repo add bitnami https://charts.bitnami.com/bitnami \
        && helm repo add hashicorp https://helm.releases.hashicorp.com \
#        && helm repo add redpanda https://charts.vectorized.io \
#        && helm repo add jetstack https://charts.jetstack.io \
        && for d in helm/*; do [[ -f "$d"/Chart.yaml ]] && helm dependency update $d; done',
    #auto_init=False,
    #trigger_mode=TRIGGER_MODE_MANUAL,
    allow_parallel = True,
    labels=['Build']
)

###################################################################################################
#                                       API GATEWAY
###################################################################################################

local_resource(
    'pack-api-gateway',
    'docker build -t api-gateway --build-arg binary_location=target/docker/debug/api-gateway --build-arg binary_name=api-gateway . \
        && minikube image load api-gateway',
    resource_deps=[
        'docker-build'
    ],
    deps=['target/docker/debug/api-gateway'],
    labels=['Build'],
    allow_parallel = True
)
k8s_yaml(helm('./helm/api-gateway'))
k8s_resource(
    workload='api-gateway',
    resource_deps=[
        'pack-api-gateway',
        'update-helm',
    ],
    port_forwards=[
        port_forward(8082, 8082, name='API Gateway'),
    ],
    labels=['Core_Services']
)

###################################################################################################
#                                       User Service
###################################################################################################

local_resource(
    'pack-user-service',
    'docker build -t user-service --build-arg binary_location=target/docker/debug/user-service --build-arg binary_name=user-service . \
        && minikube image load user-service',
    resource_deps=[
        'docker-build'
    ],
    deps=['target/docker/debug/user-service'],
    labels=['Build'],
    allow_parallel = True
)
k8s_yaml(helm('./helm/user-service'))
k8s_resource(
    workload='user-service',
    resource_deps=[
        'pack-user-service',
        'update-helm',
    ],
    labels=['Core_Services']
)

###################################################################################################
#                                       Consumer
###################################################################################################

local_resource(
    'pack-consumer',
    'docker build -t consumer --build-arg binary_location=target/docker/debug/consumer --build-arg binary_name=consumer . \
        && minikube image load consumer',
    resource_deps=[
        'docker-build'
    ],
    deps=['target/docker/debug/consumer'],
    labels=['Build'],
    allow_parallel = True
)
k8s_yaml(helm('./helm/consumer'))
k8s_resource(
    workload='consumer',
    resource_deps=[
        'pack-consumer',
        'update-helm',
        'rabbitmq',
    ],
    labels=['Core_Services']
)

###################################################################################################
#                                       Producer
###################################################################################################

local_resource(
    'pack-producer',
    'docker build -t producer --build-arg binary_location=target/docker/debug/producer --build-arg binary_name=producer . \
        && minikube image load producer',
    resource_deps=[
        'docker-build',
        'rabbitmq'
    ],
    deps=['target/docker/debug/producer'],
    labels=['Build'],
    allow_parallel = True
)
k8s_yaml(helm('./helm/producer'))
k8s_resource(
    workload='producer',
    resource_deps=[
        'pack-producer',
        'update-helm',
        'rabbitmq',
    ],
    labels=['Core_Services']
)

###################################################################################################
#                                       PostgreSQL
###################################################################################################

k8s_yaml(helm(
    './helm/postgresql',
    name='postgresql',
    values=[
        'helm/postgresql/values.yaml'
    ]
))
k8s_resource(
    workload='postgresql',
    resource_deps=[
#        'update-helm',
    ],
    port_forwards=[
      port_forward(5432, 5432, name='PostgreSQL'),
    ],
    labels=['PostgreSQL']
)

###################################################################################################
#                                       RaabitMQ
###################################################################################################

k8s_yaml(helm(
    './helm/rabbitmq',
    name='rabbitmq',
    values=[
        'helm/rabbitmq/values.yaml'
    ]
))
k8s_resource(
    workload='rabbitmq',
    resource_deps=[
        'update-helm',
    ],
    port_forwards=[
      port_forward(5672, 5672, name='Rabbit MQ'),
      port_forward(15672, 15672, name='Rabbit MQ Console'),
    ],
    labels=['RabbitMQ']
)

###################################################################################################
#                                       Vault
###################################################################################################

#k8s_yaml(helm(
#    './helm/vault',
#    name='vault',
#    values=[
#        'helm/vault/values.yaml'
#    ]
#))
#k8s_resource(
#    workload='vault',
#    resource_deps=[
#        'update-helm',
#    ],
#    port_forwards=[
#      port_forward(8200, 8200, name='Vault Port'),
#    ],
#    labels=['Vault']
#)

###################################################################################################
#                                       Frontend
###################################################################################################

local_resource(
    'build-frontend',
    'rm -rf frontend/build \
     	&& npm --prefix frontend install \
     	&& npm --prefix frontend run build \
        && eval $(minikube docker-env) \
        && docker build -f frontend/Dockerfile -t frontend:latest ./frontend',
    labels=['Build'],
    allow_parallel = True
)
k8s_yaml(helm(
    './helm/frontend',
    name='frontend',
    values=[
        'helm/frontend/values.yaml'
    ]
))
k8s_resource(
    workload='frontend',
    resource_deps=[
        'build-frontend',
    ],
    port_forwards=[
      port_forward(8000, 80, name='Frontend'),
    ],
    labels=['Frontend']
)

###################################################################################################
#                                       Swagger
###################################################################################################

local_resource(
    'update-swagger-doc',
    # This is hacky... try to do something better
    'if [ ! -z "$(kubectl get pods | grep swagger)" ]; then kubectl delete configmap vol-cfg-swaggerdoc && kubectl create configmap vol-cfg-swaggerdoc --from-file=helm/swagger/spec/swagger.yaml && kubectl delete pod $(kubectl get pods | grep swagger | cut -d " " -f 1); fi',
    deps=[
        './helm/swagger/spec/swagger.yaml',
        './crates/api-gateway/doc/swagger.yaml'
    ],
    labels=['Docs'],
)

k8s_yaml(helm(
    './helm/swagger',
    name='swagger',
    values=[
        'helm/swagger/values.yaml'
    ],
))
k8s_resource(
    workload='swagger',
    port_forwards=[
        port_forward(8081, 8080, name='Swagger'),
    ],
    resource_deps=[
        'update-swagger-doc',
    ],
    labels=['Docs'],
)

###################################################################################################
#                                       Redpanda
###################################################################################################

#k8s_yaml(helm(
#    './helm/redpanda',
#    name='redpanda',
#    values=[
#        'helm/redpanda/values.yaml'
#    ]
#))
#k8s_resource(
#    workload='redpanda-redpanda-operator',
#    labels=['Redpanda']
#)

###################################################################################################
#                                            END
###################################################################################################