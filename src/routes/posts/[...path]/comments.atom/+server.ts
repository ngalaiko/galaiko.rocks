import { error, type RequestHandler } from '@sveltejs/kit';
import { compareDesc, max } from 'date-fns';
import { findByPathname as commentsByPathname, type Comment } from '$lib/comments';
import { findByPathname as postsByPathname, type Post } from '$lib/posts';

export const GET: RequestHandler = async ({ url }) => {
    const comments = await commentsByPathname(url.pathname.replace('comments.atom', ''));
    const post = await postsByPathname(url.pathname.replace('comments.atom', ''));
    if (!post) throw error(404, 'not found');
    const body = render(
        url.origin,
        post,
        comments.sort((a, b) => compareDesc(new Date(a.created), new Date(b.created)))
    );
    return new Response(body, {
        headers: {
            'Cache-Control': 'max-age=0, s-maxage=3600',
            'Content-Type': 'application/atom+xml'
        }
    });
};

const renderComment = (baseUrl: string, comment: Comment, index: number) => `
        <entry>
            <title>${comment.authorName} commented</title>
            <link rel="alternate" type="text/html" href="${new URL(comment.pathname, baseUrl)}"/>
            <id>${new URL(index.toString() + '/', new URL(comment.pathname, new URL(baseUrl)))}</id>
            <published>${comment.created.toISOString()}</published>
            <updated>${comment.created.toISOString()}</updated>
            <author>
                <name>${comment.authorName}</name>
            </author>
            <content type="html"><![CDATA[${comment.default.render().html}]]></content>
        </entry>`;

const render = (baseUrl: string, post: Post, comments: Comment[]) =>
    `<?xml version="1.0" encoding="UTF-8"?>
<feed xmlns="http://www.w3.org/2005/Atom">
    <title>Comments for ${post.title}</title>
    <id>${new URL('comments.atom', new URL(post.path, baseUrl))}</id>
    <link rel="alternate" type="text/html" href="${baseUrl}"/>
    <link href="${new URL('/posts.atom', baseUrl)}" rel="self"/>
    <updated>${max([...comments.map(({ created }) => created), 0]).toISOString()}</updated>
    < icon > ${new URL('favicon.png', baseUrl)} </icon>
    ${comments
        .map((post, index, all) => renderComment(baseUrl, post, all.length - index - 1))
        .join('')}
</feed>`;
