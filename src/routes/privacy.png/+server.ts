import type { RequestHandler } from '@sveltejs/kit';

import { image } from '$lib/og';

export const GET: RequestHandler = async () => {
    const img = await image({ message: 'Privacy Policy' });

    return new Response(img.asPng(), {
        headers: {
            'content-type': 'image/png'
        }
    });
};