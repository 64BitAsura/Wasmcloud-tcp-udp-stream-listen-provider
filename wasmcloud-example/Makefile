.PHONY: build deploy undeploy clean

build:
	@echo "Building provider..."
	cd .. && cargo build --release
	@echo "Building component..."
	cd consumer-component && wash build

deploy:
	@echo "Deploying application..."
	wash app deploy wadm.yaml

undeploy:
	@echo "Undeploying application..."
	wash app undeploy tcp-udp-stream-example

status:
	@echo "Application status..."
	wash app list

clean:
	cd .. && cargo clean
	cd consumer-component && cargo clean
