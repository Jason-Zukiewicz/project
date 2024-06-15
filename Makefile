.PHONY: backend backtest frontend

b: backend
backend:
	cd backend && cargo watch -q -c -w src/ --poll -x run

bt: backtest
backtest:
	cd backend && cargo watch -q -c -w tests/ --poll -x "test -q quick_dev -- --nocapture"

f: frontend
frontend:
	# Add frontend commands here

###################
# DOCKER COMMANDS #

venv:
	@echo Building Ubuntu iso...
	@docker build -t ubuntu .
	@echo Initializing virtual environment...
	@docker run -it --name ubuntu.venv ubuntu

# Runs the docker container
run:
	@echo Booting container...
	@docker start ubuntu.venv
	@docker attach ubuntu.venv

# Deletes docker container and image
clean:
	@docker stop ubuntu.venv
	@docker container rm ubuntu.venv
	@docker image rm ubuntu




