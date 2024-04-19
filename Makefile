run-server:
	@clear
	@cargo watch -x run -p rchaty-server

run-style:
	@sass --watch style/src/scss/style.scss ./assets/css/style.css
