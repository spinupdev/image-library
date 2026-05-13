.PHONY: build desktop ubuntu workstation

build: desktop ubuntu workstation

desktop:
	$(MAKE) -C images/desktop build

ubuntu:
	$(MAKE) -C images/ubuntu build

workstation:
	$(MAKE) -C images/workstation build
