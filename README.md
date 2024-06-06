# galaiko.rocks

my website

## requirements

- [python3](https://www.python.org)
- [bash](https://www.gnu.org/software/bash/)
- [make](https://www.gnu.org/software/make/)
- [sed](https://www.gnu.org/software/sed/)
- [jq](https://github.com/jqlang/jq)
- [yq](https://github.com/mikefarah/yq)
- [cookcli](https://github.com/cooklang/cookcli)
- [imagemagick](https://imagemagick.org)
- [pandoc](https://pandoc.org)

## build

```bash
$ make
```

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
