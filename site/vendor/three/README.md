# three.js, vendored

`three.bundle.min.js` is the part of three.js r186 (npm `three@0.186.1`, MIT, see `LICENSE`)
that `site/scene.js` imports, bundled and minified so the page needs no build step. It is
third-party code, unmodified apart from bundling.

To rebuild it after changing the imports in `scene.js`, list the same names in `entry.mjs` and run,
with `three@0.186.1` and `esbuild@0.28.2` installed in `node_modules`:

```sh
esbuild entry.mjs --bundle --minify --format=esm --target=es2020 --legal-comments=inline \
  --alias:three=node_modules/three/build/three.module.js \
  --alias:three/addons=node_modules/three/examples/jsm \
  --outfile=three.bundle.min.js
```
