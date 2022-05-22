import { createFilter } from '@rollup/pluginutils';

export default function graphql({ include, exclude } = {}) {
	// path filter
	const filter = createFilter(include, exclude);
	const filterExt = /\.(cook)$/i;

	return {
		name: 'cooklang',
		transform(source, id) {
			if (!filter(id)) return null;
			if (!filterExt.test(id)) return null;
			const code = `import { Parser } from "@cooklang/cooklang-ts"
export default new Parser().parse(\`${source}\`)`;
			const map = { mappings: '' };
			return {
				code,
				map
			};
		}
	};
}
