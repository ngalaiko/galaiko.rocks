# galaiko.rocks

my website. builds with make, runs on [fly][].

## requirements

- [uv](https://docs.astral.sh/uv)
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
$ uv run ./scripts/update/letterboxd.py
```

### discogs

```bash
$ uv run ./scripts/update/discogs.py --token <personal access token>
```

### ledger

```bash
$ uv run ./scripts/update/ledger.py [--file <main ledger file>]
```

## run

```bash
$ make serve
```

## deploy

this requires [fly.io][] account.

```
$ fly deploy
```

[fly.io]: https://fly.io)
