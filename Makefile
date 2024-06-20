# # # # # # # #
# D O C K E R #
# # # # # # # #

# Compiles a fresh image and new container
img:
	@docker build -t ubuntu .
	@docker run -it --name server -p 8080:8080 -p 3000:3000 -v ${PWD}:/root/volume ubuntu

# Runs the closed container
run:
	@docker start server
	@docker attach server

# Closes the server without deleting it
stop:
	@docker stop server

# Deletes Container AND Image
clean:
	@docker stop server
	@docker container rm server
	@docker image rm ubuntu


# # # # # # # # #
# R U N T I M E #
# # # # # # # # #

.PHONY: backend backtest frontend

f: frontend
frontend:
	live-server --port=8080 ./frontend/

b: backend
backend:
	cd backend && cargo watch -q -c -w src/ --poll -x run

bt: backtest
backtest:
	cd backend && cargo watch -q -c -w tests/ --poll -x "test -q quick_dev -- --nocapture"
