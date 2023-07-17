import { readFileSync } from 'fs';

export default () => ({
  name: 'vite-b64-plugin',
  transform: (code: string, id: string) => {
    if (!id.match(/\?b64$/)) return;
    console.log(id, code);
    var path = id.replace(/\?b64/, '');
    var data = readFileSync(path, 'base64');
    return `export default '${data}'`;
  }
});
