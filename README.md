# galaiko.rocks

my website

## update 3rd party data

### letterboxd

```bash
$ cargo run --package update letterboxd
```

### discogs

```bash
$ cargo run --package update discogs --token <personal access token>
```

### hledger

```bash
$ cargo run --package update hledger [--file <main hledger file>]
```

## run

```bash
$ cargo run --package serve
```

## deploy

this requires [fly.io](https://fly.io) account.

```
$ fly deploy
```
