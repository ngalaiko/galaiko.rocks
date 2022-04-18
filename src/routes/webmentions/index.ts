import type { RequestHandler } from '@sveltejs/kit';
import { create, ValidationError } from '$lib/webmentions';

export const post: RequestHandler = async ({ request, url, platform }) => {
	const contentType = request.headers.get('content-type');
	if (!contentType || !contentType.includes('application/x-www-form-urlencoded'))
		return { status: 415, body: 'Unsupported content type' };

	const body = await request.formData();

	const target = body.get('target') as string;
	const source = body.get('source') as string;

	try {
		const webmention = await create(platform, { source, target });
		const statusUrl = new URL(webmention.id + '/', url);
		return {
			status: 201,
			headers: { location: statusUrl.toString() },
			body: `Created ${statusUrl}`
		};
	} catch (e) {
		if (e instanceof ValidationError) {
			console.log(e.message);
			return { status: 400, body: e.message };
		}
		return { status: 500, body: 'Internal Server Error' };
	}
};
