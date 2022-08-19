# galaiko.rocks

[![Update Restaurants and Cafes list](https://github.com/ngalaiko/blog/actions/workflows/restaurants_and_cafes.yaml/badge.svg)](https://github.com/ngalaiko/blog/actions/workflows/restaurants_and_cafes.yaml)
[![Update records list](https://github.com/ngalaiko/blog/actions/workflows/records.yaml/badge.svg)](https://github.com/ngalaiko/blog/actions/workflows/records.yaml)

my website

## cloudflare pages deployment

Environment variables:

| name         | value |
| ------------ | ----- |
| NODE_VERSION | 17    |
| NPM_FLAGS    | 17    |

Build command:

```
npm install -g pnpm && pnpm install && (pnpm build || true) && pnpm build
```

The first `build` will fail once, because records covers won't be downloaded for the first run.

Build output directory:

```
build
```
