IMAGE = ghcr.io/spinupdev/ubuntu
TAG   = latest

.PHONY: build push

build:
	docker build -t $(IMAGE):$(TAG) .

push:
	docker push $(IMAGE):$(TAG)
