# About

I compared the performance of these web frameworks:
* Go Echo;
* JavaScript Koa;
* PHP Slim.
* Python FastAPI;
* Python Flask;
* Rust Actix Web;
* Rust Axum;
* TypeScript Nest;

A web service with an endpoint is built on top of each framework. When the endpoint is visited, the service requests weather data from a weather server and returns the value.

For testing, I use a Caddy server on the same Docker network as a weather service.

I use Grafana K6 load testing tool.

To monitor CPU and memory usage of Docker containers, I use cAdvisor, Prometheus, and Grafana.

For testing on AWS, I use an R&D playground from the company [Wildix](https://www.wildix.com/).

#### 8 Frameworks

![Response Time](https://github.com/spolanyev/performance-comparison/blob/main/k6-summary.png?raw=true)

![CPU Consumption](https://github.com/spolanyev/performance-comparison/blob/main/grafana-docker-cpu.png?raw=true)

![Memory Consumption](https://github.com/spolanyev/performance-comparison/blob/main/grafana-docker-memory.png?raw=true)

![Containers](https://github.com/spolanyev/performance-comparison/blob/main/containers.png?raw=true)

#### Top 3 Frameworks

![Top 3 Response Time](https://github.com/spolanyev/performance-comparison/blob/main/k6-summary-top-3.png?raw=true)

![Top 3 CPU Consumption](https://github.com/spolanyev/performance-comparison/blob/main/grafana-docker-cpu-top-3.png?raw=true)

![Top 3 Memoru Consumption](https://github.com/spolanyev/performance-comparison/blob/main/grafana-docker-memory-top-3.png?raw=true)

#### AWS Lambda

![Lambda 128 K6 Total Duration](https://github.com/spolanyev/performance-comparison/blob/main/k6-lambda-128mb-total-duration.png?raw=true)

![Lambda 128 CloudWatch Metrics](https://github.com/spolanyev/performance-comparison/blob/main/cloudwatch-lambda-128mb-total-duration.png?raw=true)

#### Lambda Cold Start

![Lambda 128 K6 Cold Start](https://github.com/spolanyev/performance-comparison/blob/main/k6-lambda-128mb-cold-start.png?raw=true)

![Lambda 128 CloudWatch Cold Start](https://github.com/spolanyev/performance-comparison/blob/main/cloudwatch-lambda-128mb-cold-start.png?raw=true)

#### Lambda Warm Start

![Lambda 128 K6 Warm Start](https://github.com/spolanyev/performance-comparison/blob/main/k6-lambda-128mb-warm-start.png?raw=true)

![Lambda 128 CloudWatch Warm Start](https://github.com/spolanyev/performance-comparison/blob/main/cloudwatch-lambda-128mb-warm-start.png?raw=true)


#### Amazon ECS on Fargate

![Fargate ECS K6](https://github.com/spolanyev/performance-comparison/blob/main/k6-fargate-ecs-duration.png?raw=true)

![Fargate ECS CloudWatch](https://github.com/spolanyev/performance-comparison/blob/main/cloudwatch-fargate-ecs.png?raw=true)

# Contacts

If you are hiring, feel free to contact me at [spolanyev@gmail.com](mailto:spolanyev@gmail.com?subject=Web%20Frameworks).
