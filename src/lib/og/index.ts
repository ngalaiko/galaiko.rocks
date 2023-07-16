export { default as Card } from './Card.svelte';

import { html as toHTML } from 'satori-html';
import satori from 'satori';
import { Resvg } from '@resvg/resvg-js';

import IowanOldStyle from '$lib/assets/fonts/IowanOldStyle.ttf';
import Card from './Card.svelte';

const height = 630;
const width = 1200;

export const image = async (props: { message?: string } = {}) => {
	const component = Card.render(props);
	const html = toHTML(`${component.html}<style>${component.css.code}</style>`);

	const svg = await satori(html, {
		fonts: [
			{
				name: 'Iowan Old Style',
				data: Buffer.from(IowanOldStyle),
				style: 'normal'
			}
		],
		height,
		width
	});

	const resvg = new Resvg(svg, {
		fitTo: {
			mode: 'width',
			value: width
		}
	});

	return resvg.render();
};
