<?php

//@author Stanislav Polaniev <spolanyev@gmail.com>
namespace App;

// phpcs:disable
final readonly class ExportedWeather
{
    public function __construct(
        public string $city,
        public float $temperature,
        public int $humidity,
        public float $wind
    ) {
    }
}
// phpcs:enable
