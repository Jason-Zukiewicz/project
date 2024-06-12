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

