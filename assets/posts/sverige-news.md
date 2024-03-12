---
title: "Sverige News"
date: 2024-03-12
id: "/posts/sverige-news.html"
---

i have been using [thetruestorynews](https://thetruestory.news/en) for a while now.
idea of automatic news aggregation, without any human curation, is very appealing to me.

a few days ago [this post](https://duarteocarmo.com/blog/newshavn-danish-news-in-english) popped up in my rss feed. it is about a small service Duarte built for himself to keep track of news in denmark, where he lives, but does not speak danish.

i thought it would be cool to have something similar for sweden, where i live, but do not speak swedish. so i built it. with a little sprinkles of ai.

instead of translating articles, my service first groups them by meaning and only translates headlines for each group.

grouping is done by using openai's embeddings api and dbscan algorithm. it is not perfect, but good enough for my use case.

here it is: [sverige-news.fly.dev](https://sverige-news.fly.dev)
