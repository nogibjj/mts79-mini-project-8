install:
	pip install --upgrade pip &&\
		pip install -r requirements.txt

test:
	python -m pytest -vv --cov=main --cov=mylib test_*.py

format:	
	black *.py 

lint:
	ruff check *.py mylib/*.py

container-lint:
	docker run --rm -i hadolint/hadolint < Dockerfile

refactor: format lint
		
all: install lint test format deploy

extract:
	python main.py extract

transform_load: 
	python main.py transform_load

query:
	python main.py general_query "SELECT * FROM USBirths WHERE year=2002;"