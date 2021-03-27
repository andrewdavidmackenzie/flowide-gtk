APTGET := $(shell command -v apt-get 2> /dev/null)
YUM := $(shell command -v yum 2> /dev/null)
UNAME := $(shell uname)
export SHELL := /bin/bash

.PHONY: all
all: build test clippy

########## Configure Dependencies ############
.PHONY: config
config:
	@echo "Detected $(UNAME)"
ifeq ($(UNAME), Linux)
	@$(MAKE) config-linux
endif
ifeq ($(UNAME), Darwin)
	@$(MAKE) config-darwin
endif

.PHONY: config-darwin
config-darwin:
	@echo "	Installing macos specific dependencies using brew"
	@brew install gtk+3 glib cairo

.PHONY: config-linux
config-linux:
ifneq ($(YUM),)
	@echo "	Installing linux specific dependencies using $(YUM)"
	@sudo yum --color=auto --quiet install curl-devel openssl-devel gtk3-devel || true
else ifneq ($(APTGET),)
	@echo "	Installing linux specific dependencies using $(APTGET)"
	@sudo apt-get -y install libcurl4-openssl-dev libdw-dev libssl-dev libgtk-3-dev || true
else
	@echo "	Neither apt-get nor yum detected for installing linux specific dependencies"
endif

.PHONY: build
build:
	@PKG_CONFIG_PATH="/usr/local/lib/pkgconfig:/usr/local/opt/lib/pkgconfig:/usr/local/Cellar/glib/2.62.3/lib/pkgconfig:/usr/lib64/pkgconfig" cargo build

.PHONY: run
run:
	@PKG_CONFIG_PATH="/usr/local/lib/pkgconfig:/usr/local/opt/lib/pkgconfig:/usr/local/Cellar/glib/2.62.3/lib/pkgconfig:/usr/lib64/pkgconfig" cargo run

.PHONY: test
test:
	@PKG_CONFIG_PATH="/usr/local/lib/pkgconfig:/usr/local/opt/lib/pkgconfig:/usr/local/Cellar/glib/2.62.3/lib/pkgconfig:/usr/lib64/pkgconfig" cargo test

.PHONY: clippy
clippy:
	cargo clippy