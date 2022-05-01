import type { RequestHandler } from '@sveltejs/kit';
import { getById as getById, NotFoundError, type Webmention } from '$lib/webmentions';

const statusToCode = (status: Webmention['status']): number => {
	switch (status) {
		case 'created':
			return 201;
		case 'accepted':
			return 202;
		case 'rejected':
			return 400;
		case 'removed':
			return 202;
	}
};

export const get: RequestHandler = async ({ params, platform }) => {
	try {
		const webmention = await getById(platform, params.id);
		const status = statusToCode(webmention.status);
		return { status, body: webmention.message ?? '' };
	} catch (e) {
		if (e instanceof NotFoundError) {
			return { status: 404, body: e.message };
		}
		return { status: 500, body: 'Internal Server Error' };
	}
};
