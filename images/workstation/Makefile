IMAGE = spinupdev/workstation
TAG   = latest

.PHONY: build run

build:
	docker build -t $(IMAGE):$(TAG) .

run:
	docker run --rm -it \
		--privileged \
		-v /var/run/docker.sock:/var/run/docker.sock \
		-p 2222:22 \
		$(IMAGE):$(TAG) \
		/bin/bash
