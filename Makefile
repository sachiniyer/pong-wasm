##
# Pong Wasm
#
# @file
# @version 0.1
#
SHELL := /bin/bash
TARGET := no-modules

.PHONY: all
all: build

.PHONY: build
build:
	@echo "Building wasm-pack..."
	wasm-pack build --target no-modules

.PHONY: clean
clean:
	@echo "Cleaning up..."
	cargo clean
	rm -rf pkg
	rm -rf target
	rm -rf Cargo.lock

.DEFAULT_GOAL := build

