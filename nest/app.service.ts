//@author Stanislav Polaniev <spolanyev@gmail.com>
import { Injectable } from '@nestjs/common';
import { HttpService } from '@nestjs/axios';

@Injectable()
export class AppService {
  constructor(private readonly httpService: HttpService) {}

  async getWeather(city: string) {
    const apiKey = '';
    const apiURL = `http://caddy/data/2.5/weather?q=${city}&appid=${apiKey}&units=metric`;
    const apiPage = await this.httpService.axiosRef.get(apiURL);
    const apiData = apiPage.data;

    if (
      'main' in apiData &&
      'temp' in apiData.main &&
      'humidity' in apiData.main &&
      'wind' in apiData &&
      'speed' in apiData.wind
    ) {
      const importedWeather = new ImportedWeather(
        apiData.main.temp,
        apiData.main.humidity,
        apiData.wind.speed,
      );

      return new ExportedWeather(
        city,
        importedWeather.temperature,
        importedWeather.humidity,
        importedWeather.wind,
      );
    } else {
      throw new Error('Wrong Weather API response received');
    }
  }
}

export class ImportedWeather {
  constructor(
    public temperature: number,
    public humidity: number,
    public wind: number,
  ) {}
}

export class ExportedWeather {
  constructor(
    public city: string,
    public temperature: number,
    public humidity: number,
    public wind: number,
  ) {}
}
