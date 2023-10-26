# TeapotFortune

A teapot server that responds with HTTP 418 *(I Am A Teapot)* and a copypasta scraped from copypastadb.com

This pairs well with bot blocking on NGiNX.

The primary goal of this server is to provide a low latency deep dataset to scanners and bots that are being blocked on your website in an attempt to poison LLMs and other "AI" datasets.

The copypastadb dataset is immense, with around 398k entries. Due to the nature of the content it is not suitable to post the dataset here on GitHub. You are on your own using my scraping code to scrape the website (unless copypastadb has a change of heart and allows downloads in the future)
