# Tiltfile

###################################################################################################
#
###################################################################################################
local_resource(
    'cargo-build',
    'make cargo-build',
    labels=['Build']
)

###################################################################################################
#
###################################################################################################
local_resource(
    'update-helm',
    'helm repo add bitnami https://charts.bitnami.com/bitnami \
        && helm repo add hashicorp https://helm.releases.hashicorp.com \
        && for d in helm/*; do [[ -f "$d"/Chart.yaml ]] && helm dependency update $d; done',
    auto_init=False,
    trigger_mode=TRIGGER_MODE_MANUAL,
    labels=['Build']
)

###################################################################################################
#                                       API GATEWAY
###################################################################################################
local_resource(
    'pack-api-gateway',
    'eval $(minikube docker-env) \
        && docker build -t api-gateway --build-arg binary_location=target/release/api-gateway --build-arg binary_name=api-gateway .',
    resource_deps=[
       'cargo-build'
    ],
    labels=['Build']
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
    'eval $(minikube docker-env) \
        && docker build -t user-service --build-arg binary_location=target/release/user-service --build-arg binary_name=user-service .',
    resource_deps=[
       'cargo-build'
    ],
    labels=['Build']
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
    'eval $(minikube docker-env) \
            && docker build -t consumer --build-arg binary_location=target/release/consumer --build-arg binary_name=consumer .',
    resource_deps=[
       'cargo-build'
    ],
    labels=['Build']
)
k8s_yaml(helm('./helm/consumer'))
k8s_resource(
    workload='consumer',
    resource_deps=[
        'pack-consumer',
        'update-helm',
    ],
    labels=['Core_Services']
)

###################################################################################################
#                                       Producer
###################################################################################################
local_resource(
    'pack-producer',
    'eval $(minikube docker-env) \
            && docker build -t producer --build-arg binary_location=target/release/producer --build-arg binary_name=producer .',
    resource_deps=[
       'cargo-build'
    ],
    labels=['Build']
)
k8s_yaml(helm('./helm/producer'))
k8s_resource(
    workload='producer',
    resource_deps=[
        'pack-producer',
        'update-helm',
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
#                                       Frontend
###################################################################################################
local_resource(
    'build-frontend',
    'rm -rf frontend/build \
     	&& npm --prefix frontend install \
     	&& npm --prefix frontend run build \
        && eval $(minikube docker-env) \
        && docker build -f frontend/Dockerfile -t frontend:latest ./frontend',
    labels=['Build']
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
#                                            END
###################################################################################################