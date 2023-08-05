import { readFileSync } from 'fs';

export default () => ({
  name: 'vite-b64-plugin',
  transform: (_code: string, id: string) => {
    if (!id.match(/\?b64$/)) return;
    var path = id.replace(/\?b64/, '');
    var data = readFileSync(path, 'base64');
    return `export default '${data}'`;
  }
});
