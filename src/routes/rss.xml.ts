import type { RequestHandler } from '@sveltejs/kit';
import { list, type Post } from '$lib/posts';
import { compareDesc } from 'date-fns';

const baseUrl = 'https://galaiko.rocks/';

export const get: RequestHandler = async () => {
	const posts = await list();
	const body = render(
		posts
			.filter(({ hidden }) => !hidden)
			.sort((a, b) => compareDesc(new Date(a.date), new Date(b.date)))
	);
	const headers = {
		'Cache-Control': 'max-age=0, s-maxage=3600',
		'Content-Type': 'application/xml'
	};
	return {
		body,
		headers
	};
};

const renderPost = (post: Post) => `
        <item>
            <title>${post.title}</title>
            <link>${new URL(post.path, baseUrl)}</link>
            <pubDate>${post.date.toUTCString()}</pubDate>
            <author>Nikita Galaiko (nikita@galaiko.rocks)</author>
            <guid>${post.path}</guid>
            <content:encoded>${post.html}</content:encoded>
        </item>`;

const render = (posts: Post[]) =>
	`<?xml version="1.0" encoding="utf-8" standalone="yes"?>
<rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom">
    <channel>
        <title>Posts | Nikita Galaiko</title>
        <link>${baseUrl}</link>
        <description>Recent content on Nikita Galaiko's website</description>
        <language>en</language>
        <managingEditor>Nikita Galaiko (nikita@galaiko.rocks)</managingEditor>
        <webMaster>Nikita Galaiko <nikita@galaiko.rocks></webMaster>
        <copyright>CC BY-NC 4.0</copyright>
        <lastBuildDate>${new Date().toUTCString()}</lastBuildDate>
        <atom:link href="https://galaiko.rocks/rss.xml" rel="self" type="application/rss+xml />
        ${posts.map(renderPost).join('')}
    </channel>
</rss>`;
