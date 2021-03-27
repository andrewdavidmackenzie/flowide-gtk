APTGET := $(shell command -v apt-get 2> /dev/null)
YUM := $(shell command -v yum 2> /dev/null)
STIME = @mkdir -p target;date '+%s' > target/.$@time ; echo "<------ Target '$@' starting"
ETIME = @read st < target/.$@time ; st=$$((`date '+%s'`-$$st)) ; echo "------> Target '$@' done in $$st seconds"
UNAME := $(shell uname)
export SHELL := /bin/bash

.PHONY: all
all: build

########## Configure Dependencies ############
.PHONY: config
config:
	$(STIME)
	@echo "Detected $(UNAME)"
ifeq ($(UNAME), Linux)
	@$(MAKE) config-linux
endif
ifeq ($(UNAME), Darwin)
	@$(MAKE) config-darwin
endif
	$(ETIME)

.PHONY: config-darwin
config-darwin:
	$(STIME)
	@echo "	Installing macos specific dependencies using brew"
	@brew install gtk+3 glib cairo zmq
	$(ETIME)

.PHONY: config-linux
config-linux:
	$(STIME)
ifneq ($(YUM),)
	@echo "	Installing linux specific dependencies using $(YUM)"
	@sudo yum --color=auto --quiet install curl-devel openssl-devel || true
	@sudo yum --color=auto --quiet install gtk3-devel zeromq zeromq-devel || true
else ifneq ($(APTGET),)
	@echo "	Installing linux specific dependencies using $(APTGET)"
	@sudo apt-get -y install libcurl4-openssl-dev libdw-dev libssl-dev || true
	@sudo apt-get -y install libgtk-3-dev libzmq3-dev || true
else
	@echo "	Neither apt-get nor yum detected for installing linux specific dependencies"
endif
	$(ETIME)

.PHONY: build
build:
	$(STIME)
	@PKG_CONFIG_PATH="/usr/local/lib/pkgconfig:/usr/local/opt/lib/pkgconfig:/usr/local/Cellar/glib/2.62.3/lib/pkgconfig:/usr/lib64/pkgconfig" cargo build
	$(ETIME)

.PHONY: run
run:
	$(STIME)
	@PKG_CONFIG_PATH="/usr/local/lib/pkgconfig:/usr/local/opt/lib/pkgconfig:/usr/local/Cellar/glib/2.62.3/lib/pkgconfig:/usr/lib64/pkgconfig" cargo run
	$(ETIME)

.PHONY: test
test:
	$(STIME)
	@PKG_CONFIG_PATH="/usr/local/lib/pkgconfig:/usr/local/opt/lib/pkgconfig:/usr/local/Cellar/glib/2.62.3/lib/pkgconfig:/usr/lib64/pkgconfig" cargo run
	$(ETIME)