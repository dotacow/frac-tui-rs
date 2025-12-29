COMPOSE_FILE := docker/docker-compose.yaml
SERVICE_NAME := frac-tui

.PHONY: help up down build run logs clean

help:
	@echo "available commands:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2}'

run: ## Build and run interactively (if unsure run this)
	docker compose -f $(COMPOSE_FILE) run --rm $(SERVICE_NAME)

build: ## Build the Docker image
	docker compose -f $(COMPOSE_FILE) build

clean: ## Remove images and containers
	docker compose -f $(COMPOSE_FILE) down --rmi local

# If you're wondering what a makefile is doing in a rust project,it's exclusively for easy docker management, all actual dependency management is done through cargo.