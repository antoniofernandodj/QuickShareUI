# =========================
# Project config
# =========================
APP_NAME := quickshare-ui
IMAGE := $(APP_NAME):latest
CONTAINER := $(APP_NAME)-dev
PORT := 8080

# =========================
# Helpers
# =========================
.DEFAULT_GOAL := help

help:
	@echo ""
	@echo "Available commands:"
	@echo "  make build     - Build docker image"
	@echo "  make run       - Run container locally"
	@echo "  make down      - Stop and remove container"
	@echo "  make logs      - Show container logs"
	@echo "  make clean     - Remove image and build artifacts"
	@echo "  make serve     - Trunk serve (local dev, no docker)"
	@echo ""

# =========================
# Docker
# =========================
build:
	docker build -t $(IMAGE) .

run:
	docker run --name $(CONTAINER) -p $(PORT):80 --rm $(IMAGE)

down:
	-docker stop $(CONTAINER)
	-docker rm $(CONTAINER)

logs:
	docker logs -f $(CONTAINER)

clean:
	-docker rmi $(IMAGE)
	-rm -rf dist target

# =========================
# Local dev (no docker)
# =========================
serve:
	trunk serve --open

