SHELL := /bin/bash

.PHONY: help web desktop android ios check-dx

help:
	@echo "KoloHub web app commands"
	@echo ""
	@echo "make web       - Run Dioxus in web mode"
	@echo "make desktop   - Run Dioxus in desktop mode"
	@echo "make android   - Run Dioxus in Android mode"
	@echo "make ios       - Run Dioxus in iOS mode (macOS only)"

check-dx:
	@command -v dx >/dev/null || (echo "Error: dx not found. Install with: cargo install dioxus-cli --locked" && exit 1)

web: check-dx
	dx serve --platform web

desktop: check-dx
	dx serve --platform desktop

android: check-dx
	dx serve --platform android

ios: check-dx
	dx serve --platform ios
