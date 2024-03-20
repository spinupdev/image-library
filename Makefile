

build: build-image build-builder


build-image:
	@${MAKE} -C images/ubuntu build

build-builder:
