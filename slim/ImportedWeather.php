<?php

//@author Stanislav Polaniev <spolanyev@gmail.com>
namespace App;

// phpcs:disable
final readonly class ImportedWeather
{
    public function __construct(
        public float $temperature,
        public int $humidity,
        public float $wind
    ) {
    }
}
// phpcs:enable
