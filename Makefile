css-build:
	@npx tailwindcss -i ./input.css -o tailwind.css --watch

t:
	dx translate --file ./src/index.html --output ./src/index.rs --component

run:
	dx serve --hot-reload --platform fullstack
