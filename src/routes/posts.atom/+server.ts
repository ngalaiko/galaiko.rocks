import type { RequestHandler } from '@sveltejs/kit';
import { list, type Post } from '$lib/posts';
import { compareDesc, max } from 'date-fns';

const baseUrl = 'https://galaiko.rocks/';

export const GET: RequestHandler = async () => {
	const posts = await list();
	const body = render(
		posts
			.filter(({ hidden }) => !hidden)
			.sort((a, b) => compareDesc(new Date(a.date), new Date(b.date)))
	);
	return new Response(body, {
		headers: {
			'Cache-Control': 'max-age=0, s-maxage=3600',
			'Content-Type': 'application/atom+xml'
		}
	});
};

const renderPost = (post: Post) => `
        <entry>
            <title>${post.title}</title>
            <link rel="alternate" type="text/html" href="${new URL(post.path, baseUrl)}"/>
            <id>${post.path}</id>
            <published>${post.date.toISOString()}</published>
            <updated>${post.date.toISOString()}</updated>
            ${post.categories.map((c) => `<category term="${c}">`).join('\n            ')}
            <content type="html"><![CDATA[${post.default.render()}]]></content>
        </entry>`;

const render = (posts: Post[]) =>
	`<?xml version="1.0" encoding="UTF-8"?>
<feed xmlns="http://www.w3.org/2005/Atom">
    <title>Posts | Nikita Galaiko</title>
    <id>${new URL('/posts', baseUrl)}</id>
    <link rel="alternate" type="text/html" href="${baseUrl}"/>
    <link href="${new URL('/posts.atom', baseUrl)}" rel="self"/>
    <updated>${max(posts.map(({ date }) => date)).toISOString()}</updated>
    <icon>${new URL('favicon.png', baseUrl)}</icon>
    <author>
	    <name>Nikita Galaiko</name>
        <uri>${baseUrl}</uri>
        <email>nikita@galaiko.rocks</email>
	</author>
    ${posts.map(renderPost).join('')}
</feed>`;
