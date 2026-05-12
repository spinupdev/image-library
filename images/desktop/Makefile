IMAGE      = spinupdev/desktop
TAG        = latest
INIT_IMAGE = spinupdev/init
DIR        = $(shell pwd)
DEPOT_INITD = $(DIR)/../../../../depot/components/initd

.PHONY: build build-init run push

# Build the spinupdev/init base image from depot/components/initd source.
# Only needs to run once (or when initd changes).
build-init:
	CGO_ENABLED=0 GOOS=linux go build \
		-C $(DEPOT_INITD) \
		-o $(DIR)/build/initd .
	printf 'FROM scratch\nCOPY build/initd /init\n' | \
		docker build -t $(INIT_IMAGE):latest -f - $(DIR)
	rm -f $(DIR)/build/initd

build: build-init
	docker build -t $(IMAGE):$(TAG) $(DIR)

run:
	docker run --rm -p 8080:8080 \
		-e VNC_PASSWORD=changeme \
		--shm-size=256m \
		$(IMAGE):$(TAG)

push:
	docker push $(IMAGE):$(TAG)
