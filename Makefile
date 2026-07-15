.PHONY: build base desktop ubuntu workstation

build: base desktop ubuntu workstation

base:
	$(MAKE) -C images/base build

desktop: base
	$(MAKE) -C images/desktop build

ubuntu:
	$(MAKE) -C images/ubuntu build

workstation: base
	$(MAKE) -C images/workstation build
