# galaiko.rocks

my website

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
$ make -j$(nproc)
```

## update 3rd party data

### letterboxd

```bash
$ python3 ./scripts/update/letterboxd.py
```

### discogs

```bash
$ python3 ./scripts/update/discogs.py --token <personal access token>
```

### hledger

```bash
$ python3 ./scripts/update/hledger.py [--file <main hledger file>]
```

## run

```bash
$ make serve
```

## deploy

this requires [fly.io](https://fly.io) account.

```
$ fly deploy
```
