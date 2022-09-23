# Tiltfile
update_settings(max_parallel_updates=8)

###################################################################################################
#
###################################################################################################

local_resource(
    'docker-build-and-pack',
    'docker run -u $(id -u ${USER}):$(id -g ${USER}) --mount=target=/app,type=bind,source=$PWD rustbuilder \
        && eval $(minikube docker-env) \
        && docker build -t core-service . --no-cache \
        && minikube image tag core-service connectedhome/api-gateway:latest \
        && minikube image tag core-service connectedhome/user-service:latest \
        && minikube image tag core-service connectedhome/consumer:latest \
        && minikube image tag core-service connectedhome/producer:latest',
    labels=['Build'],
    allow_parallel = True
)

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

k8s_yaml(helm('./helm/api-gateway'))
k8s_resource(
    workload='api-gateway',
    resource_deps=[
        'docker-build-and-pack',
        'update-helm',
        'jaeger',
    ],
    port_forwards=[
        port_forward(8082, 8082, name='API Gateway'),
    ],
    labels=['Core_Services']
)

###################################################################################################
#                                       User Service
###################################################################################################

k8s_yaml(helm('./helm/user-service'))
k8s_resource(
    workload='user-service',
    resource_deps=[
        'docker-build-and-pack',
        'update-helm',
        'jaeger',
    ],
    labels=['Core_Services']
)

###################################################################################################
#                                       Consumer
###################################################################################################

k8s_yaml(helm('./helm/consumer'))
k8s_resource(
    workload='consumer',
    resource_deps=[
        'docker-build-and-pack',
        'update-helm',
        'jaeger',
        'rabbitmq'
    ],
    labels=['Core_Services']
)

###################################################################################################
#                                       Producer
###################################################################################################

k8s_yaml(helm('./helm/producer'))
k8s_resource(
    workload='producer',
    resource_deps=[
        'docker-build-and-pack',
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

local_resource(
    'make-migrate-db',
    'cd crates/user-service \
        && DATABASE_URL=postgres://postgres:password@localhost/postgres diesel migration run',
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
#                                       Jaeger
###################################################################################################

k8s_yaml(helm(
    './helm/jaeger',
    name='jaeger',
    values=[
        'helm/jaeger/values.yaml'
    ]
))
k8s_resource(
    workload='jaeger',
    port_forwards=[
        port_forward(16686, 16686, name='Dashboard'),
    ],
    labels=['Observability']
)

###################################################################################################
#                                            END
###################################################################################################