package config

import (
	"log"
	"os"

	"github.com/joho/godotenv"
)

type Config struct {
	ApiPort string
}

func Load() *Config {
	err := godotenv.Load()

	if err != nil {
		log.Fatal("Error loading .env file")
	}

	return &Config{
		ApiPort: getEnv("PORT", "8080"),
	}
}

func getEnv(envKey string, fallback string) string {
	if env := os.Getenv(envKey); env != "" {
		return env
	}

	return fallback
}
