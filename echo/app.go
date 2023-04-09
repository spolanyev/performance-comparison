//@author Stanislav Polaniev <spolanyev@gmail.com>

package main

import (
	"encoding/json"
	"github.com/labstack/echo/v4"
	"io"
	"net/http"
)

func main() {
	app := echo.New()
	app.GET("/weather/:city", func(context echo.Context) error {
		apiKey := ""
		apiURL := "http://caddy/data/2.5/weather?q=" + context.Param("city") + "&appid=" + apiKey + "&units=metric"
		apiPage, err := http.Get(apiURL)

		if err == nil {
			defer func(Body io.ReadCloser) {
				err := Body.Close()
				if err != nil {

				}
			}(apiPage.Body)
			var apiData map[string]interface{}
			err = json.NewDecoder(apiPage.Body).Decode(&apiData)
			if err == nil {
				if main, ok := apiData["main"].(map[string]interface{}); ok {
					if temp, ok := main["temp"].(float64); ok {
						if humidity, ok := main["humidity"].(float64); ok {
							if wind, ok := apiData["wind"].(map[string]interface{}); ok {
								if speed, ok := wind["speed"].(float64); ok {
									importedWeather := ImportedWeather{
										Temperature: temp,
										Humidity:    int(humidity),
										Wind:        speed,
									}

									exportedWeather := ExportedWeather{
										City:        context.Param("city"),
										Temperature: importedWeather.Temperature,
										Humidity:    importedWeather.Humidity,
										Wind:        importedWeather.Wind,
									}

									return context.JSON(http.StatusOK, exportedWeather)
								}
							}
						}
					}
				}
			}
		}
		return context.JSON(http.StatusInternalServerError, map[string]string{
			"message": "Wrong Weather API response received",
		})
	})
	app.Logger.Fatal(app.Start(":80"))
}

type ImportedWeather struct {
	Temperature float64 `json:"temperature"`
	Humidity    int     `json:"humidity"`
	Wind        float64 `json:"wind"`
}

type ExportedWeather struct {
	City        string  `json:"city"`
	Temperature float64 `json:"temperature"`
	Humidity    int     `json:"humidity"`
	Wind        float64 `json:"wind"`
}
