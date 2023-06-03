# About

I compared the performance of the web frameworks:
* Rust Actix Web;
* Rust Axum;
* Go Echo;
* Python FastAPI;
* Python Flask;
* JavaScript Koa;
* JavaScript Nest;
* PHP Slim.

Each web service has an endpoint. When the endpoint is visited, it requests weather data from the server and returns the value.

For testing, I used the Caddy server located on the same Docker network and Grafana K6 load testing tool.

To monitor CPU and memory usage of Docker containers for the top 3 frameworks, I used cAdvisor, Prometheus, and Grafana.

For the AWS test, I used the [Wildix](https://www.wildix.com/)'s R&D playground. The lambdas had 128 and 512 MB of memory.

![Response Time](https://github.com/spolanyev/performance-comparison/blob/main/k6-summary.png?raw=true)

![CPU Consumption](https://github.com/spolanyev/performance-comparison/blob/main/grafana-docker-cpu.png?raw=true)

![Memory Consumption](https://github.com/spolanyev/performance-comparison/blob/main/grafana-docker-memory.png?raw=true)

![Containers](https://github.com/spolanyev/performance-comparison/blob/main/containers.png?raw=true)

![Top 3 Response Time](https://github.com/spolanyev/performance-comparison/blob/main/k6-summary-top-3.png?raw=true)

![Top 3 CPU Consumption](https://github.com/spolanyev/performance-comparison/blob/main/grafana-docker-cpu-top-3.png?raw=true)

![Top 3 Memoru Consumption](https://github.com/spolanyev/performance-comparison/blob/main/grafana-docker-memory-top-3.png?raw=true)

![Lambda 128 Response Time](https://github.com/spolanyev/performance-comparison/blob/main/k6-lambda-128mb-summary.png?raw=true)

![Lambda 128 Metrics](https://github.com/spolanyev/performance-comparison/blob/main/cloudwatch-lambda-128mb.png?raw=true)

![Lambda 512 Response Time](https://github.com/spolanyev/performance-comparison/blob/main/k6-lambda-512mb-summary.png?raw=true)

![Lambda 512 Metrics](https://github.com/spolanyev/performance-comparison/blob/main/cloudwatch-lambda-512mb.png?raw=true)

![Lambda 128 Compiled Response Time](https://github.com/spolanyev/performance-comparison/blob/main/k6-lambda-128mb-compiled.png?raw=true)

![Lambda 128 Compiled Metrics](https://github.com/spolanyev/performance-comparison/blob/main/cloudwatch-lambda-128mb-compiled.png?raw=true)

# Contacts

If you are hiring, feel free to contact me at [spolanyev@gmail.com](mailto:spolanyev@gmail.com?subject=Vacancy).
