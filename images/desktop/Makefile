IMAGE = ghcr.io/spinupdev/desktop
TAG   = latest
DIR   = $(shell pwd)

.PHONY: build run push

build:
	docker build -t $(IMAGE):$(TAG) $(DIR)

run:
	docker run --rm -p 8080:8080 \
		-e VNC_PASSWORD=changeme \
		--shm-size=256m \
		$(IMAGE):$(TAG)

push:
	docker push $(IMAGE):$(TAG)
