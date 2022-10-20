---
title: 'Hello, IndieWeb'
tags: ['vim', 'development']
date: '2022-04-16'
section: 'Blog'
---

In this post I want to share how this website connects with [IndieWeb][].

## What is IndieWeb

[IndieWeb][] how it describes itself, is:

> a people-focused alternative to the "corporate web"

I also don't like the "corporate web" and try not to participate in it. However, right now IndieWeb for me is mostly
something fun to do on the weekends.

On a more technical level, it's a combination of protocols like [WebMentions][], [Microsub][] and
[some others](https://www.w3.org/TR/social-web-protocols/) that together enable a distributed social interaction on
the Web.

## What I would like to achieve

- be discoverable on IndieWeb
- be able to interact with others on IndieWeb
- not use any 3rd party IndieWeb services like [webmention.io][]
- pay zero moneys for my setup
- have all of my data locally stored in a form of static files
- my website to be full functioning without client side javascript

## Setup

I used to power this website with [Hugo](https://gohugo.io) - a static website generator. But it doesn't quite fit
IndieWeb requirements for me.

Most people, as I understand, use centralized platforms like [webmention.io][] or
[micro.blog](https://micro.blog) mixed with different [CMS plugins](https://wordpress.org/plugins/indieweb/) to be on
IndieWeb. For me - it's no fun. I want to do it all myself - thus static website won't do it. I have to have a server
to at least receive webmentions.

Some time ago [SvelteKit](https://kit.svelte.dev) got me interested. SvelteKit is a framework to build webapps using
[Svelte](https://svelte.dev). What is especially cool about it for my use-case is that it is:

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

First step in IndieWeb is to setup [microformats](http://microformats.org). It's a data format built on top of xml that
provides a standard way of communicating between IndieWeb sites, meaning:

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

Second step is webmentions. Webmentions is a very simple protocol to let other's know that you've mentioned their page.
A mention could be anything form a 'like' to 'comment' or even 'rsvp'.

Since [the protocol](https://www.w3.org/TR/webmention/#updating-existing-webmentions-li-4) is so simple, I am implementing
it myself. Both because it's more fun and to satisfy my requirements for the project.

There are two parts when it comes to webmentions: receiving and sending.

### Receiving

Since I don't want to pay for the hosting, here is a plan:

1. When someone send me a webmention, the request is be processed by a serverless handler hosted on
   [cloudflare workers](https://developers.cloudflare.com/workers/) and then stored in their
   [key value storage](https://developers.cloudflare.com/workers/platform/pricing/#workers-kv)
2. Later, [an export script](https://github.com/ngalaiko/blog/blob/12021e08b9310aaae9b3d8aa3a179cc9e3549473/scripts/webmentions/download.ts) is triggered
   by myself manually or via GitHub Actions cronjob to download all of the new
   webmentions [into my repository](https://github.com/ngalaiko/blog/blob/12021e08b9310aaae9b3d8aa3a179cc9e3549473/src/lib/data/webmentions.json)
3. Then, [a processing script](https://github.com/ngalaiko/blog/blob/12021e08b9310aaae9b3d8aa3a179cc9e3549473/scripts/webmentions/process.ts) is
   triggered in a similar manner to process new webmentions
4. Viola, now all the raw webmentions data is available in
   my repository at all times, and I can use it to render static pages

What I like about this plan is that:

- it doesn't really depend much on the hosting solution since SvelteKit is quite
  flexible with that
- I expect the amount of data and read/write operations be close to 0, so free tier on cloudflare and github is more
  than enough
- all of the data is stored inside my repository which means I can render static pages

Of course the non real timeness of the solution might be considered a downside.

### Sending

After receiving part was done I conviniently had written code that can be reused to send webmentions too. Since this
website is fully prerendered and distributed as a set of static files, I can analyze those files and extract my
webmentions from it.

Once webmentions are extracted, it's trivial to implement [the discovery](https://www.w3.org/TR/webmention/#sender-discovers-receiver-webmention-endpoint)
and make [script](https://github.com/ngalaiko/galaiko.rocks/blob/12021e08b9310aaae9b3d8aa3a179cc9e3549473/scripts/webmentions/send.ts) ping the endpoints.

## Test

The footer of this article contains a list of received webmentions, and I've already sent one!

## Update

Support for webmentions was successfully removed, since I do not use it :)

[webmentions]: https://indieweb.org/Webmention
[indieweb]: https://indieweb.org
[microsub]: https://indieweb.org/Microsub
[webmention.io]: https://webmention.io
[sveltekit]: https://kit.svelte.dev
