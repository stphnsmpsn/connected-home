# Tiltfile
load('ext://helm_remote', 'helm_remote')

local_resource('cargo build', 'make cargo-build', trigger_mode=TRIGGER_MODE_MANUAL)

local_resource(
    'pack api gateway',
    'make pack-api-gateway',
    trigger_mode=TRIGGER_MODE_AUTO,
    resource_deps=[
       'cargo build',
    ],
)

local_resource(
    'pack consumer',
    'make pack-consumer',
    trigger_mode=TRIGGER_MODE_AUTO,
    resource_deps=[
       'cargo build',
    ],
)

local_resource(
    'pack producer',
    'make pack-producer',
    trigger_mode=TRIGGER_MODE_AUTO,
    resource_deps=[
       'cargo build',
    ],
)

k8s_yaml(helm('./helm/api-gateway'))
k8s_resource(
   workload='api-gateway',
   resource_deps=[
   'cargo build',
   'pack api gateway',
   ],
   port_forwards=[
      port_forward(8082, 8082, name='API Gateway'),
   ]
)

k8s_yaml(helm('./helm/producer'))
k8s_resource(
    workload='producer',
    resource_deps=[
        'cargo build',
        'pack producer',
    ],
)

k8s_yaml(helm('./helm/consumer'))
k8s_resource(
    workload='consumer',
    resource_deps=[
        'cargo build',
        'pack consumer',
    ],
)

helm_remote(
    'mariadb',
    repo_name='bitnami',
    repo_url='https://charts.bitnami.com/bitnami',
    version='9.5.1',
)

helm_remote(
    'rabbitmq',
    repo_name='bitnami',
    repo_url='https://charts.bitnami.com/bitnami',
    version='8.22.0',
)