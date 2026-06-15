#!/usr/bin/bash

cd frontend && cd kolohub-web
sleep 2
echo "Starting development server"
sleep 2
dx serve --platform web&
sleep 1
clear
sleep 1
echo "Development server is running. Visit localhost:8080 to view the site live"