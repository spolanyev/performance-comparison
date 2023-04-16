<?php

//@author Stanislav Polaniev <spolanyev@gmail.com>
use App\ExportedWeather;
use App\ImportedWeather;
use Psr\Http\Client\ClientInterface;
use Psr\Http\Message\RequestFactoryInterface;
use Psr\Http\Message\ResponseInterface;
use Psr\Http\Message\ServerRequestInterface;
use Slim\App;

return function (App $app) {
    $app->get(
        '/weather/{city}',
        function (ServerRequestInterface $serverRequest, ResponseInterface $serverResponse, $serverArguments) {
            $apiKey = '';
            $apiUrl = sprintf(
                "http://caddy/data/2.5/weather?q=%s&appid=%s&units=metric",
                $serverArguments['city'],
                $apiKey
            );
            $apiRequest = $this->get(RequestFactoryInterface::class)->createRequest('GET', $apiUrl);
            $apiPage = $this->get(ClientInterface::class)->sendRequest($apiRequest);
            $apiData = json_decode((string)$apiPage->getBody(), true);

            if (isset($apiData['main']['temp'], $apiData['main']['humidity'], $apiData['wind']['speed'])) {
                $importedWeather = new ImportedWeather(
                    $apiData['main']['temp'],
                    $apiData['main']['humidity'],
                    $apiData['wind']['speed']
                );

                $exportedWeather = new ExportedWeather(
                    $serverArguments['city'],
                    $importedWeather->temperature,
                    $importedWeather->humidity,
                    $importedWeather->wind
                );

                $serverResponse->getBody()->write(
                    json_encode($exportedWeather)
                );
                return $serverResponse->withHeader('Content-Type', 'application/json');
            } else {
                $serverResponse->getBody()->write(json_encode(['message' => 'Wrong Weather API response received']));
                return $serverResponse->withHeader('Content-Type', 'application/json')->withStatus(500);
            }
        }
    );
};
