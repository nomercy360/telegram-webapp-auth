

deploy:
	cargo lambda build --release; \
    zip -j target/lambda/lambda.zip target/lambda/webapp-auth/bootstrap; \
    aws lambda update-function-code \
    	--function-name WebAppAuth \
    	--zip-file fileb://target/lambda/lambda.zip \
    	--output table --no-cli-page
