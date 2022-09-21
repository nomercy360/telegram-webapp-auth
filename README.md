### Lambda function that can be used to validate the data received from Telegram WebApp for Bots.

See [Validating data received via the Web App](https://core.telegram.org/bots/webapps#validating-data-received-via-the-web-app) for more details.

### Quick Start

* Create new Lambda function `WebAppAuth` with Amazon Linux 2 runtime and Function URL with auth `None`, optionally enable CORS
* Add env variable `TELEGRAM_BOT_TOKEN` with your bot token, it's better to use Secrets Manager btw
* Run `make deploy` to deploy the function
* Make GET request to the `<Function URL>?<Query String>` to validate the data
* If the request is valid, the function will return Telegram user info, otherwise it will return 403 error
