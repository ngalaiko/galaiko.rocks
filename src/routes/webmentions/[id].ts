import type { RequestHandler } from '@sveltejs/kit';
import { getById as getById, NotFoundError, Status } from '$lib/webmentions';

const statusToCode = (status: Status): number => {
	switch (status) {
		case Status.Created:
			return 201;
		case Status.Accepted:
			return 202;
		case Status.Rejected:
			return 400;
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
