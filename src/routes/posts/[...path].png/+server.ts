import type { RequestHandler } from '@sveltejs/kit';
import { findByPathname as postByPathname } from '$lib/posts';

import { image } from '$lib/og';

export const GET: RequestHandler = async ({ url }) => {
  const post = await postByPathname(url.pathname.replace('.png', '/'));
  if (!post) {
    return new Response('Not Found', { status: 404 });
  } else {
    const img = await image({ message: post.title });

    return new Response(img.asPng(), {
      headers: {
        'content-type': 'image/png'
      }
    });
  }
};
