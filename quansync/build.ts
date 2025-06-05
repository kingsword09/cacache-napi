import { build } from 'jsr:@deno/dnt@^0.42.1'

if (import.meta.main) {
  await build({
    entryPoints: ['./mod.ts'],
    outDir: './npm',
    shims: {
      deno: true,
    },
    package: {
      name: 'cacache-napi',
      version: Deno.args[0],
      description: 'A cacache wrapper for deno',
      license: 'MIT',
      author: 'Kingsword kingsword09 <kingsword09@gmail.com>',
      repository: {
        type: 'git',
        url: 'git+https://github.com/kingsword09/cacache-napi.git',
      },
      bugs: {
        url: 'https://github.com/kingsword09/cacache-napi/issues',
        email: 'kingsword09@gmail.com',
      },
    },
  })
}
