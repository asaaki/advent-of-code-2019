# Advent of Code

MAKEFILE = $(firstword $(MAKEFILE_LIST))

# https://stackoverflow.com/a/14061796
ifeq (new,$(firstword $(MAKECMDGOALS)))
  NEW_ARGS := $(wordlist 2,$(words $(MAKECMDGOALS)),$(MAKECMDGOALS))
  NEW_DAY_NAME = $(firstword $(NEW_ARGS))
  $(eval $(NEW_ARGS):;@:)
endif

DAY_DIR_PREFIX = day_
DAYS=$(shell find * -maxdepth 0 -type d -iname "$(DAY_DIR_PREFIX)")

DAY_NUMERS=$(DAYS:$(DAY_DIR_PREFIX)%=%)
DAY_NAME = $(shell basename $(CURDIR))

export CARGO_TARGET_DIR = ../target

help:
	@echo "Run with make <day-as-word> or make all"
	@echo "Available days: $(DAY_NUMERS)"

all: $(DAYS)

$(DAY_NUMERS): %:
	@$(MAKE) day -s -f ../$(MAKEFILE) -C $(DAY_DIR_PREFIX)$@
.PHONY: $(DAY_NUMERS)

### target to be called from within a day directory

day:
	echo "************************"
	echo "AoC $(DAY_NAME)"
	cargo fmt
	cargo check
	cargo test
	cargo build --release
	echo
	../target/release/$(DAY_NAME) input/data.txt
	echo
	echo "^^^^^^^^^^^^^^^^^^^^^^^^"

### generate a new day

COMMON_CRATES = \
	clap@2.33

new:
	cargo new --bin --edition 2018 $(DAY_DIR_PREFIX)$(NEW_DAY_NAME)
	cd $(DAY_DIR_PREFIX)$(NEW_DAY_NAME) && \
		for CRATE in $(COMMON_CRATES); do cargo add $$CRATE; done && \
		cargo check && \
		mkdir input && \
		curl -sSL --cookie "$(SESSION_COOKIE)" \
			https://adventofcode.com/2019/day/$(NEW_DAY_NAME)/input \
			-o input/data.txt

cargo-utils:
	-cargo install cargo-edit
	-cargo install --git https://github.com/kbknapp/cargo-outdated
	-cargo install cargo-cache
	-cargo install cargo-modules
	-cargo install cargo-deps
