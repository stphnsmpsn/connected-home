# Connected Home

[![Build Status](https://drone.stephensampson.dev/api/badges/stphnsmpsn/connected-home/status.svg?ref=refs/heads/devel)](https://drone.stephensampson.dev/stphnsmpsn/connected-home)

A distributed multi-service application written in Rust and deployed to Kubernetes. Useful for collecting, aggregating, 
and visualizing data from various sensors around the home. Disclaimer: There is little reason to be using Kubernetes 
other than the fact that I want to learn it (along with helm and Tilt) better. I suppose you could argue that the 
readiness / health probes are worth it, but I'm sure using Docker compose would have been easier and just as practical. 


## What does it do?

Not much yet... Currently, I am working on the design, creating the base structure, evaluating and learning tools / 
technologies to use in the project. 

## TODO

* [x] Set up persistent volume for MariaDB (consider PostgreSQL)
* [x] Create user registration / authentication mechanism using JWTs
* [ ] Implement permissions and groups 
  * [ ] Add these to the JWT 'claims'
* [ ] Evaluate DB options for logging sensor data (no SQL)  
* [ ] Create mqtt consumer that can deserialize incoming messages into a struct and serialize to DB
* [ ] Create Raspberry Pi (or similar) w/sensor and create base image + producer application
    * [ ] Create mock producer for testing locally and for integration testing
* [ ] Create useful READY/healthy probes for services
* [ ] Define pre-commit and pre-push hooks
* [ ] Create CI/CD pipeline    
* [x] gRPC for synchronous inter-service communication
  * [x] Proxy requests that hit API Gateway to the appropriate service rather than handling all requests in API Service
    * [x] Create user service for handling user registration, login, and profile retrieval 
      * [ ] Properly handle errors in user service (no unwraps)
      * [ ] Properly handle secrets in user service (hashicorp vault?)
      * [ ] Properly handle config in user service
* [ ] GraphQL API (maybe)
* [ ] Frontend for visualizing data
    * [ ] Define user/groups/permission model and create administrator frontend for provisioning (mobile app?)
    * [ ] Create a dashboard where users can view and filter data from their sensors
    * [ ] Prometheus & Grafana
* [ ] Create alarms/alerts and trigger user customizable actions (send an email / Slack message). 

## Requirements

* [rust](https://www.rust-lang.org/tools/install)
* [docker](https://www.docker.com/)
* [minikube](https://minikube.sigs.k8s.io/docs/start/)
* [tilt](https://tilt.dev/)


## Getting Started

1. `git clone`
2. `cd connected-home`
3. `make builder`
4. `tilt up`