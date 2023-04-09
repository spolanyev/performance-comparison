#@author Stanislav Polaniev <spolanyev@gmail.com>
import requests
from flask import Flask, jsonify, Response, make_response

app = Flask(__name__)


@app.route('/weather/<city>')
def get_weather(city: str) -> Response:
    api_key: str = ''
    api_url: str = f'http://caddy/data/2.5/weather?q={city}&appid={api_key}&units=metric'
    api_page: requests.Response = requests.get(api_url)
    api_data: dict = api_page.json()

    if 'main' in api_data and 'temp' in api_data['main'] and 'humidity' in api_data['main'] and 'wind' in api_data and 'speed' in api_data['wind']:
        imported_weather = ImportedWeather(temperature=api_data['main']['temp'], humidity=api_data['main']['humidity'],
                                           wind=api_data['wind']['speed'])
        exported_weather: ExportedWeather = ExportedWeather(city=city, temperature=imported_weather.temperature,
                                                            humidity=imported_weather.humidity,
                                                            wind=imported_weather.wind)
    else:
        return make_response(jsonify({'message': 'Wrong Weather API response received'}), 500)

    return jsonify(exported_weather.__dict__)


class ImportedWeather:
    def __init__(self, temperature: float, humidity: int, wind: float) -> None:
        self.temperature = temperature
        self.humidity = humidity
        self.wind = wind


class ExportedWeather:
    def __init__(self, city: str, temperature: float, humidity: int, wind: float) -> None:
        self.city = city
        self.temperature = temperature
        self.humidity = humidity
        self.wind = wind


if __name__ == '__main__':
    app.run()
