import fs from 'fs';

export default (ext: string[]) => ({
  name: 'vite-plugin-raw-fonts',
  transform: (_source: string, path: string) => {
    if (ext.some((e) => path.endsWith(e))) {
      const buffer = fs.readFileSync(path);
      return { code: `export default ${JSON.stringify(buffer)}`, map: null };
    }
  }
});
