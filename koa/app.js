//@author Stanislav Polaniev <spolanyev@gmail.com>
'use strict';

import Koa from 'koa';
import Router from 'koa-router';
import request from 'request-promise';

const router = new Router();
router.get('/weather/:city', async (context) => {
    const apiKey = '';
    const apiUrl = `http://caddy/data/2.5/weather?q=${context.params.city}&appid=${apiKey}&units=metric`;
    const apiPage = await request(apiUrl);
    const apiData = JSON.parse(apiPage);

    if (undefined !== apiData.main?.temp && undefined !== apiData.main?.humidity && undefined !== apiData.wind?.speed) {
        const importedWeather = new ImportedWeather(apiData.main.temp, apiData.main.humidity, apiData.wind.speed);
        context.body = new ExportedWeather(context.params.city, importedWeather.temperature, importedWeather.humidity, importedWeather.wind);
    } else {
        context.status = 500;
        context.body = {'message': 'Wrong Weather API response received'};
    }
});

const app = new Koa();
app.use(router.routes());
app.listen(80, () => {
    console.log('Node.js server started...');
});

class ImportedWeather {
    constructor(temperature, humidity, wind) {
        this.temperature = temperature;
        this.humidity = humidity;
        this.wind = wind;
    }
}

class ExportedWeather {
    constructor(city, temperature, humidity, wind) {
        this.city = city;
        this.temperature = temperature;
        this.humidity = humidity;
        this.wind = wind;
    }
}
