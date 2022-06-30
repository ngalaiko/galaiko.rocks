# galaiko.rocks

[![Update Restaurants and Cafes list](https://github.com/ngalaiko/blog/actions/workflows/restaurants_and_cafes.yaml/badge.svg)](https://github.com/ngalaiko/blog/actions/workflows/restaurants_and_cafes.yaml)
[![Update records list](https://github.com/ngalaiko/blog/actions/workflows/records.yaml/badge.svg)](https://github.com/ngalaiko/blog/actions/workflows/records.yaml)
[![Download webmentions](https://github.com/ngalaiko/galaiko.rocks/actions/workflows/download-webmentions.yaml/badge.svg)](https://github.com/ngalaiko/galaiko.rocks/actions/workflows/download-webmentions.yaml)

my website with a basic webmentions support

## cloudflare pages deployment

Build command:
```
yarn install --frozen-lockfile && yarn build && yarn build
```

Note: double build is not a mistake. This is required to make sure that records' covers are downloaded for ssr.

Build output directory:
```
.svelte-kit/cloudflare
```
