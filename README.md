# galaiko.rocks

[![Update Restaurants and Cafes list](https://github.com/ngalaiko/blog/actions/workflows/restaurants_and_cafes.yaml/badge.svg)](https://github.com/ngalaiko/blog/actions/workflows/restaurants_and_cafes.yaml)
[![Update records list](https://github.com/ngalaiko/blog/actions/workflows/records.yaml/badge.svg)](https://github.com/ngalaiko/blog/actions/workflows/records.yaml)
[![Download webmentions](https://github.com/ngalaiko/galaiko.rocks/actions/workflows/download-webmentions.yaml/badge.svg)](https://github.com/ngalaiko/galaiko.rocks/actions/workflows/download-webmentions.yaml)

my website with a basic webmentions support

## cloudflare pages deployment

Environment variables:

| NODE_VERSION | 17  |
| ------------ | --- |
| NPM_FLAGS    | 17  |

Build command:

```
npm install -g pnpm && pnpm install && pnpm build && pnpm build
```

Build output directory:

```
.svelte-kit/cloudflare
```
