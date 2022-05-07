import type { Author, Reply } from '$lib/webmentions';
import me from '$lib/assets/people/nikita.jpeg?preset=avatar';
import type { RequestHandler } from '@sveltejs/kit';

const range = (start: number, end: number) => {
	const result = [];
	for (let i = start; i <= end; i++) {
		result.push(i);
	}
	return result;
};

const nikita: Author = {
	picture: me.slice(-1)[0].src,
	name: 'Nikita Galaiko',
	url: 'https://galaiko.rocks'
};

const newReply = ({
	id,
	content,
	target,
	timestamp
}: {
	id: string;
	content: string;
	target: string;
	timestamp: Date;
}): Reply & {
	id: string;
} => {
	return {
		id,
		author: nikita,
		content,
		timestamp: timestamp.getTime(),
		source: `/replies/${id}`,
		target
	};
};

const replies: (Reply & { id: string })[] = [
	newReply({
		id: '0',
		content: 'It works!',
		target: 'https://galaiko.rocks/posts/blog/hello-indieweb/',
		timestamp: new Date('2022-04-30')
	}),
	...range(1, 23).map((i) =>
		newReply({
			id: `${i}`,
			content: `Hello! This is a test reply #${i}`,
			target: `https://webmention.rocks/test/${i}`,
			timestamp: new Date('2022-05-01')
		})
	)
];

export const get: RequestHandler = async () => ({
	status: 200,
	body: replies.sort((a, b) => a.timestamp - b.timestamp)
});
