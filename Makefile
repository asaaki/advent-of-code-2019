# Advent of Code

MAKEFILE = $(firstword $(MAKEFILE_LIST))

# https://stackoverflow.com/a/14061796
# make new <int>
ifeq (new,$(firstword $(MAKECMDGOALS)))
  NEW_ARGS := $(wordlist 2,$(words $(MAKECMDGOALS)),$(MAKECMDGOALS))
  NEW_DAY_NAME = $(firstword $(NEW_ARGS))
  $(eval $(NEW_ARGS):;@:)
endif
# make day <int>
ifeq (day,$(firstword $(MAKECMDGOALS)))
  DAY_ARGS := $(wordlist 2,$(words $(MAKECMDGOALS)),$(MAKECMDGOALS))
  DAY_TO_RUN = $(firstword $(DAY_ARGS))
  $(eval $(DAY_ARGS):;@:)
endif

DAY_DIR_PREFIX = day_
DAYS=$(shell find * -maxdepth 0 -type d -iname "$(DAY_DIR_PREFIX)*")
DAY_NUMERS=$(DAYS:$(DAY_DIR_PREFIX)%=%)
DAY_NAME = $(shell basename $(CURDIR))

export CARGO_TARGET_DIR = ../target

help:
	@echo "Run with make day <int> or make all"
	@echo "Available days: $(DAY_NUMERS)"

all: $(DAYS)

# make day_<int>
$(DAYS): %:
	@$(MAKE) single-day -s -f ../$(MAKEFILE) -C $@
.PHONY: $(DAYS)

# make day <int> -> make day_<int>
day:
	@$(MAKE) $(DAY_DIR_PREFIX)$(DAY_TO_RUN)

### target to be called from within a day directory

single-day:
	echo "************************"
	echo "AoC $(DAY_NAME)"
	cargo fmt
	cargo +nightly fix -Z unstable-options --clippy --allow-dirty
	cargo check
	cargo test -- --nocapture
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
	-cargo install --git https://github.com/hhatto/cargo-strict.git
	-cargo install --git https://github.com/kbknapp/cargo-outdated
	-cargo install cargo-cache
	-cargo install cargo-deps
	-cargo install cargo-edit
	-cargo install cargo-modules
	-cargo install cargo-update
