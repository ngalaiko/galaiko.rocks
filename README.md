# galaiko.rocks

my website

## update 3rd party data

```bash
$ cargo run --package update letterboxd
$ cargo run --package update discogs --token <personal access token>
$ cargo run --package update hledger [--file <main file>]
```

## serve processed assets

```bash
$ cargo run --package serve
```
