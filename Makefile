default: test

.PHONY: build test release

build:
	python setup.py develop

test: build
	pytest -v 

release:
	# Not working yet
	docker run --rm -v `pwd`:/io quay.io/pypa/manylinux2010_x86_64 /io/scripts/build-wheels.sh
