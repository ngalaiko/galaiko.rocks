# galaiko.rocks

[![Daily job](https://github.com/ngalaiko/galaiko.rocks/actions/workflows/daily.yaml/badge.svg)](https://github.com/ngalaiko/galaiko.rocks/actions/workflows/daily.yaml)

my website

## deployment

Build command:

```
npm install -g pnpm && pnpm install && (pnpm build || true) && pnpm build
```

The first `build` will fail because records covers won't be downloaded for the first run.

Build output directory:

```
build
```
