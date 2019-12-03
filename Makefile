# Advent of Code

MAKEFILE = $(firstword $(MAKEFILE_LIST))

DAYS=$(shell find * -maxdepth 0 -type d '!' -iname "target")
DAY_NAME = $(shell basename $(CURDIR))

export CARGO_TARGET_DIR = ../target

help:
	@echo "Run with make <day-as-word> or make all"

all: $(DAYS)

$(DAYS): %:
	@$(MAKE) day -s -f ../$(MAKEFILE) -C $@
.PHONY: $(DAYS)

day:
	echo "************************"
	echo "AoC - day $(DAY_NAME)"
	cargo fmt
	cargo check
	cargo test
	cargo build --release
	echo
	../target/release/$(DAY_NAME) input/modules.txt
	echo
	echo "^^^^^^^^^^^^^^^^^^^^^^^^"
.PHONY: day
