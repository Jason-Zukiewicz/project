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

# # # # # # # # # # # # # # # # #
# D O C K E R   C O M M A N D S #
# # # # # # # # # # # # # # # # #
env:
	@docker build -t ubuntu .
	@echo Ubuntu Container Loaded.
	@docker run -it --name ubuntu.venv -v ${PWD}:/root/volume ubuntu

# Runs Container
run:
	@echo Booted.
	@docker start ubuntu.venv
	@docker attach ubuntu.venv

# Deletes Container AND Image
clean:
	@docker stop ubuntu.venv
	@docker container rm ubuntu.venv
	@docker image rm ubuntu
