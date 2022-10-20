export default (ext: string[]) => ({
    name: 'cooklang',
    transform: (source: string, path: string) => {
        if (ext.some((e) => path.endsWith(e))) {
            const code = `import { Parser } from "@cooklang/cooklang-ts"
export default new Parser().parse(\`${source}\`)`;
            const map = { mappings: '' };
            return {
                code,
                map
            };
        }
    }
});
