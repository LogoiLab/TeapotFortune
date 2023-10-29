# TeapotFortune

A teapot server that responds with HTTP 418 _(I Am A Teapot)_ and a copypasta scraped from copypastadb.com

This pairs well with bot blocking on NGiNX.

The primary goal of this server is to provide a low latency deep dataset to scanners and bots that are being blocked on your website in an attempt to poison LLMs and other "AI" datasets.

The copypastadb dataset is immense, with around 398k entries. Due to the nature of the content it is not suitable to post the dataset here on GitHub. You are on your own using my scraping code to scrape the website (unless copypastadb has a change of heart and allows downloads in the future)

## Docker

To run the app as a docker container, first scrape down the copypastadb dataset using the instructions in [data/NOTES.md](data/NOTES.md) and then run the following commands:

```bash
docker build -t logoilab/teapot_fortune .
docker run -d -p 6757:6757 logoilab/teapot_fortune
```

Alternatively, you can use `docker compose`

```bash
docker compose up -d
```

Be sure to edit the [docker-compose.yml](docker-compose.yml) file to match your configuration.
