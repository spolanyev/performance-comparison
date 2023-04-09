# About

I compared the performance of the web frameworks:
* Rust Actix Web;
* Rust Axum;
* Go Echo;
* Python Flask;
* Node.js Koa;
* Node.js Nest;
* PHP Slim.

Each of them has an endpoint that, when visited, requests weather data from the server and returns the value. For testing, I used the Caddy server located on the same Docker network and Grafana K6 load testing tool.

![Summary](https://github.com/spolanyev/performance-comparison/blob/main/k6-summary.png?raw=true)

![Consumption](https://github.com/spolanyev/performance-comparison/blob/main/consumption-during-test.png?raw=true)

![Containers](https://github.com/spolanyev/performance-comparison/blob/main/containers.png?raw=true)

![Top](https://github.com/spolanyev/performance-comparison/blob/main/top-3.png?raw=true)

# Contacts

If you are hiring, feel free to contact me at [spolanyev@gmail.com](mailto:spolanyev@gmail.com?subject=Vacancy).
