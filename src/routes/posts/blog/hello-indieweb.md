---
title: 'Hello, IndieWeb'
tags: ['vim', 'development']
date: '2022-04-16'
categories: ['Blog']
hidden: true
---

In this post I want to share how this website connects with [IndieWeb][] and webmention myself!

## What is IndieWeb

[IndieWeb][] how it describes itself, is:

> a people-focused alternative to the "corporate web"

I also don't like the "corporate web" and try not to participate in it. However, right now IndieWeb for me is mostly
something fun to do on the weekends.

On a more technical level, it's a combination of protocols like [WebMentions][], [Microsub][] and
[some others](https://www.w3.org/TR/social-web-protocols/) that together enable a distributed social interaction on
the Web.

## What I want

- I'd like to be discoverable on IndieWeb
- I'd like to be be able to interact with others on IndieWeb
- I'd like to not use any 3rd party IndieWeb services like [webmention.io][]
- I'd like to pay zero moneys for my setup
- I'd like to have all of my data locally stored in a form of static files
- I'd like my website to be full functioning without client side javascript

## Setup

I used to power this website with [Hugo](https://gohugo.io) - a static website generator. But it doesn't quite fit
IndieWeb requirements for me.

Most people, as I understand, use centralized platforms like [webmention.io][] and
[micro.blog](https://micro.blog) mixed with different [CMS plugins](https://wordpress.org/plugins/indieweb/) to be on
IndieWeb. For me - it's no fun. I want to do it all myself - thus static website won't do it. I have to have a server
to at least receive webmentions.

Some time ago [SvelteKit](https://kit.svelte.dev) got me interested. SvelteKit is a framework to build webapps using
[Svelte](https://svelte.dev). What is especially cool about it is that it is:

- prerendered by default
- comes with [adapters](https://github.com/sveltejs/kit/tree/master/packages) for different deploy targets out of the box
- allows to seamlessly write serverless functions

With that said, my tech choice is: [SvelteKit][] deployed to [Cloudflare Pages](https://pages.cloudflare.com)

### SvelteKit

I won't dive into details here, you can check out the [code](https://github.com/ngalaiko/blog). Pretty much it's [SvelteKit][]
with [mdsvex](https://mdsvex.com). While configuring it, there two articles helped me a lot:

- [Let's learn SvelteKit by building a static Markdown blog from scratch](https://joshcollinsworth.com/blog/build-static-sveltekit-markdown-blog/)
- [MDSveX and Svelte Kit](https://www.furudean.com/blog/svelte-kit-mdsvex)

## Microformats

First is to setup [microformats](http://microformats.org). It's a data format built on top of xml that provides a
standard way of communicating between IndieWeb sites, meaning:

You add some attributes to your website's html:

```html
<a class="h-card" href="/" rel="me">Jimmy</a>
```

Microformats parsers take html and convert it to standardised JSON:

```JSON
{
  "items": [
    {
      "properties": {
        "name": ["Jimmy"],
        "url": ["http://example.com/"]
      },
      "type": ["h-card"]
    }
  ],
  "rel-urls": {
    "http://example.com": {
      "rels": ["me"],
      "text": "Jimmy"
    }
  },
  "rels": {
    "me": ["http://example.com/"]
  }
}
```

The JSON is used to power comments functionality on your website (for example).

You can read more [here](https://indieweb.org/microformats) and [here](http://microformats.org).

## Webmentions

Second step is webmentions. TODO

### Receiving

TODO

### Processing

TODO

### Sending

TODO

## Test

Webmention myself!

[webmentions]: https://indieweb.org/Webmention
[indieweb]: https://indieweb.org
[microsub]: https://indieweb.org/Microsub
[webmention.io]: https://webmention.io
[sveltekit]: https://kit.svelte.dev
