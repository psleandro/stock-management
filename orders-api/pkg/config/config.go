package config

import (
	"log"
	"os"

	"github.com/joho/godotenv"
)

type Config struct {
	ApiPort      string
	PostgresHost string
	PostgresPort string
	PostgresUser string
	PostgresPass string
	PostgresDB   string
	KafkaBrokers string
}

func Load() *Config {
	err := godotenv.Load()

	if err != nil {
		log.Fatal("Error loading .env file")
	}

	return &Config{
		ApiPort:      getEnv("PORT", "8080"),
		PostgresHost: getEnv("POSTGRES_HOST", "localhost"),
		PostgresPort: getEnv("POSTGRES_PORT", "5432"),
		PostgresUser: getEnv("POSTGRES_USER", "myuser"),
		PostgresPass: getEnv("POSTGRES_PASSWORD", "secretpassword"),
		PostgresDB:   getEnv("POSTGRES_DB", "orders"),
		KafkaBrokers: getEnv("KAFKA_BROKERS", "localhost:9092"),
	}
}

func getEnv(envKey string, fallback string) string {
	if env := os.Getenv(envKey); env != "" {
		return env
	}

	return fallback
}
