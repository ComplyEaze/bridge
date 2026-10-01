// scene.js — Direction C "Flagged" v2: one client file, nine chapters, one
// pinned canvas. The file object never hands off: it opens, its focal page
// changes content (ledger / voucher / bank statement), flags spring on and
// off it, and it shuts again at the close. Every number and label drawn
// here comes from BRIEF.md's synthetic demo book or its V2 ADDENDUM.
//
// Performance notes:
// - named imports only, no three/addons, so esbuild can tree-shake this.
// - textures are generated small and lazily, per chapter, not all at once.
// - no real-time shadow map (30 Sep, TBT pass): the first frame compiled ~14 shadow
//   depth programs synchronously (~100 ms, ~400 ms at 4x CPU) and every frame paid a
//   second render. The desktop floor now carries one baked soft contact shadow
//   (_buildContactShadow); the renderer never enables a shadow map, never toggles it.
// - pixel ratio capped 1.5 (1.25 mobile).
// - the Spring integrator runs fixed 1/120s substeps over real elapsed
//   time (never a slow-motion dt clamp), so it is frame-rate independent.

import {
  Scene, PerspectiveCamera, WebGLRenderer, Group, Mesh, PlaneGeometry, BoxGeometry,
  MeshStandardMaterial, CanvasTexture, Color, DirectionalLight, HemisphereLight,
  PointLight, Vector3, MathUtils, Clock, DoubleSide, SRGBColorSpace,
  ACESFilmicToneMapping, BufferGeometry, Float32BufferAttribute, MeshBasicMaterial, FrontSide,
  MeshPhysicalMaterial, PMREMGenerator, CylinderGeometry, Quaternion, Euler, MultiplyBlending, BackSide,
  RoomEnvironment,
} from './vendor/three/three.bundle.min.js';

// A flat ribbon along a polyline, drawn progressively with setDrawRange: the
// pencil ticks animate as geometry, so scrolling never redraws or re-uploads
// a page texture.
// colorAt (optional, bahi world only): (x, y) of a segment's midpoint -> a Color; it adds a flat per-segment
// vertex colour (the geometry is non-indexed, so each segment owns its six vertices). Absent: no attribute.
function makeRibbon(points, width, subdivide = 8, taper = 0, colorAt = null) {
  const pts = [];
  for (let k = 0; k < points.length - 1; k++) {
    const [ax, ay] = points[k], [bx, by] = points[k + 1];
    for (let j = 0; j < subdivide; j++) pts.push([ax + (bx - ax) * (j / subdivide), ay + (by - ay) * (j / subdivide)]);
  }
  pts.push(points[points.length - 1]);
  const pos = [], col = [];
  const N = pts.length - 1;
  // taper > 0: the stroke starts and ends thin, like a pencil touching down
  const hw = (k) => width * 0.5 * (taper ? 0.2 + 0.8 * Math.min(1, Math.min(k, N - k) / (N * taper)) : 1);
  for (let k = 0; k < N; k++) {
    const [ax, ay] = pts[k], [bx, by] = pts[k + 1];
    const dx = bx - ax, dy = by - ay, len = Math.hypot(dx, dy) || 1;
    const nx = -dy / len, ny = dx / len, a = hw(k), b = hw(k + 1);
    pos.push(ax + nx * a, ay + ny * a, 0, ax - nx * a, ay - ny * a, 0, bx + nx * b, by + ny * b, 0);
    pos.push(bx + nx * b, by + ny * b, 0, ax - nx * a, ay - ny * a, 0, bx - nx * b, by - ny * b, 0);
    if (colorAt) {
      const c = colorAt((ax + bx) / 2, (ay + by) / 2);
      for (let v = 0; v < 6; v++) col.push(c.r, c.g, c.b);
    }
  }
  const geo = new BufferGeometry();
  geo.setAttribute('position', new Float32BufferAttribute(pos, 3));
  if (colorAt) geo.setAttribute('color', new Float32BufferAttribute(col, 3));
  geo.userData.segments = N;
  geo.setDrawRange(0, 0);
  return geo;
}
function setRibbonProgress(geo, t) {
  geo.setDrawRange(0, Math.round(Math.max(0, Math.min(1, t)) * geo.userData.segments) * 6);
}

// ---------------------------------------------------------------- constants

// One object, four material worlds. The page picks its world with
// <html data-theme>; setTheme() must run before the scene is built.
const THEMES = {
  // board values are sRGB; before the colour-space fix the cobalt cover rendered
  // as ~#0e1f8c, so these keep that look
  cobalt: { board: 0x1a2fb0, boardDeep: 0x0c1668, block: 0xeef1f7, paper: '#f6f8fb', edge: '#eef1f7', rule: 'rgba(31,63,191,0.45)', ink: '#0b1436', inkRgb: '11,20,54', floor: 0.5, label: '#fbfbfa', exposure: 0.95, hemi: [0xe8eeff, 0x1a1f33, 0.45], rim: true, tick: '#b3261e',
    // Cobalt: a lighter cloth board against a darker spine and back board, on the ink room (style.css);
    // stationery flags with a cut edge (yellow, pink, green, orange, periwinkle, white); the "?" state in blue
    // pencil (the book's cobalt) on a pale wash, its tick red; the pastedown as board paper (epTone, no lattice); a pen tick on the label (penTick)
    flags: { '40a3': 0xf2d33b, '43bh': 0xee7fa4, '411': 0x74c596, cash: 0xf29a4c, round: 0xa9b8e6, '26as': 0xffffff }, flagEdge: true,
    decideInk: '#1f3fbf', decideText: '#f6f8fb', decideRim: '#f6f8fb', washQuery: '#e3e8fa', washDone: '#f4f5f9', decideSwatch: 'rgba(31,63,191,0.16)',
    epTone: 0x4a5384, penTick: true, mobileFrame: true },
  // bahi-khata ("bahi" is the
  // flag every red-only branch below reads, so cobalt, green and light never run them):
  //   board/boardDeep: the tiled cloth of the back board and spine, darker than the old 0xb3201c (measured median ~#920c06);
  //   cloth*: the front cover's unique texture (colours, weave, stitch, tape); the texture is darker than the
  //   rendered colour because the key light and the tone map lift it ~1.3x;
  //   paper: cream, edge: the aged page-block edge; ink/inkSoft; tick on paper and tickOnCloth on the cloth;
  //   rimColor: the warm rim light that separates cloth from the dark ground
  red: { bahi: true, board: 0x6d120c, boardDeep: 0x4a0a06, block: 0xe9d9b0, paper: '#fbeec9', edge: '#ecdcb4', rule: 'rgba(179,32,28,0.70)', ink: '#2b0e0c', inkRgb: '43,14,12', floor: 0.42, label: '#efe2c0', labelRule: 'rgba(179,32,28,0.85)',
    cloth: '#922811', looseSheet: '#ebc985', clothShadow: '#4a0906', clothRub: '#b86a58', clothHi: '#a8321f', clothLo: '#6a0f0b', turnIn: '#6d120c', thread: '#ece0c2', threadLit: '#fdefc7', tape: '#cdbf9e', rimColor: 0xc4553f,
    // muted stationery tabs (cut card, ink lettering), one distinct colour per finding type; 26AS is the outlined off-white one
    flags: { '40a3': 0xefd96a, '43bh': 0xd9a08c, '411': 0xb9c4a0, cash: 0xe3cf9f, round: 0xa9b6c9, '26as': 0xf4ecd8 }, flagEdge: true, flagOutlineFill: '#f4ecd8',
    // the closing tick is a tapered blue-black ink stroke on the label (the reviewer's pen; printed text is brown-black)
    closeTick: '#1e2b4f',
    // the till-roll slip is a warmer thermal stock in the red world (cobalt's #fdfdfb renders grey among the warm creams)
    receiptPaper: '#f3e9d2',
    // the phone reframing with the red book's own values ([dolly, dy, dx] per keyframe 1-7, as MOBILE_FRAME)
    mobileFrame: [null, [0.639, 0.054, 0.13], [0.46, 0.679, -0.26], [0.67, -0.133, -0.16], [0.486, 0.26, -0.423], [0.649, 0.24, -0.141], [0.7, 0.15, -0.2], [0.8, 0.35, 0]],
    // the chapter-3 "decide" state in blue-black instead of cyan; wash* are multiply factors (10% ink over the paper, and #f3e6c4 over the paper)
    decideInk: '#1e2b4f', decideText: '#fbeec9', decideRim: '#fbeec9', washQuery: '#e9eaf0', washDone: '#f7f6f9', decideSwatch: 'rgba(30,43,79,0.16)',
    exposure: 0.95, hemi: [0xfff1e6, 0x2a1210, 0.45], rim: false, tick: '#b3261e' },
  // green ledger: buckram-green cloth, oxblood spine with gold bands, green-ruled paper
  green: { board: 0x1f6b47, boardDeep: 0x14472f, block: 0xe6efd9, paper: '#eaf3de', edge: '#e3edd4', rule: 'rgba(76,140,90,0.55)', ink: '#15261c', inkRgb: '21,38,28', floor: 0.4, label: '#f3f8ec', labelRule: 'rgba(31,107,71,0.75)', spine: 0x5a1a16, bands: true, exposure: 0.95, hemi: [0xeef6f0, 0x16201a, 0.45], rim: false, tick: '#b3261e' },
  // light tint: pale room, the cobalt file is the only saturated mass
  light: { board: 0x0b168a, boardDeep: 0x06106a, block: 0xeef1f7, paper: '#fbfcfe', edge: '#eef1f7', rule: 'rgba(31,63,191,0.45)', ink: '#0b1436', inkRgb: '11,20,54', floor: 0.3, label: '#fbfbfa', outlineInk: '11,20,54', exposure: 1.1, hemi: [0xffffff, 0xc9d3e6, 0.8], rim: false, tick: '#b3261e' },
};
// hybrid: the cobalt page with only the book swapped for the red bahi. The page is cobalt's (CSS has no hybrid rules); in the
// scene the book (cloth, stitch, niwar, cream pages, red-world faces, closing tick) is red's, and everything that is
// the finding-type colour law (flag palette, the cyan decide state) is cobalt's.
THEMES.hybrid = { ...THEMES.red, flags: null, flagEdge: false, flagOutlineFill: null, decideInk: null, decideText: null, washQuery: null, washDone: null, decideSwatch: null };
let THEME = THEMES.cobalt;
export function setTheme(name) {
  THEME = THEMES[name] || THEMES.cobalt;
  flagTextureCache.clear(); // flag textures are drawn in the theme's palette
}

// The colour law. One colour per finding kind, never decorative. Cyan is
// new in v2: "needs follow-up" for an ask-the-books result, not a clause.
export const FLAG_GROUPS = [
  { key: '40a3', color: 0xf7e733, count: 7, label: '40A(3)' },
  { key: '43bh', color: 0xff5fa2, count: 4, label: '43B(h)' },
  { key: '411', color: 0x3ddc84, count: 5, label: '41(1)' },
  { key: 'cash', color: 0xff8a1f, count: 3, flagTag: 'CASH' },
  { key: 'round', color: 0xa78bfa, count: 6, flagTag: '\u20b9 ROUND' },
  { key: '26as', color: 0xffffff, count: 1, label: '26AS', outline: true },
];
// cyan is reserved for an ask-the-books follow-up (the "?" near-miss flags,
// scene.ensureBank): a distinct meaning from every clause colour above and
// from the white/ink "resolved" tick.
const CYAN = 0x37d5f0;

export const CHAPTER_COUNT = 8;

// the hero flag's lettering, in px on screen, when it lands in the working
// paper's clause cell (app.js draws that cell's chip at the same size)
const HERO_FONT = 13;
// flag plane: 0.276 x 0.0665 world, lettering 44 px of a 448 x 108 canvas
const RECEIPT_SCALE = 1.15;
// camera time-warp amplitude (speed stays within 1 -/+ WARP_A of the mean) and the
// hold node after keyframe 2: the camera has only crept by HOLD2.f of the way to the
// statement by 2.85, so the working paper's band under the voucher stays clear until it leaves
const WARP_A = 0.35;
// a slow vertical sway added to the path (view translation, cam and look together):
// +-SWAY world units (~9 px at 1440x900), one cycle per chapter, fastest exactly at the
// keyframes, where the warp and the path's reversals are slowest, so the file is never
// at rest whatever the keyframes do (about 0.06 px per scrolled px there)
const SWAY = 0.0;
// [dolly along the view ray, look-point dy, dx] per keyframe 1-6, solved so the open page spans about 260 x 360 px of the
// 390 x 444 canvas box at 1.8, 2.8, 4.4 and 5.5 (and 3.72 and 6.5, narrower, to leave the lifted slips and the receipt their room).
// [6] keeps the slip's right edge at or under 374 px at 6.5 and the page's foot at the box's foot; [7] (the closed file) is closer and lower, so the
// words under it are not a hand's breadth away. dy is a world-space shift of camera and look together: a larger dy puts the book LOWER on screen.
const MOBILE_FRAME = [null, [0.639, 0.054, 0.13], [0.46, 0.679, -0.26], [0.67, -0.133, -0.16], [0.486, 0.195, -0.423], [0.649, 0.163, -0.141], [0.7, 0.15, -0.2], [0.8, 0.35, 0]];
// the object actions' windows, each on a sine ease (the cover on a flatter one: trapEase) so no action
// packs a large move into a few dozen px of scroll (peak = 1.57 x the mean slope)
const COVER = { open0: 1.12, open1: 1.60, shut0: 7.04, shut1: 7.46, r: 0.2 }; // r: trapEase's soft ends (peak 1 / (1 - r) of the mean)
// The cover's corners do not move at a constant screen speed per unit of angle (the swing is
// fastest where the board passes edge-on), so the angle is not eased directly: the ease drives the
// corners' path length on screen, and these tables (measured at 1440x900 from the scene's own
// corner projection, 48 steps of the angle, cumulative path / total) turn that back into an angle.
// Peak screen speed is then the ease's peak x the mean, not 1.5-2 x more.
const COVER_ARC_OPEN = [0, 0.0156, 0.0285, 0.0428, 0.0594, 0.0768, 0.096, 0.1161, 0.1386, 0.1623, 0.1877, 0.2155, 0.2416, 0.2709, 0.2999, 0.3298, 0.3606, 0.3923, 0.4229, 0.4547, 0.4875, 0.5183, 0.5477, 0.5799, 0.6094, 0.6382, 0.6659, 0.693, 0.7179, 0.7432, 0.7652, 0.7859, 0.8047, 0.8224, 0.8387, 0.8519, 0.8634, 0.8744, 0.8837, 0.8912, 0.8978, 0.9042, 0.9118, 0.92, 0.9286, 0.9395, 0.9507, 0.9662, 1];
const COVER_ARC_SHUT = [0, 0.013, 0.0229, 0.0317, 0.039, 0.0464, 0.0518, 0.0579, 0.0649, 0.0728, 0.0826, 0.0952, 0.1074, 0.1234, 0.1405, 0.1606, 0.1809, 0.2056, 0.2288, 0.2564, 0.2838, 0.314, 0.3438, 0.3766, 0.4071, 0.4392, 0.4716, 0.5048, 0.5381, 0.5714, 0.6015, 0.6349, 0.6664, 0.6974, 0.7274, 0.7559, 0.783, 0.809, 0.8336, 0.858, 0.8809, 0.9008, 0.9188, 0.9366, 0.9526, 0.9669, 0.9792, 0.9901, 1];
function arcInv(tab, u) {
  const n = tab.length - 1;
  if (u <= 0) return 0;
  if (u >= 1) return 1;
  let k = 0;
  while (k < n - 1 && tab[k + 1] <= u) k++;
  return (k + (u - tab[k]) / (tab[k + 1] - tab[k])) / n;
}
const TURN_LEN = [0.3, 0.36]; // the page turns into chapters 2 and 3 (2.04-2.34, 3.10-3.46)
const PUSH_V = 1.0; // the close's push-in, world units per chapter, after 7.35
const SLIP_WINDOW = [3.4, 3.72]; // the five slips lift; the chapter-3 counter rides this
const SLIP_STAG = 0.03, SLIP_DUR = 0.19; // 4 x 0.03 + 0.19 = 0.31 <= the window
const HOLD2 = { from: 2, t: 2.85, f: 0.08, dx: 0.16 }; // dx: the view also slides right, so the page glides left ~45 px while it waits
const FLAG_W = 0.276, FLAG_LETTER = (44 / 448) * FLAG_W;
// the three ledger names decided in chapter 3, one after another (3.72-3.88),
// in the statement's row order; the chips, the 3D query flags and the cyan
// row washes all read this one function, so they can never disagree
export const nameResolve = (p, k) => smoothstep(3.72 + 0.04 * k, 3.8 + 0.04 * k, p);
export const slipWindow = (p) => smoothstep(SLIP_WINDOW[0], SLIP_WINDOW[1], p);
// constant-speed travel with short soft ends: peak speed 1 / (1 - r) of the mean
function trapEase(t, r = 0.2) {
  t = clamp01(t);
  const v = 1 / (1 - r);
  if (t < r) return (v * t * t) / (2 * r);
  if (t > 1 - r) return 1 - (v * (1 - t) * (1 - t)) / (2 * r);
  return v * (t - r / 2);
}

// deterministic PRNG: every capture and every viewer sees the same file
function mulberry32(seed) {
  return function () {
    seed |= 0;
    seed = (seed + 0x6d2b79f5) | 0;
    let t = Math.imul(seed ^ (seed >>> 15), 1 | seed);
    t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
}
function clamp01(v) {
  return v < 0 ? 0 : v > 1 ? 1 : v;
}
function smoothstep(e0, e1, x) {
  const t = clamp01((x - e0) / (e1 - e0));
  return t * t * (3 - 2 * t);
}
function sineInOut(t) {
  return -(Math.cos(Math.PI * clamp01(t)) - 1) / 2;
}
const linear01 = clamp01;

// ---------------------------------------------------------------- geometry

function makeFlagGeometry(w = 0.276, h = 0.066, foldFrac = 0.3) {
  const geo = new PlaneGeometry(w, h, 5, 1);
  const pos = geo.attributes.position;
  const creaseX = -w / 2 + w * (1 - foldFrac);
  const angle = -0.72;
  for (let i = 0; i < pos.count; i++) {
    const x = pos.getX(i);
    if (x > creaseX) {
      const local = x - creaseX;
      pos.setX(i, creaseX + Math.cos(angle) * local);
      pos.setZ(i, pos.getZ(i) + Math.sin(angle) * local);
    }
  }
  geo.computeVertexNormals();
  return geo;
}

// A cover board with a chamfered rim: front and back caps inset by b, joined
// to the walls by 45-degree facets. UVs run 0..1 across the board in x/y, so
// the cloth tiles like it did on the plain box.
function makeBoard(w, h, d, b) {
  const hw = w / 2, hh = h / 2, hd = d / 2;
  const ring = (x, y, z) => [[-x, -y, z], [x, -y, z], [x, y, z], [-x, y, z]];
  const fi = ring(hw - b, hh - b, hd), fo = ring(hw, hh, hd - b), bo = ring(hw, hh, -hd + b), bi = ring(hw - b, hh - b, -hd);
  const pos = [], uv = [];
  // each quad is wound outward: the solid is convex, so its centroid points out
  const quad = (p, q, r, s) => {
    const cx = p[0] + q[0] + r[0] + s[0], cy = p[1] + q[1] + r[1] + s[1], cz = p[2] + q[2] + r[2] + s[2];
    const ux = q[0] - p[0], uy = q[1] - p[1], uz = q[2] - p[2], vx = r[0] - p[0], vy = r[1] - p[1], vz = r[2] - p[2];
    const out = (uy * vz - uz * vy) * cx + (uz * vx - ux * vz) * cy + (ux * vy - uy * vx) * cz >= 0;
    (out ? [p, q, r, p, r, s] : [p, r, q, p, s, r]).forEach((v) => {
      pos.push(v[0], v[1], v[2]);
      uv.push(v[0] / w + 0.5, v[1] / h + 0.5);
    });
  };
  quad(fi[0], fi[1], fi[2], fi[3]);
  quad(bi[0], bi[1], bi[2], bi[3]);
  for (let k = 0; k < 4; k++) {
    const j = (k + 1) % 4;
    quad(fi[k], fi[j], fo[j], fo[k]);
    quad(bi[k], bi[j], bo[j], bo[k]);
    quad(fo[k], fo[j], bo[j], bo[k]);
  }
  const geo = new BufferGeometry();
  geo.setAttribute('position', new Float32BufferAttribute(pos, 3));
  geo.setAttribute('uv', new Float32BufferAttribute(uv, 2));
  geo.computeVertexNormals();
  return geo;
}

// ---------------------------------------------------------------- textures
// Kept small and drawn once per chapter (never per frame): this is the
// single biggest main-thread cost cut from the first build.

function makeClothTexture(hex) {
  const c = document.createElement('canvas');
  c.width = c.height = 256;
  const ctx = c.getContext('2d');
  ctx.fillStyle = '#' + new Color(hex).getHexString(); // sRGB, not the linear .r/.g/.b
  ctx.fillRect(0, 0, 256, 256);
  // a plain weave in 8px cells (about two screen pixels each), warp threads
  // pale and weft threads dark, at ~3% contrast: cloth up close, flat from afar
  // (bahi worlds: no lattice on the back board and spine either; the grain below is all they get)
  for (let j = 0; j < (THEME.bahi ? 0 : 32); j++) {
    for (let i = 0; i < 32; i++) {
      const warp = (i + j) & 1;
      ctx.fillStyle = warp ? 'rgba(255,255,255,0.04)' : 'rgba(0,0,0,0.05)';
      if (warp) ctx.fillRect(i * 8 + 2, j * 8, 4, 8);
      else ctx.fillRect(i * 8, j * 8 + 2, 8, 4);
    }
  }
  const rnd = mulberry32(211);
  for (let i = 0; i < 1400; i++) {
    ctx.fillStyle = rnd() > 0.5 ? 'rgba(255,255,255,0.03)' : 'rgba(0,0,0,0.04)';
    ctx.fillRect(rnd() * 256, rnd() * 256, 2 + rnd() * 3, 0.8);
  }
  const tex = new CanvasTexture(c);
  tex.colorSpace = SRGBColorSpace;
  tex.wrapS = tex.wrapT = 1000; // RepeatWrapping
  tex.repeat.set(6, 6);
  tex.anisotropy = 4;
  return tex;
}

// ---- the bahi world's textures (red theme only). Each is drawn once when the file is built; nothing here is
// called per frame.

const hexRgb = (hex) => {
  const n = parseInt(hex.slice(1), 16);
  return `${(n >> 16) & 255},${(n >> 8) & 255},${n & 255}`;
};
// the cover's design space, in px; the canvas is this times `scale` (1 desktop, 0.75 phones)
const BAHI_W = 1024, BAHI_H = 1354; // = the 1.55 x 2.05 board

// Used, dark madder cloth with a fine matte weave, rubbed lighter at edges and corners; cream machine
// stitching in slanting rows across the whole cover; and a plain woven tape along the cover edges, all in one
// unique, non-repeating texture: the stitching is sewn through the cover, so it must not tile.
// the cloth is fine grain + sparse slubs under a raking gradient (no tile lattice); the stitch
// is sewn thread lying in a groove; the niwar is a ribbed tape that wraps the edge.

// thread along `path`: a groove pulled into the cloth (centred on the thread, no offset), the thread, then its lit core.
// A stitch of length d and gap g drawn with round caps of width w is laid out as a run of d - w and a break of g + w.
function sewThread(ctx, path, w, grooveA) {
  ctx.lineCap = 'round';
  ctx.lineJoin = 'round';
  ctx.strokeStyle = `rgba(40,4,2,${grooveA})`;
  ctx.lineWidth = 4.5;
  ctx.stroke(path);
  ctx.strokeStyle = THEME.thread;
  ctx.lineWidth = w;
  ctx.stroke(path);
  ctx.strokeStyle = THEME.threadLit;
  ctx.globalAlpha = 0.7;
  ctx.lineWidth = 0.9;
  ctx.stroke(path);
  ctx.globalAlpha = 1;
}
function makeBahiCoverTexture(scale) {
  const c = document.createElement('canvas');
  c.width = Math.round(BAHI_W * scale);
  c.height = Math.round(BAHI_H * scale);
  const ctx = c.getContext('2d');
  ctx.scale(scale, scale);
  const W = BAHI_W, H = BAHI_H, rnd = mulberry32(3307);
  const rub = hexRgb(THEME.clothRub), shade = hexRgb(THEME.clothShadow);
  ctx.fillStyle = THEME.cloth;
  ctx.fillRect(0, 0, W, H);
  // raking light: #a8321f at the upper spine-side corner to #6a0f0b at the lower fore-edge corner, at half
  // strength, so the face's median stays where it was
  const rake = ctx.createLinearGradient(0, 0, W, H);
  rake.addColorStop(0, THEME.clothHi);
  rake.addColorStop(1, THEME.clothLo);
  ctx.globalAlpha = 0.5;
  ctx.fillStyle = rake;
  ctx.fillRect(0, 0, W, H);
  ctx.globalAlpha = 1;
  // uneven dye and fade: a few very large, very soft patches ("used-looking", not grungy)
  for (let i = 0; i < 9; i++) {
    const x = rnd() * W, y = rnd() * H, r = 170 + rnd() * 260, lit = rnd() > 0.72;
    const g = ctx.createRadialGradient(x, y, 0, x, y, r);
    g.addColorStop(0, lit ? `rgba(${rub},0.04)` : `rgba(${shade},0.14)`);
    g.addColorStop(1, 'rgba(0,0,0,0)');
    ctx.fillStyle = g;
    ctx.fillRect(x - r, y - r, 2 * r, 2 * r);
  }
  // fine grain at half resolution (drawn up 2x, so ~2 design px), about +-2.5% luminance, no repeating tile;
  // then sparse slubs, 20-60 px long at 3-5% opacity
  const nw = 512, nh = Math.round((nw * H) / W);
  const nc = document.createElement('canvas');
  nc.width = nw;
  nc.height = nh;
  const nctx = nc.getContext('2d');
  const img = nctx.createImageData(nw, nh), px = img.data;
  for (let i = 0; i < px.length; i += 4) {
    const n = rnd() + rnd() - 1; // triangular, -1..1
    px[i] = px[i + 1] = px[i + 2] = n > 0 ? 255 : 0;
    px[i + 3] = Math.abs(n) * (n > 0 ? 13 : 16);
  }
  nctx.putImageData(img, 0, 0);
  ctx.imageSmoothingEnabled = true;
  ctx.drawImage(nc, 0, 0, W, H);
  for (let i = 0; i < 70; i++) {
    const len = 20 + rnd() * 40, x = rnd() * W, y = rnd() * H, a = 0.03 + rnd() * 0.02;
    ctx.fillStyle = rnd() > 0.5 ? `rgba(255,225,205,${a.toFixed(3)})` : `rgba(20,0,0,${(a * 1.3).toFixed(3)})`;
    if (rnd() > 0.3) ctx.fillRect(x, y, len, 1.4 + rnd()); else ctx.fillRect(x, y, 1.4 + rnd(), len);
  }

  // Rubbing. Edges and, more, corners desaturate toward brick-pink (#b86a58), kept to a soft 18% at the rim
  const band = 90;
  [[0, 0, 0, band, 0, 0, W, band], [0, H, 0, H - band, 0, H - band, W, band], [0, 0, band, 0, 0, 0, band, H], [W, 0, W - band, 0, W - band, 0, band, H]].forEach(([x0, y0, x1, y1, rx, ry, rw, rh]) => {
    const g = ctx.createLinearGradient(x0, y0, x1, y1);
    g.addColorStop(0, `rgba(${rub},0.22)`);
    g.addColorStop(1, `rgba(${rub},0)`);
    ctx.fillStyle = g;
    ctx.fillRect(rx, ry, rw, rh);
  });
  [[0, 0], [W, 0], [0, H], [W, H]].forEach(([x, y], k) => {
    const r = k > 1 ? 210 : 160, g = ctx.createRadialGradient(x, y, 0, x, y, r);
    g.addColorStop(0, `rgba(${rub},0.18)`); // was 0.42, which read as an airbrushed glow; wear is the hard scuffs drawn below
    g.addColorStop(1, `rgba(${rub},0)`);
    ctx.fillStyle = g;
    ctx.fillRect(x - r, y - r, 2 * r, 2 * r);
  });

  // the stitching, as sewn thread. Slanting rows 73 px apart horizontally (7.1% of the width, so a horizontal cut
  // crosses 14 of them); each is a run of stitches (~10 px, gaps 1.2-1.8 px, about 6:1) of 2.2 px thread in a 4.5 px groove,
  // with no offset shadow. The slope drifts slowly row to row (0.36 +- 0.05) and each row wanders a little, so neighbours
  // converge and diverge by ~12 px and no two rows are copies. The cloth puffs between rows: a faint lit line half a pitch away.
  const pitch = 73, TH = 2.2, STEP = 1;
  const seat = new Path2D(), chan = new Path2D(), puff = new Path2D();
  for (let i = -7; i < 23; i++) {
    // hand-guided rows, not ruled lines: a 12-20 px wave on a 280-420 px wavelength, a free phase per row, and +-8 px of
    // seeded spacing jitter, so neighbours converge and diverge (the worst gap is 73 - 2 x 20 - 16 = 17 px: rows never cross)
    const slant = 0.36 + 0.05 * Math.sin(i * 0.33 + 1.1), ph = i * 0.45 + rnd() * 6.28, amp = 12 + rnd() * 8, lam = 280 + rnd() * 140, bow = 0.014 + 0.004 * Math.sin(i * 0.3);
    const x0 = i * pitch + 6 + (rnd() - 0.5) * 16;
    const xAt = (yy) => { const u = (yy - H / 2) / H; return x0 + slant * (yy - H / 2) + bow * H * u * u * 4 + amp * Math.sin((yy / lam) * 2 * Math.PI + ph); };
    if (xAt(0) < -60 && xAt(H) < -60) continue;
    if (xAt(0) > W + 60 && xAt(H) > W + 60) continue;
    let y = -30, on = true, left = 10 * (0.4 + rnd() * 0.6), open = false;
    chan.moveTo(xAt(y), y);
    puff.moveTo(xAt(y) + pitch / 2, y);
    while (y < H + 30) {
      const ny = y + STEP, x = xAt(y), nx = xAt(ny), ds = Math.hypot(nx - x, STEP);
      if (on) {
        if (!open) { seat.moveTo(x, y); open = true; }
        seat.lineTo(nx, ny);
      }
      chan.lineTo(nx, ny);
      puff.lineTo(nx + pitch / 2, ny);
      left -= ds;
      if (left <= 0) {
        on = !on;
        if (!on) open = false;
        left = on ? Math.max(1, (10 - TH) * (0.85 + rnd() * 0.3)) : TH + 1.5 * (0.8 + rnd() * 0.4);
      }
      y = ny;
    }
  }
  // the same paths, drawn once more as height (mid-grey cloth, a shallow channel along the row, a deeper groove under
  // each stitch and the thread standing proud of it) for the material's bumpMap, so the rows catch the light as the cover turns
  const bump = document.createElement('canvas');
  bump.width = Math.round(BAHI_W * scale * 0.5);
  bump.height = Math.round(BAHI_H * scale * 0.5);
  const bctx = bump.getContext('2d');
  bctx.scale(scale * 0.5, scale * 0.5);
  bctx.fillStyle = '#808080';
  bctx.fillRect(0, 0, W, H);
  bctx.lineCap = bctx.lineJoin = 'round';
  bctx.strokeStyle = '#6c6c6c';
  bctx.lineWidth = 2.6;
  bctx.stroke(chan);
  bctx.strokeStyle = '#3a3a3a';
  bctx.lineWidth = 4.5;
  bctx.stroke(seat);
  bctx.strokeStyle = '#e0e0e0';
  bctx.lineWidth = TH;
  bctx.stroke(seat);
  ctx.lineCap = 'round';
  ctx.strokeStyle = 'rgba(255,220,200,0.05)'; // the puffed cloth between rows
  ctx.lineWidth = 1;
  ctx.stroke(puff);
  ctx.strokeStyle = 'rgba(40,4,2,0.10)'; // the faint channel the thread has pulled along the whole row
  ctx.lineWidth = 2.6;
  ctx.stroke(chan);
  sewThread(ctx, seat, TH, 0.35);

  // niwar, a plain undyed woven tape, 21 px wide, wrapping the edge: its colour runs to texture px 0, which is
  // what the board's straight side walls sample, so the board's thickness is tape-coloured too, and the chamfer (px 0-5)
  // catches a little light. Lengthwise ribs every 2 px at 6%; one sewn line 4 px inside the tape's inner edge; corners
  // mitred and slightly soiled. No coloured piping.
  const TW = 21;
  ctx.fillStyle = THEME.tape;
  ctx.fillRect(0, 0, W, TW); ctx.fillRect(0, H - TW, W, TW); ctx.fillRect(0, 0, TW, H); ctx.fillRect(W - TW, 0, TW, H);
  ctx.strokeStyle = 'rgba(60,40,15,0.06)';
  ctx.lineWidth = 1;
  ctx.beginPath();
  for (let k = 1; k < TW; k += 2) {
    ctx.moveTo(TW, k); ctx.lineTo(W - TW, k);
    ctx.moveTo(TW, H - k); ctx.lineTo(W - TW, H - k);
    ctx.moveTo(k, TW); ctx.lineTo(k, H - TW);
    ctx.moveTo(W - k, TW); ctx.lineTo(W - k, H - TW);
  }
  ctx.stroke();
  ctx.strokeStyle = 'rgba(255,248,225,0.06)';
  ctx.beginPath();
  for (let k = 0; k < TW; k += 2) {
    ctx.moveTo(TW, k); ctx.lineTo(W - TW, k);
    ctx.moveTo(TW, H - k); ctx.lineTo(W - TW, H - k);
    ctx.moveTo(k, TW); ctx.lineTo(k, H - TW);
    ctx.moveTo(W - k, TW); ctx.lineTo(W - k, H - TW);
  }
  ctx.stroke();
  ctx.save();
  ctx.beginPath();
  ctx.rect(0, 0, W, H);
  ctx.rect(TW, TW, W - 2 * TW, H - 2 * TW);
  ctx.clip('evenodd');
  [[0, 0], [W, 0], [0, H], [W, H]].forEach(([x, y]) => {
    const g = ctx.createRadialGradient(x, y, 0, x, y, 90);
    g.addColorStop(0, 'rgba(70,45,20,0.20)');
    g.addColorStop(1, 'rgba(70,45,20,0)');
    ctx.fillStyle = g;
    ctx.fillRect(x - 90, y - 90, 180, 180);
  });
  ctx.restore();
  ctx.strokeStyle = 'rgba(255,246,222,0.22)'; // the chamfer (texture px 0-5) catches light along the wrapped edge
  ctx.lineWidth = 4;
  ctx.strokeRect(2.5, 2.5, W - 5, H - 5);
  ctx.strokeStyle = 'rgba(28,2,1,0.4)'; // where the tape's inner edge meets the cloth
  ctx.lineWidth = 2;
  ctx.strokeRect(TW + 1, TW + 1, W - 2 * TW - 2, H - 2 * TW - 2);
  const sew = new Path2D();
  sew.rect(TW - 4, TW - 4, W - 2 * TW + 8, H - 2 * TW + 8);
  ctx.setLineDash([10 - TH, 1.5 + TH]);
  sewThread(ctx, sew, TH, 0.3);
  bctx.setLineDash([10 - TH, 1.5 + TH]);
  bctx.strokeStyle = '#3a3a3a';
  bctx.lineWidth = 4.5;
  bctx.stroke(sew);
  bctx.strokeStyle = '#e0e0e0';
  bctx.lineWidth = TH;
  bctx.stroke(sew);
  ctx.setLineDash([]);
  ctx.strokeStyle = 'rgba(40,22,6,0.2)';
  ctx.lineWidth = 1.2;
  ctx.beginPath();
  [[0, 0, TW, TW], [W, 0, W - TW, TW], [0, H, TW, H - TW], [W, H, W - TW, H - TW]].forEach(([a, b, d, e]) => { ctx.moveTo(a, b); ctx.lineTo(d, e); });
  ctx.stroke();

  // wear as a few small hard-edged scuffs at each corner (brick-pink where the cloth is rubbed through, dull grey
  // where the board shows through the tape), seeded, replacing the soft halo
  const scuff = (x, y, r, col) => {
    ctx.fillStyle = col;
    ctx.beginPath();
    for (let v = 0; v < 5; v++) {
      const a = (v / 5) * 6.283 + rnd() * 0.9, rr = r * (0.55 + rnd() * 0.6);
      if (v) ctx.lineTo(x + Math.cos(a) * rr * 1.4, y + Math.sin(a) * rr); else ctx.moveTo(x + Math.cos(a) * rr * 1.4, y + Math.sin(a) * rr);
    }
    ctx.closePath();
    ctx.fill();
  };
  [[0, 0, 1, 1], [W, 0, -1, 1], [0, H, 1, -1], [W, H, -1, -1]].forEach(([cx, cy, sx, sy]) => {
    for (let k = 0; k < 2; k++) scuff(cx + sx * (6 + rnd() * 34), cy + sy * (6 + rnd() * 12), 1.6 + rnd() * 2, 'rgba(112,100,84,0.8)');
    for (let k = 0; k < 3; k++) scuff(cx + sx * (TW + 4 + rnd() * 44), cy + sy * (TW + 4 + rnd() * 44), 1.6 + rnd() * 2.6, `rgba(${rub},0.85)`);
  });

  const tex = new CanvasTexture(c);
  tex.colorSpace = SRGBColorSpace;
  tex.anisotropy = 8;
  const bumpTex = new CanvasTexture(bump);
  bumpTex.anisotropy = 8;
  return { map: tex, bump: bumpTex };
}

// A sheet of the bahi's blank paper, cream, aged toward the edges, and creased (not inked) into eight equal
// columns (sal) as a paired shadow + highlight line, so the fold reads without competing with any printed page.
// Horizontal ruling is left out (traditional books are plain).
function drawBahiFolds(ctx, w, h, shadowA, lightA) {
  for (let k = 1; k < 8; k++) {
    const x = Math.round((w * k) / 8);
    ctx.fillStyle = `rgba(107,74,34,${shadowA})`;
    ctx.fillRect(x, 0, 1, h);
    ctx.fillStyle = `rgba(255,255,255,${lightA})`;
    ctx.fillRect(x + 1, 0, 1, h);
  }
}
function makeBahiLooseTexture() {
  const c = document.createElement('canvas');
  c.width = 512;
  c.height = 640;
  const ctx = c.getContext('2d');
  ctx.fillStyle = THEME.looseSheet;
  ctx.fillRect(0, 0, 512, 640);
  const rnd = mulberry32(7);
  ctx.fillStyle = `rgba(${THEME.inkRgb},0.05)`;
  for (let i = 0; i < 700; i++) ctx.fillRect(rnd() * 512, rnd() * 640, 1, 1);
  drawBahiFolds(ctx, 512, 640, 0.16, 0.45);
  agePaperRect(ctx, 512, 640, 36, 0.16);
  const tex = new CanvasTexture(c);
  tex.colorSpace = SRGBColorSpace;
  return tex;
}
// The page block's aged edge (#ecdcb4) also fades a little into every printed page, inside the margins
function agePaperRect(ctx, w, h, band, a) {
  const rgb = '150,108,48';
  [[0, 0, band, 0, 0, 0, band, h], [w, 0, w - band, 0, w - band, 0, band, h], [0, 0, 0, band, 0, 0, w, band], [0, h, 0, h - band, 0, h - band, w, band]].forEach(([x0, y0, x1, y1, rx, ry, rw, rh]) => {
    const g = ctx.createLinearGradient(x0, y0, x1, y1);
    g.addColorStop(0, `rgba(${rgb},${a})`);
    g.addColorStop(1, `rgba(${rgb},0)`);
    ctx.fillStyle = g;
    ctx.fillRect(rx, ry, rw, rh);
  });
}
// the pasted-down lining of the open cover: lit paper, #eadcb6 at the fore-edge to #bfa983 at the hinge (never
// darker than #a08a66), inside a turn-in of red cloth (#6d120c, ~7% of the width) where the cover cloth folds over the board's
// edge. The lining is flat: no creases (a real pastedown has none). Drawn unlit (MeshBasicMaterial, no tone map), so these
// are the colours on screen.
function drawBahiEndpaper(ctx, w, h) {
  const tx = Math.round(w * 0.07), ty = Math.round(tx * (w / 1.5) / (h / 2)); // the same ~0.105 world units on all four sides
  ctx.fillStyle = THEME.turnIn;
  ctx.fillRect(0, 0, w, h);
  const rnd = mulberry32(41);
  for (let i = 0; i < 900; i++) { ctx.fillStyle = rnd() > 0.5 ? 'rgba(255,225,205,0.05)' : 'rgba(20,0,0,0.07)'; ctx.fillRect(rnd() * w, rnd() * h, 1 + rnd() * 3, 1); }
  const g = ctx.createLinearGradient(tx, 0, w - tx, 0); // the right end is the hinge side
  // flat to within 4% (was #eadcb6 to #bfa983, a 16% ramp that the left-edge fade turned into a brass highlight band)
  g.addColorStop(0, '#ddcea6');
  g.addColorStop(1, '#d7c79f');
  ctx.fillStyle = g;
  ctx.fillRect(tx, ty, w - 2 * tx, h - 2 * ty);
  ctx.strokeStyle = 'rgba(30,6,3,0.4)'; // the paper's edge lying on the cloth
  ctx.lineWidth = 1;
  ctx.strokeRect(tx + 0.5, ty + 0.5, w - 2 * tx - 1, h - 2 * ty - 1);
  for (let i = 0; i < 500; i++) { ctx.fillStyle = `rgba(120,90,40,${(0.02 + rnd() * 0.04).toFixed(3)})`; ctx.fillRect(tx + rnd() * (w - 2 * tx), ty + rnd() * (h - 2 * ty), 1, 1); }
}
// the page leaf's outer 2.4% is warm, never cooler than the interior: #d9c08e at the very edge into #e4cfa2
function warmPageEdge(ctx, w, h) {
  const B = Math.round(w * 0.024);
  [[0, 0, B, 0, 0, 0, B, h], [w, 0, w - B, 0, w - B, 0, B, h], [0, 0, 0, B, 0, 0, w, B], [0, h, 0, h - B, 0, h - B, w, B]].forEach(([x0, y0, x1, y1, rx, ry, rw, rh]) => {
    const g = ctx.createLinearGradient(x0, y0, x1, y1);
    g.addColorStop(0, 'rgba(217,192,142,1)');
    g.addColorStop(0.3, 'rgba(228,207,162,0.85)');
    g.addColorStop(1, 'rgba(228,207,162,0)');
    ctx.fillStyle = g;
    ctx.fillRect(rx, ry, rw, rh);
  });
}

function makeEdgeStripeTexture() {
  const c = document.createElement('canvas');
  c.width = 64;
  c.height = 512;
  const ctx = c.getContext('2d');
  ctx.fillStyle = THEME.edge;
  ctx.fillRect(0, 0, 64, 512);
  const rnd = mulberry32(53);
  let y = 3;
  while (y < 508) {
    ctx.fillStyle = `rgba(${THEME.inkRgb},${(0.05 + rnd() * 0.12).toFixed(3)})`;
    ctx.fillRect(0, y, 64, rnd() < 0.12 ? 1.4 : 0.8);
    y += 2.2 + rnd() * 2.2;
  }
  const tex = new CanvasTexture(c);
  tex.colorSpace = SRGBColorSpace;
  return tex;
}

function makePlainPaperTexture() {
  const c = document.createElement('canvas');
  c.width = 128;
  c.height = 160;
  const ctx = c.getContext('2d');
  ctx.fillStyle = '#f3f5f9';
  ctx.fillRect(0, 0, 128, 160);
  const rnd = mulberry32(7);
  ctx.fillStyle = `rgba(${THEME.inkRgb},0.05)`;
  for (let i = 0; i < 260; i++) ctx.fillRect(rnd() * 128, rnd() * 160, 1, 1);
  ctx.strokeStyle = `rgba(${THEME.inkRgb},0.08)`;
  for (let y = 12; y < 150; y += 14) {
    ctx.beginPath();
    ctx.moveTo(8, y);
    ctx.lineTo(120, y);
    ctx.stroke();
  }
  const tex = new CanvasTexture(c);
  tex.colorSpace = SRGBColorSpace;
  return tex;
}

// The paper label glued flat to the cover, drawn once. The closing tick is
// geometry (a ribbon over the label), not a redraw of this canvas.
function drawCoverLabel(ctx) {
  ctx.clearRect(0, 0, 640, 220);
  ctx.fillStyle = 'rgba(6,10,26,0.28)';
  ctx.fillRect(9, 13, 622, 200);
  ctx.fillStyle = THEME.label;
  ctx.fillRect(0, 0, 616, 196);
  if (THEME.labelRule) {
    ctx.strokeStyle = THEME.labelRule;
    ctx.lineWidth = 3;
    ctx.strokeRect(10, 10, 596, 176);
    ctx.lineWidth = 1.5;
    ctx.strokeRect(17, 17, 582, 162);
  }
  ctx.strokeStyle = `rgba(${THEME.inkRgb},0.16)`;
  ctx.lineWidth = 2;
  ctx.strokeRect(1, 1, 614, 194);
  // three lines, like a CA's own file label: entity, the audit and its year,
  // then the voucher period, with an ink rule closing off the entity line
  ctx.fillStyle = THEME.ink;
  ctx.font = '700 28px "Bricolage Grotesque Variable", sans-serif';
  ctx.fillText('Demo Traders Pvt Ltd (synthetic)', 26, 56);
  ctx.strokeStyle = `rgba(${THEME.inkRgb},0.55)`;
  ctx.lineWidth = 2;
  ctx.beginPath();
  ctx.moveTo(26, 72);
  ctx.lineTo(590, 72);
  ctx.stroke();
  ctx.font = '450 24px "Geist Mono Variable", monospace';
  ctx.fillStyle = `rgba(${THEME.inkRgb},0.75)`;
  ctx.fillText('Tax audit, F.Y. 2025-26 (A.Y. 2026-27)', 26, 112);
  ctx.font = '450 24px "Geist Mono Variable", monospace';
  ctx.fillStyle = `rgba(${THEME.inkRgb},0.75)`;
  ctx.fillText('Vouchers, Apr to Mar', 26, 148);
}

// A small, crisp glyph shared by every flag of one kind: colour + label
// (or a custom short string, for the query flags). The canvas is 448x108, the
// same 4.15:1 as the 0.30 x 0.072 flag plane, so lettering is not stretched.
// The left ~70% is the flat strip (label right-aligned to its end, before the
// crease); the right 30% is the folded tab. `tick` draws a stroked ink check
// instead of text (Geist Mono has no check glyph).
const flagTextureCache = new Map();
function makeFlagTexture(key, hex, text, { outline = false, tick = false, textColor = null, rim = null } = {}) {
  if (flagTextureCache.has(key)) return flagTextureCache.get(key);
  const c = document.createElement('canvas');
  c.width = 448;
  c.height = 108;
  const ctx = c.getContext('2d');
  const h8 = new Color(hex).getHex(); // sRGB
  const rgb = `${(h8 >> 16) & 255},${(h8 >> 8) & 255},${h8 & 255}`;
  const inkRgb = outline && THEME.outlineInk ? THEME.outlineInk : THEME.inkRgb;
  // solid paper: a translucent flag reads as a render glitch on a dark ground
  ctx.fillStyle = outline ? (THEME.flagOutlineFill || (THEME.outlineInk ? '#f2f4f9' : '#ffffff')) : `rgb(${rgb})`;
  ctx.fillRect(0, 0, 448, 108);
  if (THEME.flagEdge && !outline) {
    // cut card: a 1 px (3 canvas px) ink-at-20% edge and a little tooth
    const rt = mulberry32(5 + key.length);
    for (let i = 0; i < 260; i++) { ctx.fillStyle = rt() > 0.5 ? 'rgba(255,255,255,0.10)' : `rgba(${THEME.inkRgb},0.06)`; ctx.fillRect(rt() * 448, rt() * 108, 2 + rt() * 3, 1.4); }
    ctx.strokeStyle = `rgba(${THEME.inkRgb},0.2)`;
    ctx.lineWidth = 3;
    ctx.strokeRect(1.5, 1.5, 445, 105);
  }
  if (outline) {
    ctx.strokeStyle = `rgba(${inkRgb},0.85)`;
    ctx.lineWidth = 6;
    ctx.strokeRect(5, 5, 438, 98);
  }
  if (rim) {
    // a pale paper-tone outline, so a blue-black flag keeps its shape against the dark ground
    ctx.strokeStyle = rim;
    ctx.lineWidth = 9;
    ctx.strokeRect(4.5, 4.5, 439, 99);
  }
  ctx.fillStyle = textColor || `rgba(${inkRgb},0.9)`;
  if (tick) {
    ctx.strokeStyle = `rgb(${inkRgb})`;
    ctx.lineWidth = 13;
    ctx.lineCap = ctx.lineJoin = 'round';
    ctx.beginPath();
    ctx.moveTo(214, 58);
    ctx.lineTo(242, 84);
    ctx.lineTo(296, 26);
    ctx.stroke();
  } else {
    ctx.font = '700 44px "Geist Mono Variable", monospace';
    ctx.textAlign = 'right';
    ctx.textBaseline = 'middle';
    ctx.fillText(text, 300, 56);
  }
  // contact: a darker strip where the flag slips under the pages
  const g = ctx.createLinearGradient(0, 0, 26, 0);
  g.addColorStop(0, 'rgba(0,0,0,0.34)');
  g.addColorStop(1, 'rgba(0,0,0,0)');
  ctx.fillStyle = g;
  ctx.fillRect(0, 0, 26, 108);
  const tex = new CanvasTexture(c);
  tex.colorSpace = SRGBColorSpace;
  tex.anisotropy = 4;
  flagTextureCache.set(key, tex);
  return tex;
}
// ---------------------------------------------------------------- focal page
// One plane, one 1024px-wide canvas, redrawn only on a mode change or a
// quantised tick step (never on every scroll pixel). The voucher face is
// kept exactly as proven: large type, ~34-68px on this 1024px canvas.

// shared with _buildFile: where the pencil ticks and the presented 40A(3)
// flag land, so the geometry and the drawn face never drift apart
const VOUCHER_ROW_Y = 410;
const VOUCHER_NARRATION_Y = 568;
// the two pencil ticks: the amount row (in the gap after the payee's name),
// and the narration text (just past the end of its second line)
const VOUCHER_TICKS = [[640, VOUCHER_ROW_Y], [610, VOUCHER_NARRATION_Y + 74]];

function drawVoucherFace(ctx, w, h) {
  ctx.clearRect(0, 0, w, h);
  ctx.fillStyle = THEME.paper;
  ctx.fillRect(0, 0, w, h);
  if (THEME.bahi) { agePaperRect(ctx, w, h, 54, 0.12); warmPageEdge(ctx, w, h); } // the aged page edge
  const rnd = mulberry32(41);
  ctx.fillStyle = `rgba(${THEME.inkRgb},0.04)`;
  for (let i = 0; i < 700; i++) ctx.fillRect(rnd() * w, rnd() * h, 1, 1);

  const L = 72, R = w - 72, ink = THEME.ink, soft = `rgba(${THEME.inkRgb},0.66)`, rule = `rgba(${THEME.inkRgb},0.22)`;
  const hr = (y, a = rule, lw = 2) => {
    ctx.strokeStyle = a;
    ctx.lineWidth = lw;
    ctx.beginPath();
    ctx.moveTo(L, y);
    ctx.lineTo(R, y);
    ctx.stroke();
  };
  // header, laid out like a TallyPrime voucher print: title, then No./Dated
  // on one line, then the account the voucher is drawn on
  ctx.fillStyle = ink;
  ctx.font = '750 68px "Bricolage Grotesque Variable", sans-serif';
  ctx.fillText('Payment Voucher', L, 132);
  ctx.font = '450 30px "Geist Mono Variable", monospace';
  ctx.fillStyle = soft;
  ctx.fillText('No. 1187', L, 192);
  ctx.textAlign = 'right';
  ctx.fillText('Dated 14-Nov-2025', R, 192);
  ctx.textAlign = 'left';
  ctx.font = '450 28px "Geist Mono Variable", monospace';
  ctx.fillStyle = soft;
  ctx.fillText('Account: Cash', L, 234);
  hr(270, `rgba(${THEME.inkRgb},0.5)`, 3);

  // body: Particulars | Amount, exactly as a voucher print reads it
  ctx.font = '450 24px "Geist Mono Variable", monospace';
  ctx.fillStyle = soft;
  ctx.fillText('Particulars', L, 326);
  ctx.textAlign = 'right';
  ctx.fillText('Amount', R, 326);
  ctx.textAlign = 'left';
  hr(344, `rgba(${THEME.inkRgb},0.35)`, 2);

  ctx.font = '500 40px "Geist Variable", sans-serif';
  ctx.fillStyle = ink;
  ctx.fillText('Demo Packaging Supplies', L, VOUCHER_ROW_Y); // fictional by construction: the earlier payee name matched real shops
  ctx.font = '600 40px "Geist Mono Variable", monospace';
  ctx.textAlign = 'right';
  ctx.fillText('₹18,400.00', R, VOUCHER_ROW_Y);
  ctx.textAlign = 'left';
  hr(VOUCHER_ROW_Y + 30, `rgba(${THEME.inkRgb},0.14)`, 1.5);

  ctx.font = '450 26px "Geist Mono Variable", monospace';
  ctx.fillStyle = soft;
  ctx.fillText('Narration:', L, VOUCHER_NARRATION_Y);
  ctx.font = '500 34px "Geist Variable", sans-serif';
  ctx.fillStyle = ink;
  ctx.fillText('Cash purchase of packing', L, VOUCHER_NARRATION_Y + 46);
  ctx.fillText('material for godown dispatch', L, VOUCHER_NARRATION_Y + 90); // revenue, unambiguously

  hr(730, `rgba(${THEME.inkRgb},0.5)`, 3);
  ctx.font = '700 38px "Geist Mono Variable", monospace';
  ctx.fillStyle = ink;
  ctx.fillText('Total', L, 782);
  ctx.textAlign = 'right';
  ctx.fillText('₹18,400.00', R, 782);
  ctx.textAlign = 'left';

  // the print footer: the amount in words, then the two signature lines
  hr(822, `rgba(${THEME.inkRgb},0.14)`, 1.5);
  ctx.font = '450 24px "Geist Mono Variable", monospace';
  ctx.fillStyle = soft;
  ctx.fillText('Amount (in words):', L, 868);
  ctx.font = '500 36px "Geist Variable", sans-serif';
  ctx.fillStyle = ink;
  ctx.fillText('Rupees Eighteen Thousand Four Hundred Only', L, 918);

  // a pencilled audit note, not typed: the 40A(3) breach is in the single
  // payment itself, and the same-day aggregate is a second, separate line
  ctx.save();
  ctx.translate(L, 1024);
  ctx.rotate((-3 * Math.PI) / 180);
  ctx.font = 'italic 500 30px "Geist Variable", sans-serif';
  ctx.fillStyle = THEME.tick;
  ctx.fillText('Cash > ₹10,000 to one payee/day: 40A(3)', 0, 0);
  ctx.fillText('+ Pay 1188 ₹6,200, same payee & day: ₹24,600', 0, 44);
  ctx.restore();

  ctx.font = '450 22px "Geist Mono Variable", monospace';
  ctx.fillStyle = soft;
  ctx.textAlign = 'right';
  ctx.fillText('for Demo Traders Pvt Ltd (synthetic)', R, 1150);
  ctx.textAlign = 'left';
  ctx.strokeStyle = `rgba(${THEME.inkRgb},0.5)`;
  ctx.lineWidth = 2;
  ctx.beginPath(); ctx.moveTo(L, 1212); ctx.lineTo(L + 330, 1212); ctx.moveTo(R - 330, 1212); ctx.lineTo(R, 1212); ctx.stroke();
  ctx.font = '450 24px "Geist Mono Variable", monospace';
  ctx.fillText("Receiver's Signature", L, 1252);
  ctx.textAlign = 'right';
  ctx.fillText('Authorised Signatory', R, 1252);
  ctx.textAlign = 'left';
}

// A bills-receivable ageing as at 31 Mar 2026: party, then the amount in
// whichever bucket its oldest bill falls (0-30 / 31-60 / 61-90 / over 90 days).
// The five over-90 parties and figures are the ones the rest of the page
// quotes (the cash supplier in chapter 2 is a different invented name,
// Demo Packaging Supplies). The other six parties are invented, generic and young.
// Check: over 90 = 2,14,000 + 1,92,500 + 64,800 + 1,70,000 + 1,45,000 = 7,86,300.
const ASK_ROWS = [
  ['Ardhan General Stores', '58,200', '', '', ''],
  ['Gauravel Dairy Products', '', '76,400', '', ''],
  ['Jaimeet Enterprises', '41,300', '', '29,900', ''],
  ['Kesarvan Textiles', '', '', '', '1,70,000'],
  ['Nevrika Agro Foods', '', '', '', '2,14,000'],
  ['Padmira Packaging', '', '', '', '1,45,000'],
  ['Rudhvan Distributors', '', '', '', '64,800'],
  ['Shravika Steels', '', '', '', '1,92,500'],
  ['Tamrapath Agencies', '1,12,500', '34,000', '', ''],
  ['Ushnal Traders', '96,700', '22,150', '', ''],
  ['Vasmira Fabrics', '', '', '67,800', ''],
];
const ASK_TOTALS = ['3,08,700', '1,32,550', '97,700', '7,86,300']; // column sums; grand total 13,25,250
const ASK_HEAD = ['0-30', '31-60', '61-90', '>90'];
const ASK_COL_X = [502, 652, 802, 952]; // right edges of the four amount columns
const ASK_Y0 = 318, ASK_PITCH = 70, ASK_TOTAL_Y = ASK_Y0 + ASK_PITCH * ASK_ROWS.length + 6;
// the six pencil rings: the five over-90 figures, then their total
const ASK_RINGS = ASK_ROWS.map((r, i) => (r[4] ? [ASK_COL_X[3] - 58, ASK_Y0 + i * ASK_PITCH - 9, 82, 26] : null))
  .filter(Boolean)
  .concat([[ASK_COL_X[3] - 58, ASK_TOTAL_Y - 9, 82, 27]]);

// a hand-drawn ring in canvas px: it overshoots its start a little, drifts
// outward as it goes round, and wobbles by a couple of pixels
function penRing(cx, cy, rx, ry, seed) {
  const rnd = mulberry32(seed);
  const a0 = -2.4 + rnd() * 0.5, n = 30, out = [];
  for (let k = 0; k <= n; k++) {
    const t = k / n, a = a0 + t * Math.PI * 2.16, d = 1 + 0.07 * t;
    out.push([cx + Math.cos(a) * rx * d + (rnd() - 0.5) * 3.5, cy + Math.sin(a) * ry * d + (rnd() - 0.5) * 3.5]);
  }
  return out;
}

function drawLedgerFace(ctx, w, h) {
  ctx.clearRect(0, 0, w, h);
  ctx.fillStyle = THEME.paper;
  ctx.fillRect(0, 0, w, h);
  if (THEME.bahi) { agePaperRect(ctx, w, h, 54, 0.12); warmPageEdge(ctx, w, h); } // the aged page
  const L = 72, R = w - 72, ink = THEME.ink, soft = `rgba(${THEME.inkRgb},0.66)`;
  ctx.fillStyle = ink;
  ctx.font = '750 56px "Bricolage Grotesque Variable", sans-serif';
  ctx.fillText('Bills Receivable Ageing', L, 128);
  ctx.font = '400 28px "Geist Variable", sans-serif';
  ctx.fillStyle = soft;
  ctx.fillText('As at 31 Mar 2026, amounts in ₹', L, 172);

  const partyFont = '600 24px "Geist Variable", sans-serif';
  const numFont = '500 24px "Geist Mono Variable", monospace';
  ctx.font = '450 20px "Geist Mono Variable", monospace';
  ctx.fillStyle = soft;
  ctx.fillText('Party', L, 254);
  ctx.textAlign = 'right';
  ASK_HEAD.forEach((t, k) => ctx.fillText(t, ASK_COL_X[k], 254));
  ctx.textAlign = 'left';
  ctx.strokeStyle = `rgba(${THEME.inkRgb},0.5)`;
  ctx.lineWidth = 3;
  ctx.beginPath(); ctx.moveTo(L, 276); ctx.lineTo(R, 276); ctx.stroke();
  if (THEME.bahi) {
    // the one ruled column. Red at 70% separates the party names from the four amount columns (the first
    // fold of a bahi holds the amount), midway between the widest name and the widest first-column figure
    ctx.font = partyFont;
    const nameEnd = L + Math.max(...ASK_ROWS.map((r) => ctx.measureText(r[0]).width));
    ctx.font = numFont;
    const figStart = ASK_COL_X[0] - Math.max(...ASK_ROWS.map((r) => (r[1] ? ctx.measureText(r[1]).width : 0)), ctx.measureText(ASK_TOTALS[0]).width);
    const rx = Math.round((nameEnd + figStart) / 2);
    ctx.strokeStyle = THEME.rule;
    ctx.lineWidth = 2;
    ctx.beginPath(); ctx.moveTo(rx, 230); ctx.lineTo(rx, ASK_TOTAL_Y + 18); ctx.stroke();
  }

  ASK_ROWS.forEach((row, i) => {
    const y = ASK_Y0 + i * ASK_PITCH;
    ctx.font = partyFont;
    ctx.fillStyle = ink;
    ctx.fillText(row[0], L, y);
    ctx.font = numFont;
    ctx.textAlign = 'right';
    for (let k = 0; k < 4; k++) {
      if (!row[k + 1]) continue;
      ctx.fillStyle = ink;
      ctx.fillText(row[k + 1], ASK_COL_X[k], y);
    }
    ctx.textAlign = 'left';
    ctx.strokeStyle = `rgba(${THEME.inkRgb},0.12)`;
    ctx.lineWidth = 1.5;
    ctx.beginPath(); ctx.moveTo(L, y + 24); ctx.lineTo(R, y + 24); ctx.stroke();
  });
  ctx.strokeStyle = `rgba(${THEME.inkRgb},0.5)`;
  ctx.lineWidth = 3;
  ctx.beginPath(); ctx.moveTo(L, ASK_TOTAL_Y - 42); ctx.lineTo(R, ASK_TOTAL_Y - 42); ctx.stroke();
  ctx.font = '700 24px "Geist Mono Variable", monospace';
  ctx.fillStyle = ink;
  ctx.fillText('Total', L, ASK_TOTAL_Y);
  ctx.textAlign = 'right';
  ASK_TOTALS.forEach((t, k) => ctx.fillText(t, ASK_COL_X[k], ASK_TOTAL_Y));
  ctx.textAlign = 'left';
}

// a realistic, fictional bank statement: Date | Narration | Chq/Ref No. | Value Dt |
// Withdrawal | Deposit | Balance. The 12 Mar row is the exact line the
// approval dialog posts (₹1,24,600 from Nevrika Agro Foods), so the two never
// contradict each other. Three rows carry the near-miss narrations that
// the names card ("3 names, your decision") points at: row 3 (Kavyarth Stationery, a supplier who is not in the ageing), row
// 5 (Nevrika Agro Food, a near miss of the book's Nevrika Agro Foods; the dialog
// posts it only after that name is decided) and row 9 (Sample Bank A/c: a
// transfer to the client's second bank account, a contra, not a self-payment
// - the statement itself is Example Bank Ltd's). Running balance is arithmetic, not random.
const BANK_ROWS = [
  { date: '03 Mar', narration: 'NEFT DR-VAYUNATH ELECTRICALS', ref: 'N26030301', dr: '27,500.00', cr: '', bal: '9,22,500.00' },
  { date: '05 Mar', narration: 'CHQ DEP-SHRAVIKA STL', ref: '004512', dr: '', cr: '42,500.00', bal: '9,65,000.00' },
  { date: '07 Mar', narration: 'SALARY FEB 2026', ref: 'SAL0226', dr: '65,000.00', cr: '', bal: '9,00,000.00' },
  { date: '09 Mar', narration: 'UPI-KAVYARTH STATIONERY', ref: '606812447103', dr: '12,300.00', cr: '', bal: '8,87,700.00', query: 'kavyarth' },
  { date: '11 Mar', narration: 'NEFT CR-KESARVAN TEX', ref: 'N26031102', dr: '', cr: '38,750.00', bal: '9,26,450.00' },
  { date: '12 Mar', narration: 'NEFT CR-NEVRIKA AGRO FOOD', ref: 'N26031204', dr: '', cr: '1,24,600.00', bal: '10,51,050.00', query: 'nevrika' },
  { date: '14 Mar', narration: 'NEFT DR-GST PAYMENT', ref: 'N26031401', dr: '22,000.00', cr: '', bal: '10,29,050.00' },
  { date: '17 Mar', narration: 'NEFT CR-KESARVAN TEX', ref: 'N26031701', dr: '', cr: '71,300.00', bal: '11,00,350.00' },
  { date: '19 Mar', narration: 'BANK CHARGES', ref: '-', dr: '590.00', cr: '', bal: '10,99,760.00' },
  { date: '21 Mar', narration: 'NEFT DR-SAMPLE BANK A/C', ref: 'N26032101', dr: '50,000.00', cr: '', bal: '10,49,760.00', query: 'samplebank' },
  { date: '24 Mar', narration: 'NEFT CR-SHRAVIKA STL', ref: 'N26032401', dr: '', cr: '54,200.00', bal: '11,03,960.00' },
];
// The page shows eight consecutive rows (09 to 24 Mar, so every running
// balance follows from the row above it, and all three query rows are on it);
// the other three of BANK_ROWS are not drawn. Row i of the page is
// BANK_ROWS[BANK_SHOWN_FROM + i]; the flags and ticks are placed from that.
const BANK_SHOWN_FROM = 3, BANK_SHOWN = 8;
const BANK_ROW_Y0 = 340, BANK_ROW_PITCH = 108;

function drawBankFace(ctx, w, h) {
  ctx.clearRect(0, 0, w, h);
  ctx.fillStyle = THEME.paper;
  ctx.fillRect(0, 0, w, h);
  if (THEME.bahi) { agePaperRect(ctx, w, h, 54, 0.12); warmPageEdge(ctx, w, h); } // the aged page
  const L = 32, R = w - 56, ink = THEME.ink, soft = `rgba(${THEME.inkRgb},0.66)`;
  const rows = BANK_ROWS.slice(BANK_SHOWN_FROM, BANK_SHOWN_FROM + BANK_SHOWN);
  ctx.fillStyle = ink;
  ctx.font = '750 54px "Bricolage Grotesque Variable", sans-serif';
  ctx.fillText('Example Bank Ltd', L, 128);
  ctx.font = '400 26px "Geist Variable", sans-serif';
  ctx.fillStyle = soft;
  ctx.fillText('Statement of account (synthetic), Mar 2026, 142 lines', L, 170);
  ctx.strokeStyle = `rgba(${THEME.inkRgb},0.5)`;
  ctx.lineWidth = 3;
  ctx.beginPath(); ctx.moveTo(L, 206); ctx.lineTo(R, 206); ctx.stroke();

  // Narration (its date set small beneath it) | Withdrawal | Deposit |
  // Balance. Column widths are measured from the longest cell in each column;
  // the mono size is the largest (26px down to 20px) at which the four
  // columns fit with a gap.
  const gap = 22;
  const widths = (fs) => {
    ctx.font = `500 ${fs}px "Geist Mono Variable", monospace`;
    const mw = (f) => Math.max(...rows.map((r) => ctx.measureText(f(r)).width));
    return [mw((r) => r.narration), mw((r) => r.dr || '0.00'), mw((r) => r.cr || '0.00'), mw((r) => r.bal)];
  };
  let fs = 26, cw = widths(fs);
  while (cw.reduce((a, b) => a + b, 0) + gap * 3 > R - L && fs > 20) cw = widths(--fs);
  const balEndX = R, crEndX = balEndX - cw[3] - gap, drEndX = crEndX - cw[2] - gap;

  ctx.font = '600 18px "Geist Mono Variable", monospace';
  ctx.fillStyle = soft;
  ctx.fillText('Narration / date', L, 240);
  ctx.textAlign = 'right';
  ctx.fillText('Withdrawal', drEndX, 240);
  ctx.fillText('Deposit', crEndX, 240);
  ctx.fillText('Balance', balEndX, 240);
  ctx.textAlign = 'left';
  ctx.strokeStyle = `rgba(${THEME.inkRgb},0.4)`;
  ctx.lineWidth = 2;
  ctx.beginPath(); ctx.moveTo(L, 258); ctx.lineTo(R, 258); ctx.stroke();
  if (THEME.bahi) {
    // the one ruled column, red at 70%, between the narration and the first figure column
    const rx = Math.round((L + cw[0] + drEndX - cw[1]) / 2);
    ctx.strokeStyle = THEME.rule;
    ctx.lineWidth = 2;
    ctx.beginPath(); ctx.moveTo(rx, 222); ctx.lineTo(rx, BANK_ROW_Y0 + (BANK_SHOWN - 1) * BANK_ROW_PITCH + 58); ctx.stroke();
  }

  rows.forEach((row, i) => {
    const y = BANK_ROW_Y0 + i * BANK_ROW_PITCH;
    // the three names Bridge will not guess wear a pale cyan bar (cyan = a ledger
    // name still to be decided, as on the "?" flags): a wash plane per row on the
    // page (ensureBank) fades it to paper as each name is decided, so it is not baked here
    ctx.font = `500 ${fs}px "Geist Mono Variable", monospace`;
    ctx.fillStyle = ink;
    ctx.fillText(row.narration, L, y);
    ctx.font = '450 20px "Geist Mono Variable", monospace';
    ctx.fillStyle = soft;
    ctx.fillText(row.date, L, y + 30);
    ctx.font = `500 ${fs}px "Geist Mono Variable", monospace`;
    ctx.textAlign = 'right';
    ctx.fillStyle = soft;
    if (row.dr) ctx.fillText(row.dr, drEndX, y);
    if (row.cr) ctx.fillText(row.cr, crEndX, y);
    ctx.fillStyle = ink;
    ctx.fillText(row.bal, balEndX, y);
    ctx.textAlign = 'left';
    ctx.strokeStyle = `rgba(${THEME.inkRgb},0.12)`;
    ctx.lineWidth = 1.5;
    ctx.beginPath(); ctx.moveTo(L, y + 58); ctx.lineTo(R, y + 58); ctx.stroke();
  });
  const endY = BANK_ROW_Y0 + (BANK_SHOWN - 1) * BANK_ROW_PITCH;
  ctx.font = '450 22px "Geist Mono Variable", monospace';
  ctx.fillStyle = soft;
  ctx.fillText('8 of 142 lines shown', L, endY + 92);
  ctx.fillStyle = THEME.decideSwatch || 'rgba(55,213,240,0.4)';
  ctx.fillRect(L, endY + 132, 44, 24);
  ctx.fillStyle = soft;
  ctx.fillText('ledger name to decide', L + 60, endY + 152);
}

// ---------------------------------------------------------------- till roll

// The slip is a piece of the product's egress log, agent-egress.jsonl: the
// receipt written when Bridge prepared its reply to the post_import call that
// just ran. Field names are the shipped EgressReceipt struct (0.3.0,
// src-tauri/src/agent_delivery.rs:5-21); only four are shown, at a size a
// reader can read; the values are synthetic and the hash is cut short with an
// ellipsis.
const RECEIPT_ROWS = [
  ['tool', 'post_import'],
  ['ts', '2026-04-02T08:32:05.114Z'],
  ['redaction_preset', 'none'],
  ['response_sha256', 'a91f2c7e90\u2026'],
];
function drawReceiptFace(ctx, w, h) {
  ctx.clearRect(0, 0, w, h);
  // a till roll, not a grey card: cream paper with a zigzag tear at the
  // bottom edge, drawn as one filled outline so the tear is part of the shape.
  // Everything outside the outline stays transparent (the material alpha-tests it).
  const tearY = h - 26;
  ctx.beginPath();
  ctx.moveTo(0, tearY);
  const teeth = Math.round(w / 16);
  for (let i = 0; i <= teeth; i++) {
    const x = (i / teeth) * w;
    ctx.lineTo(x, tearY + (i % 2 === 0 ? -7 : 7));
  }
  ctx.lineTo(w, 0);
  ctx.lineTo(0, 0);
  ctx.closePath();
  ctx.fillStyle = THEME.receiptPaper || '#fdfdfb';
  ctx.fill();

  const ink = THEME.ink, soft = `rgba(${THEME.inkRgb},0.72)`, M = 30;
  ctx.font = '600 34px "Geist Mono Variable", monospace';
  ctx.fillStyle = ink;
  ctx.fillText('agent-egress.jsonl', M, 70);
  ctx.strokeStyle = `rgba(${THEME.inkRgb},0.25)`;
  ctx.setLineDash([6, 6]);
  ctx.lineWidth = 2;
  ctx.beginPath(); ctx.moveTo(M - 4, 96); ctx.lineTo(w - M + 4, 96); ctx.stroke();
  let y = 158;
  RECEIPT_ROWS.forEach(([k, v]) => {
    ctx.font = '450 33px "Geist Mono Variable", monospace';
    ctx.fillStyle = soft;
    ctx.fillText(k, M, y);
    // the value shrinks to fit the slip (the real timestamp is UTC RFC 3339 with milliseconds: 24 characters)
    let fs = 35;
    ctx.font = `650 ${fs}px "Geist Mono Variable", monospace`;
    while (ctx.measureText(v).width > w - 2 * M && fs > 22) ctx.font = `650 ${--fs}px "Geist Mono Variable", monospace`;
    ctx.fillStyle = ink;
    ctx.fillText(v, M, y + 44);
    y += 108;
  });
  // one plain-English line for a reader who cannot parse the log fields; it
  // says only what the log itself is (the egress receipt of the post call, in
  // the app's data folder), never a field the receipt does not record
  // two lines, each shrunk to the slip's inner width like the values above (one line at 27px ran past the edge and read
  // "kept on your"); the wording is unchanged, only broken after the middle dot
  ctx.fillStyle = soft;
  ['Receipt for the post call \u00b7', 'kept on your computer'].forEach((line, i) => {
    let ffs = 27;
    ctx.font = `500 ${ffs}px "Geist Variable", sans-serif`;
    while (ctx.measureText(line).width > w - 2 * M && ffs > 16) ctx.font = `500 ${--ffs}px "Geist Variable", sans-serif`;
    ctx.fillText(line, M, y - 22 + i * 30);
  });
}

// ---------------------------------------------------------------- spring
// stiffness 170 / damping 18 / mass 1, exactly the motion-grammar constants.
// Fixed 1/120s substeps over real elapsed time: no dt clamp slows the sim,
// so all 26 flags are out within ~1.2s even at very low frame rates.
export class Spring {
  constructor(stiffness = 170, damping = 18, mass = 1) {
    this.k = stiffness;
    this.c = damping;
    this.m = mass;
    this.pos = 0;
    this.vel = 0;
    this.target = 0;
    this._acc = 0;
  }
  update(realDt) {
    const h = 1 / 120;
    this._acc += Math.max(0, realDt);
    let steps = Math.floor(this._acc / h);
    if (steps > 240) steps = 240; // safety cap after e.g. a backgrounded tab
    this._acc -= steps * h;
    for (let s = 0; s < steps; s++) {
      const force = -this.k * (this.pos - this.target) - this.c * this.vel;
      this.vel += (force / this.m) * h;
      this.pos += this.vel * h;
    }
    return this.pos;
  }
}

// ---------------------------------------------------------------- scene

const faceAt = (q) => (q < 1 ? 'voucher' : q < 2 ? 'ledger' : q < 3 ? 'voucher' : 'bank');

export class FileScene {
  constructor(canvas, { mobile = false, externalTicker = false } = {}) {
    this.canvas = canvas;
    this.mobile = mobile;
    this.externalTicker = externalTicker;
    this.clock = new Clock();
    this.pointer = { x: 0, y: 0 };
    this.pointerTarget = { x: 0, y: 0 };
    this._lookHero = new Vector3();
    this._lookClose = new Vector3();
    this._look = new Vector3();

    const renderer = new WebGLRenderer({ canvas, antialias: true, alpha: true, powerPreference: 'high-performance' });
    renderer.setPixelRatio(Math.min(window.devicePixelRatio || 1, mobile ? 1.25 : 1.5));
    renderer.outputColorSpace = SRGBColorSpace;
    renderer.toneMapping = ACESFilmicToneMapping;
    renderer.toneMappingExposure = THEME.exposure;
    this.renderer = renderer;

    const scene = new Scene();
    this.scene = scene;

    const camera = new PerspectiveCamera(mobile ? 40 : 36, 1, 0.1, 20);
    this.camera = camera;
    this.camHero = new Vector3(1.85, 0.95, 4.4);
    this.camClose = new Vector3(0.95, 0.35, 5.65);
    this._lookHero.set(0.65, -0.05, 0);
    this._lookClose.set(0.82, -0.02, 0.32);
    this.baseRotY = -0.46;
    this.closeRotY = -0.08;
    camera.position.copy(this.camHero);
    camera.lookAt(this._lookHero);

    this._buildCameraPath();

    // scratch objects for the per-frame hero-flag travel (no allocation there)
    this._v0 = new Vector3();
    this._v1 = new Vector3();
    this._v2 = new Vector3();
    this._v3 = new Vector3();
    this._qa = new Quaternion();
    this._qb = new Quaternion();
    this._qc = new Quaternion();
    this._eul = new Euler();
    this._rowPx = { x: 0, y: 0, ok: false }; // #propRow centre, canvas CSS px (app.js sets it)
    this._cw = 1;
    this._ch = 1;

    this._buildLights();
    this._buildFile();

    this._built = new Set(['core']);
    this._entranceDone = false;
    this._openAmount = 0;
    this._pageMode = 'none';
    this._elapsed = 0;
    this._raf = null;
    this._running = false;
    this._lastQuant = {};
  }

  // Camera grammar. ONE continuous path: a
  // centripetal Catmull-Rom through the eight keyframes (cam, look and exposure
  // alike), keyframe i at chapterPos i + 0.35, driven by the time-warp
  // u(p) = p - (0.7 / 2pi) sin(2pi (p - 0.35)). The warp's slope stays between
  // 0.3 and 1.7 of the mean, so the camera is never at rest. Before 0.35 (the
  // hero's own push) and after 7.35 (the close) the path is extended with a
  // velocity-matched Hermite and a decaying drift. Built once as cubic
  // coefficients; _camAt evaluates them with no allocation.
  _makeKeyframes() {
    const E = THEME.exposure;
    const V = (x, y, z) => new Vector3(x, y, z);
    const heroEnd = V(1.6, 0.8, 4.15);
    // mobile answers the first scroll with a bigger push
    const heroK0 = this.camHero.clone().lerp(heroEnd, this.mobile ? 1.2 : 0.6);
    const kfs = [
      { cam: heroK0, look: this._lookHero }, // 0 hero, after its push
      { cam: V(1.15, 0.45, 4.9), look: V(0.5, 0.1, 0.32) }, // 1 ask the books (the file leaves room above and below for the question and the answer)
      { cam: V(1.0, 0.1, 6.4), look: V(0.5, -0.5, 0.32) }, // 2 scrutiny / voucher: in the upper ~62% of the frame, the working paper in the band under it
      { cam: V(1.2, 0.35, 4.8), look: V(0.7, 0.3, 0.32) }, // 3 bank statement
      { cam: V(0.95, 0.35, 6.1), look: V(0.82, -0.02, 0.32), exp: E * 0.758 }, // 4 approval dialog
      { cam: V(1.45, 0.1, 4.7), look: V(0.55, 0, 0.32) }, // 5 read back (page top clear of the header)
      { cam: V(1.9, 0.35, 5.2), look: V(1.05, 0.2, 0.4) }, // 6 receipt
      { cam: this.camClose, look: this._lookClose }, // 7 close
    ];
    // cobalt, phone: chapters 1-6 dolly in along the view ray so the open page fills the canvas box's width
    // (it spanned ~65% of it; 41% of the screen was bare gradient). The hero is untouched.
    if (this.mobile && THEME.mobileFrame) {
      const MF = Array.isArray(THEME.mobileFrame) ? THEME.mobileFrame : MOBILE_FRAME; // a theme can carry its own values (the red book's block is slimmer)
      for (let i = 1; i <= 7; i++) {
        const k = kfs[i], [d, dy, dx] = MF[i];
        k.cam.sub(k.look).multiplyScalar(d); // look is this keyframe's own vector (V(...)), so it can be moved in place
        k.look.x += dx; k.look.y += dy;
        k.cam.add(k.look);
      }
    }
    return kfs;
  }

  // tuning hook: move one keyframe and rebuild the path (used by _lh scripts only)
  setKeyframe(i, cam, look) {
    this._camKF[i].cam.set(cam[0], cam[1], cam[2]);
    this._camKF[i].look.set(look[0], look[1], look[2]);
    this._buildCameraPath();
  }

  _buildCameraPath() {
    const E = THEME.exposure;
    const kf = this._camKF || (this._camKF = this._makeKeyframes());
    const D = 7, A = WARP_A;
    const pose = (k) => Float64Array.of(k.cam.x, k.cam.y, k.cam.z, k.look.x, k.look.y, k.look.z, k.exp ?? E);
    // nodes at u = i + 0.35, plus a short "hold" node after keyframe 2 (below)
    const nodes = kf.map((k, i) => ({ t: 0.35 + i, v: pose(k) }));
    const H2 = HOLD2;
    {
      const a = pose(kf[H2.from]), b = pose(kf[H2.from + 1]), v = new Float64Array(D);
      for (let d = 0; d < D; d++) v[d] = a[d] + (b[d] - a[d]) * H2.f;
      v[0] += H2.dx; v[3] += H2.dx;
      nodes.splice(H2.from + 1, 0, { t: H2.t, v });
    }
    const n = nodes.length;
    const P = nodes.map((nd) => nd.v), T = nodes.map((nd) => nd.t);
    this._camT = Float64Array.from(T);
    const knot = []; // centripetal knot spacing between neighbours (cam + look chord, square-rooted)
    for (let j = 0; j < n - 1; j++) {
      const c = Math.hypot(P[j + 1][0] - P[j][0], P[j + 1][1] - P[j][1], P[j + 1][2] - P[j][2]) + Math.hypot(P[j + 1][3] - P[j][3], P[j + 1][4] - P[j][4], P[j + 1][5] - P[j][5]);
      knot.push(Math.sqrt(Math.max(c, 1e-3)));
    }
    // tangent per node in u-space: the centripetal direction, one scale per node
    // (knot span per u span of its neighbours) so the u-velocity is C1
    const M = P.map((_, j) => {
      const m = new Float64Array(D);
      const j0 = j === 0 ? 0 : j - 1, j1 = j === n - 1 ? n - 2 : j;
      const d0 = knot[j0], d1 = knot[j1];
      const scale = (d0 + d1) / (T[j1 + 1] - T[j0]);
      for (let d = 0; d < D; d++) {
        const p0 = j === 0 ? 2 * P[0][d] - P[1][d] : P[j - 1][d];
        const p2 = j === n - 1 ? 2 * P[n - 1][d] - P[n - 2][d] : P[j + 1][d];
        m[d] = ((P[j][d] - p0) / d0 - (p2 - p0) / (d0 + d1) + (p2 - P[j][d]) / d1) * scale;
      }
      // keep the centripetal direction but give the camera and look a speed of the
      // neighbouring chords per unit u: at a keyframe where the path only reverses in
      // one axis (the dialog's pull-back) the plain tangent shrinks to nothing, and the file stops
      const c0 = Math.hypot(P[j1][0] - P[j0][0], P[j1][1] - P[j0][1], P[j1][2] - P[j0][2]) + Math.hypot(P[j1][3] - P[j0][3], P[j1][4] - P[j0][4], P[j1][5] - P[j0][5]);
      const c1 = Math.hypot(P[j1 + 1][0] - P[j1][0], P[j1 + 1][1] - P[j1][1], P[j1 + 1][2] - P[j1][2]) + Math.hypot(P[j1 + 1][3] - P[j1][3], P[j1 + 1][4] - P[j1][4], P[j1 + 1][5] - P[j1][5]);
      const want = j === 0 || j === n - 1 ? (j === 0 ? c1 / (T[1] - T[0]) : c0 / (T[n - 1] - T[n - 2])) : 0.5 * (c0 / (T[j] - T[j - 1]) + c1 / (T[j + 1] - T[j]));
      const have = Math.hypot(m[0], m[1], m[2]) + Math.hypot(m[3], m[4], m[5]);
      if (have > 1e-6) {
        const k = Math.min(want / have, 6);
        for (let d = 0; d < 6; d++) m[d] *= k;
      }
      return m;
    });
    // cubic Hermite coefficients per segment: v(s) = a + s(b + s(c + s d)), s = (u - t_j) / len
    this._camC = new Float64Array((n - 1) * 4 * D);
    for (let j = 0; j < n - 1; j++) {
      const len = T[j + 1] - T[j];
      for (let d = 0; d < D; d++) {
        const p0 = P[j][d], p1 = P[j + 1][d], m0 = M[j][d] * len, m1 = M[j + 1][d] * len, o = j * 4 * D + d;
        this._camC[o] = p0;
        this._camC[o + D] = m0;
        this._camC[o + 2 * D] = -3 * p0 + 3 * p1 - 2 * m0 - m1;
        this._camC[o + 3 * D] = 2 * p0 - 2 * p1 + m0 + m1;
      }
    }
    // the hero's own push (0 - 0.35): from the resting pose to keyframe 0, zero velocity at
    // the start and the path's velocity at the end; and the close's decaying drift after 7.35
    this._camPre = new Float64Array(4 * D);
    this._camPost = new Float64Array(2 * D + 3); // [P7 | velocity per unit p at 7.35 | unit view direction of the closing pose]
    const S = Float64Array.of(this.camHero.x, this.camHero.y, this.camHero.z, this._lookHero.x, this._lookHero.y, this._lookHero.z, E);
    for (let d = 0; d < D; d++) {
      const p0 = S[d], p1 = P[0][d], m0 = 0, m1 = M[0][d] * (1 - A) * 0.35; // du/dp = 1 - A at 0.35; s = p / 0.35
      this._camPre[d] = p0;
      this._camPre[D + d] = m0;
      this._camPre[2 * D + d] = -3 * p0 + 3 * p1 - 2 * m0 - m1;
      this._camPre[3 * D + d] = 2 * p0 - 2 * p1 + m0 + m1;
      this._camPost[d] = P[n - 1][d];
      this._camPost[D + d] = M[n - 1][d] * (1 - A);
    }
    {
      const c = this._camPost, dx = c[3] - c[0], dy = c[4] - c[1], dz = c[5] - c[2], n = Math.hypot(dx, dy, dz) || 1;
      c[2 * D] = dx / n; c[2 * D + 1] = dy / n; c[2 * D + 2] = dz / n;
    }
    this._camV = new Float64Array(D);
  }

  // camera pose at chapterPos p, no allocation
  _camAt(p) {
    const D = 7, v = this._camV;
    if (p < 0.35) {
      const s = p / 0.35, c = this._camPre;
      for (let d = 0; d < D; d++) v[d] = c[d] + s * (c[D + d] + s * (c[2 * D + d] + s * c[3 * D + d]));
    } else if (p > 7.35) {
      // the arrival's velocity decays (tau 0.3) while a slow push-in ramps up (time constant
      // 0.08) to PUSH_V world units per chapter along the view: the closed file is never at
      // rest before the pin releases (about 0.06 screen px per scrolled px)
      const s = p - 7.35, tau = 0.3, g = tau * (1 - Math.exp(-s / tau)), c = this._camPost;
      const push = PUSH_V * (s - 0.08 * (1 - Math.exp(-s / 0.08)));
      for (let d = 0; d < D; d++) v[d] = c[d] + c[D + d] * g;
      v[0] += c[2 * D] * push; v[1] += c[2 * D + 1] * push; v[2] += c[2 * D + 2] * push;
    } else {
      const u = p - (WARP_A / (2 * Math.PI)) * Math.sin(2 * Math.PI * (p - 0.35));
      const T = this._camT, last = T.length - 2;
      let j = 0;
      while (j < last && u >= T[j + 1]) j++;
      const s = (u - T[j]) / (T[j + 1] - T[j]), c = this._camC, o = j * 4 * D;
      for (let d = 0; d < D; d++) v[d] = c[o + d] + s * (c[o + D + d] + s * (c[o + 2 * D + d] + s * c[o + 3 * D + d]));
    }
    let sx = 0, sy = 0;
    if (p > 0.35 && p < 7.5) {
      const th = 2 * Math.PI * (p - 0.35), env = SWAY * smoothstep(0.35, 0.6, p) * (1 - smoothstep(7.2, 7.45, p));
      sy = env * Math.sin(th); // vertical only: fastest at the keyframes, where the path is slowest
    }
    this.camera.position.set(v[0] + sx, v[1] + sy, v[2]);
    this._look.set(v[3] + sx, v[4] + sy, v[5]);
    this.renderer.toneMappingExposure = v[6];
  }

  _buildLights() {
    // a soft studio room for reflections, and a key light from the camera's
    // side so the cover faces it instead of being grazed into black
    const pmrem = new PMREMGenerator(this.renderer);
    this.scene.environment = pmrem.fromScene(new RoomEnvironment(), 0.04).texture;
    this.scene.environmentIntensity = 0.35;
    pmrem.dispose();
    const key = new DirectionalLight(0xfff4e6, 2.2);
    key.position.set(1.6, 4.6, 5.4);
    this.scene.add(key);
    this.scene.add(new HemisphereLight(...THEME.hemi));
    if (THEME.rim) {
      const rim = new PointLight(0x8fb2ff, 0.45, 8);
      rim.position.set(2.6, 1.0, -1.8);
      this.scene.add(rim);
    }
    if (THEME.bahi) {
      // a warm rim light from behind and to the right, #c4553f. Dark-madder cloth on the dark red ground is only
      // 1.5-2.3:1, so the cover's far edges, the chamfer and the cloth's sheen must catch light to hold the silhouette
      const rim = new DirectionalLight(THEME.rimColor, 3);
      rim.position.set(3.4, 2.2, -3.0);
      this.scene.add(rim);
    }
    // on phones the file sits high in a short frame, so a floor shadow reads as detached
    if (!this.mobile) this._buildContactShadow();
  }

  // One baked soft contact shadow on the floor (desktop), in place of the shadow map.
  // A plane in the file's own yaw frame, its gradient drawn once: a tight dark core
  // hugging the file's foot plus a wide penumbra that leans away from the key light
  // (behind and to the left). Only the back board, spine and page block ever cast (the
  // front cover never did), so the footprint does not change as the file opens; the
  // plane follows the file's position, yaw and entrance scale, and fades in with it.
  _buildContactShadow() {
    const W = this.W_FOOT, PXU = 96; // canvas pixels per world unit
    const XA = -2.2, XB = 1.6, Z0 = -1.7, Z1 = 0.9; // world extent, file-local axes
    const c = document.createElement('canvas');
    c.width = Math.round((XB - XA) * PXU);
    c.height = Math.round((Z1 - Z0) * PXU);
    const cx = c.getContext('2d');
    const px = (x) => (x - XA) * PXU, pz = (z) => (z - Z0) * PXU;
    // a filled rect drawn far off-canvas with its shadow brought back: a cheap gaussian blur
    const blob = (x0, x1, z0, z1, blur, alpha) => {
      cx.save();
      cx.shadowColor = `rgba(0,0,0,${alpha})`;
      cx.shadowBlur = blur * PXU;
      cx.shadowOffsetX = 4000;
      cx.fillStyle = '#000';
      cx.fillRect(px(x0) - 4000, pz(z0), (x1 - x0) * PXU, (z1 - z0) * PXU);
      cx.restore();
    };
    // footprint (file-local): x -0.805..0.805, z -0.35..0.35
    blob(-1.05, 0.65, -0.95, 0.05, 0.42, 0.55); // penumbra, leaning back-left
    blob(-0.83, 0.83, -0.4, 0.34, 0.1, 0.55); // core under the file's foot
    const tex = new CanvasTexture(c);
    tex.colorSpace = SRGBColorSpace;
    const mat = new MeshBasicMaterial({ map: tex, transparent: true, opacity: 0, depthWrite: false, toneMapped: false, fog: false });
    const shadow = new Mesh(new PlaneGeometry(XB - XA, Z1 - Z0), mat);
    shadow.geometry.translate((XA + XB) / 2, -(Z0 + Z1) / 2, 0); // plane y is world -z once laid flat
    shadow.rotation.order = 'YXZ';
    shadow.rotation.x = -Math.PI / 2;
    shadow.position.y = -1.05; // the book's foot (back board bottom -1.055)
    shadow.renderOrder = -1;
    this.scene.add(shadow);
    this._contactShadow = shadow;
    this._contactAlpha = THEME.floor * 1.6;
  }

  _placeContactShadow(e) {
    const sh = this._contactShadow;
    if (!sh) return;
    const g = this.fileGroup;
    sh.position.x = g.position.x;
    sh.position.z = g.position.z;
    sh.rotation.y = g.rotation.y;
    sh.scale.set(g.scale.x, g.scale.x, 1);
    sh.material.opacity = this._contactAlpha * (e < 0 ? 0 : e > 1 ? 1 : e);
  }

  // the core file: cover, spine, paper block, loose sheets, cover label,
  // the focal page (opens to the voucher) and the 26 hero flags. Everything
  // a first-paint frame needs; later chapters lazily add to this group.
  _buildFile() {
    const group = new Group();
    group.rotation.y = this.baseRotY;
    if (this.mobile) {
      // bigger and higher in the short mobile frame: at the old 0.62 scale
      // the voucher can't be read and the file's top clips under the header
      group.position.set(0.35, 0.05, 0);
      group.scale.setScalar(0.86);
    } else {
      group.position.x = 0.85;
    }
    this.scene.add(group);
    this.fileGroup = group;

    // entrance: the object springs into place (position + scale) instead of
    // fading in as a translucent ghost; fireFlags() is timed off this in app.js
    this._entranceBaseY = group.position.y;
    this._entranceBaseScale = group.scale.x;
    this._entranceSpring = new Spring(170, 18, 1);
    group.position.y = this._entranceBaseY - 0.25;
    group.scale.setScalar(this._entranceBaseScale * 0.96);

    // bahi worlds: a slimmer block, 0.40 (was 0.62); a register is not a brick. The 1:1.55 proportion is not applied:
    // every face is drawn for the 1.55 x 2.05 board, and a taller board would stretch the lettering
    const W = 1.55, H = 2.05, T = THEME.bahi ? 0.4 : 0.62;
    this.W = W; this.H = H; this.T = T;

    const coverMat = new MeshPhysicalMaterial({ map: makeClothTexture(THEME.board), color: 0xffffff, roughness: 0.82, metalness: 0, sheen: 0.35, sheenRoughness: 0.8, sheenColor: THEME.bahi ? new Color(THEME.rimColor) : new Color(THEME.board).lerp(new Color(0xffffff), 0.12) });
    const backCover = new Mesh(makeBoard(W + 0.06, H + 0.06, 0.04, 0.008), coverMat);
    backCover.position.set(0, 0, -T / 2 - 0.02);
    backCover.castShadow = backCover.receiveShadow = true;
    group.add(backCover);
    this.backCover = backCover;

    const spineMat = THEME.spine
      ? new MeshPhysicalMaterial({ color: THEME.spine, roughness: 0.5, clearcoat: 0.35, clearcoatRoughness: 0.4 })
      : new MeshStandardMaterial({ map: makeClothTexture(THEME.boardDeep), roughness: 0.85, metalness: 0.02 });
    const spine = new Mesh(new BoxGeometry(0.05, H + 0.06, T + 0.06), spineMat);
    spine.position.set(-W / 2 - 0.005, 0, 0);
    spine.castShadow = true;
    group.add(spine);
    if (THEME.bands) {
      // two gold foil bands across the ledger's spine
      const gold = new MeshStandardMaterial({ color: 0xcaa24a, metalness: 0.9, roughness: 0.3 });
      [0.36, -0.36].forEach((fy) => {
        const band = new Mesh(new BoxGeometry(0.056, 0.035, T + 0.066), gold);
        band.position.set(-W / 2 - 0.005, fy * H, 0);
        group.add(band);
      });
    }

    const edgeTex = makeEdgeStripeTexture();
    const edgeMat = new MeshStandardMaterial({ map: edgeTex, roughness: 0.82 });
    const plainMat = new MeshStandardMaterial({ color: THEME.block, roughness: 0.88 });
    const block = new Mesh(new BoxGeometry(W - 0.01, H - 0.01, T - 0.05), [edgeMat, plainMat, edgeMat, plainMat, plainMat, plainMat]);
    block.position.set(0, 0, -0.02);
    block.castShadow = block.receiveShadow = true;
    group.add(block);

    const looseTex = THEME.bahi ? makeBahiLooseTexture() : makePlainPaperTexture();
    looseTex.anisotropy = this.renderer.capabilities.getMaxAnisotropy();
    this._looseTex = looseTex;
    const looseMat = new MeshStandardMaterial({ map: looseTex, emissiveMap: looseTex, emissive: 0xffffff, emissiveIntensity: 0.3, roughness: 0.88 });
    const rndLoose = mulberry32(303);
    const blockFrontZ = -0.02 + (T - 0.05) / 2;
    for (let i = 0; i < 5; i++) {
      const sheet = new Mesh(new BoxGeometry(W - 0.02, H - 0.02, 0.006), looseMat);
      const jz = blockFrontZ + 0.006 + i * 0.011 + (rndLoose() - 0.5) * 0.004;
      sheet.position.set(0.004 + rndLoose() * 0.01, (rndLoose() - 0.5) * 0.02, jz);
      sheet.rotation.z = (rndLoose() - 0.5) * 0.018;
      sheet.castShadow = sheet.receiveShadow = true;
      group.add(sheet);
    }

    // the focal page: one canvas, redrawn on mode change or a quantised tick
    const pageCanvas = document.createElement('canvas');
    pageCanvas.width = 1024;
    pageCanvas.height = 1320;
    this._pageCtx = pageCanvas.getContext('2d');
    drawVoucherFace(this._pageCtx, 1024, 1320);
    const pageTex = new CanvasTexture(pageCanvas);
    pageTex.colorSpace = SRGBColorSpace;
    pageTex.anisotropy = this.renderer.capabilities.getMaxAnisotropy();
    this._pageCanvas = pageCanvas;
    this._pageTex = pageTex;
    this._pageMode = 'voucher';
    this._faces = { voucher: pageTex };

    const focal = new Mesh(new PlaneGeometry(W * 0.94, H * 0.94), new MeshBasicMaterial({ map: pageTex, toneMapped: false, side: DoubleSide }));
    focal.position.set(0, 0, T / 2 + 0.01);
    this._pageZ = T / 2 + 0.01; // the till-roll slip's depth is taken from this: the book's block is slimmer in the red world (T 0.40 against 0.62)
    group.add(focal);
    this.focalPage = focal;

    // pencil ticks as geometry on the voucher (canvas coords -> page plane)
    const PW = W * 0.94, PH = H * 0.94;
    const toPage = ([x, y]) => [(x / 1024 - 0.5) * PW, (0.5 - y / 1320) * PH];
    // where the 40A(3) flag presents once it is picked up: the voucher's
    // right margin, level with the Amount line and clear of the figure
    // (the amount's right edge is at page x 0.627; the flag's flat edge at 0.64)
    this.flagPresentPos = new Vector3(PW / 2 + 0.05, toPage([0, VOUCHER_ROW_Y])[1], T / 2 + 0.05);
    // transparent so the ticks can fade out as the page turns away (a hard
    // cut on the turn sheet, which has no ticks drawn on it, was visible)
    const tickMat = new MeshBasicMaterial({ color: new Color(THEME.tick), toneMapped: false, transparent: true });
    this._tickMat = tickMat;
    const tickW = (9 / 1024) * PW;
    this.tickRibbons = VOUCHER_TICKS.map(([x, y]) => [[x, y - 16], [x + 20, y + 4], [x + 58, y - 42]]).map((pts) => {
      const m = new Mesh(makeRibbon(pts.map(toPage), tickW), tickMat);
      m.position.z = 0.003;
      m.visible = false;
      focal.add(m);
      return m;
    });

    // a real page turn: a sheet hinged at the spine carries the old face
    // over while the focal page underneath already shows the new one. It
    // bends as it turns (a rigid card flip reads as a UI swap, not paper).
    const turnPivot = new Group();
    turnPivot.position.set(-PW / 2, 0, T / 2 + 0.016);
    const turnGeo = new PlaneGeometry(PW, PH, 24, 1);
    turnGeo.translate(PW / 2, 0, 0);
    this._turnBasePos = Float32Array.from(turnGeo.attributes.position.array);
    this._turnGeo = turnGeo;
    this._turnFrontMat = new MeshBasicMaterial({ map: pageTex, toneMapped: false, side: FrontSide });
    const turnFront = new Mesh(turnGeo, this._turnFrontMat);
    // the turn is a vertex deformation of one sheet (setChapterProgress), so its
    // free edge stays within ~0.12 of the block in z instead of swinging into the lens
    const turnBack = new Mesh(turnGeo, new MeshStandardMaterial({ map: looseTex, emissiveMap: looseTex, emissive: 0xffffff, emissiveIntensity: 0.3, roughness: 0.88, side: BackSide }));
    turnFront.frustumCulled = turnBack.frustumCulled = false;
    turnPivot.add(turnFront, turnBack);
    turnPivot.visible = false;
    group.add(turnPivot);
    this.turnPivot = turnPivot;

    // a soft shadow near the spine that deepens while a page is mid-turn
    const shadowCanvas = document.createElement('canvas');
    shadowCanvas.width = 64;
    shadowCanvas.height = 4;
    const sctx = shadowCanvas.getContext('2d');
    const grad = sctx.createLinearGradient(0, 0, 64, 0);
    grad.addColorStop(0, 'rgba(0,0,0,0.6)');
    grad.addColorStop(1, 'rgba(0,0,0,0)');
    sctx.fillStyle = grad;
    sctx.fillRect(0, 0, 64, 4);
    const turnShadow = new Mesh(
      new PlaneGeometry(PW * 0.4, PH),
      new MeshBasicMaterial({ map: new CanvasTexture(shadowCanvas), transparent: true, opacity: 0, depthWrite: false })
    );
    turnShadow.position.set(-PW / 2 + PW * 0.2, 0, T / 2 + 0.013);
    group.add(turnShadow);
    this._turnShadow = turnShadow;

    const fanCount = this.mobile ? 3 : 5;
    this.fanPages = [];
    const fanGeo = new PlaneGeometry(W * 0.97, H * 0.97);
    for (let i = 0; i < fanCount; i++) {
      // kept transparent and faded by setChapterProgress: at their small
      // rotation these full-page sheets never swing clear of the camera,
      // so without the fade they sit as a near-opaque blank page over the
      // open book (verified against a capture; the cover-angle fix
      // stands, but this specific "remove the hack" sub-fix regressed here)
      const fm = new MeshStandardMaterial({ map: looseTex, emissiveMap: looseTex, emissive: 0xffffff, emissiveIntensity: 0.3, roughness: 0.88, transparent: true });
      const m = new Mesh(fanGeo, fm);
      const pivot = new Group();
      pivot.position.set(-W / 2, 0, T / 2 - 0.01 - i * 0.012);
      m.position.set(W / 2, 0, 0);
      pivot.add(m);
      group.add(pivot);
      m.castShadow = false;
      this.fanPages.push(pivot);
    }

    const coverPivot = new Group();
    coverPivot.position.set(-W / 2, 0, T / 2 + 0.02);
    group.add(coverPivot);
    // bahi items 1-4: the front board carries its own unique texture (cloth, stitching and tape in one; the back board
    // and spine keep the tiled cloth, in the darker red)
    const bahiCover = THEME.bahi ? makeBahiCoverTexture(this.mobile ? 0.75 : 1) : null;
    if (bahiCover) this.renderer.initTexture(bahiCover.bump); // uploaded now with the build, never on a scroll frame
    const frontMat = THEME.bahi
      ? new MeshPhysicalMaterial({ map: bahiCover.map, bumpMap: bahiCover.bump, bumpScale: 1, color: 0xffffff, roughness: 0.9, metalness: 0, specularIntensity: 0.35, sheen: 0.15, sheenRoughness: 0.75, sheenColor: new Color(THEME.rimColor) })
      : coverMat;
    const frontCover = new Mesh(makeBoard(W, H, 0.04, 0.008), frontMat);
    frontCover.position.set(W / 2, 0, 0.02);
    frontCover.castShadow = false;
    frontCover.receiveShadow = true;
    coverPivot.add(frontCover);
    this.coverPivot = coverPivot;
    // the pasted-down endpaper on the cover's inner face: once the cover lies
    // open the file reads as a book, not as a blue slab
    const epCanvas = document.createElement('canvas');
    epCanvas.width = THEME.bahi ? 384 : 256;
    epCanvas.height = THEME.bahi ? 480 : 320;
    const epCtx = epCanvas.getContext('2d');
    if (THEME.bahi) drawBahiEndpaper(epCtx, 384, 480); // a creased cream lining, not the cobalt lattice
    else {
    const epGrad = epCtx.createLinearGradient(0, 0, 256, 0); // plane u=1 is the hinge side
    epGrad.addColorStop(0, '#dfe4ef'); // a near-flat board paper, a shade lighter at the hinge: no dark stop for the canvas's left mask to meet as a bright column
    epGrad.addColorStop(1, '#e6eaf3');
    epCtx.fillStyle = epGrad;
    epCtx.fillRect(0, 0, 256, 320);
    if (!THEME.epTone) {
    // a faint marbled-paper lattice: fine diagonal rules and a dot in each diamond (cobalt's pastedown is plain board paper)
    epCtx.strokeStyle = 'rgba(31,63,191,0.07)';
    epCtx.lineWidth = 1;
    for (let k = -320; k < 256; k += 16) {
      epCtx.beginPath(); epCtx.moveTo(k, 0); epCtx.lineTo(k + 320, 320); epCtx.stroke();
      epCtx.beginPath(); epCtx.moveTo(k + 320, 0); epCtx.lineTo(k, 320); epCtx.stroke();
    }
    epCtx.fillStyle = 'rgba(31,63,191,0.09)';
    for (let y = 8; y < 320; y += 16) for (let x = (y / 16) % 2 ? 0 : 8; x < 256; x += 16) epCtx.fillRect(x, y, 1.6, 1.6);
    }
    }
    const epTex = new CanvasTexture(epCanvas);
    epTex.colorSpace = SRGBColorSpace;
    epTex.anisotropy = 4;
    this._epTex = epTex; // uploaded in idle time (ensureAsk), not on the frame the cover first opens
    // darkened to a board lining: the open cover runs off the frame's left edge in
    // chapters 1-6, and a pale panel there read as the second-largest light area on screen
    const endpaper = new Mesh(new PlaneGeometry(W - 0.05, H - 0.05), new MeshBasicMaterial({ map: epTex, color: THEME.bahi ? 0xffffff : (THEME.epTone || 0x9aa1b3), toneMapped: !THEME.bahi }));
    endpaper.rotation.y = Math.PI;
    endpaper.position.z = -0.0205;
    frontCover.add(endpaper);
    const labelCanvas = document.createElement('canvas');
    labelCanvas.width = 640;
    labelCanvas.height = 220;
    this._labelCtx = labelCanvas.getContext('2d');
    drawCoverLabel(this._labelCtx);
    const labelTex = new CanvasTexture(labelCanvas);
    labelTex.colorSpace = SRGBColorSpace;
    if (THEME.bahi) labelTex.anisotropy = 8; // the label no longer smears at grazing angles
    this._labelTex = labelTex;
    const labelH = W * 0.68 * (220 / 640);
    const label = new Mesh(new PlaneGeometry(W * 0.68, labelH), new MeshStandardMaterial({ map: labelTex, roughness: 0.7, transparent: true }));
    label.position.set(0, H * 0.26, 0.021);
    frontCover.add(label);
    // the closing tick: ~140 px across the label (640x220 canvas units), 10 px
    // pencil, drawn as geometry so the close uploads no texture
    const lw = W * 0.68, toLabel = ([x, y]) => [(x / 640 - 0.5) * lw, (0.5 - y / 220) * labelH];
    // 2.3x the old tick (its arms ran 62 and 151 canvas px), a 24 px pencil, its long arm
    // rising past the label's top edge onto the board, so the close's one gesture reads from across the room
    // the long arm rises off the paper label onto the cloth, where the paper-red #b3261e would vanish (under
    // 2:1 against the cloth); that part is drawn in the light tick #e8735f instead (vertex colour, flat per segment). The
    // V's foot dips ~19 canvas px below the paper onto the label's own shadow; it stays paper-red (a second colour change
    // on a stub that short read as a glitch, and it is joined to the stroke on the paper).
    // 30 sub-segments per arm keep the colour change within ~6 label px of the label's top edge; both arms keep
    // the same share of the drawing time as before (each is half the segments)
    // bahi worlds: a tapered blue-black ink stroke (7.5 label px, ~5.5 px on screen at 1440) wholly on the paper label,
    // in the free corner under the A.Y. line and right of the voucher line, so it never covers the lettering and never
    // crosses onto the cloth (which removes the old two-colour swap)
    // 11 label px (was 7.5) and a longer arm (about 123 label px, was 91), still wholly in the label's free lower-right
    // corner: the tip ends ~9 px below the "(A.Y. 2026-27)" line's descenders, the foot ~6 px above the inner rule
    // bahi worlds: a steeper pen tick, about 1.8x the earlier one: a short stroke down (about 69 deg) into a clear V, then a long
    // arm rising at about 49 deg (was about 19), 15 label px wide with a milder taper. It crosses the A.Y. line's right end as a
    // reviewer's ink over print does (cobalt's tick does too) and stays wholly on the label; the foot turns through three short steps
    // (about 40 deg each) so the wide stroke keeps a filled, rounded corner instead of a notch
    if (THEME.bahi) this._labelTickGeo = makeRibbon([[385, 92], [412, 160], [414, 166], [419, 168], [425, 167], [552, 20]].map(toLabel), (15 / 640) * lw, 12, 0.22);
    // cobalt: the same pen stroke, but wholly in the label's free lower-right corner (right of "Vouchers, Apr to Mar",
    // under the A.Y. line), so it never covers "A.Y. 2026-27"; red #b3261e, the tick stays on the paper
    else if (THEME.penTick) this._labelTickGeo = makeRibbon([[392, 134], [414, 178], [417, 183], [422, 185], [428, 183], [588, 132]].map(toLabel), (14 / 640) * lw, 12, 0.22);
    else this._labelTickGeo = makeRibbon([[330, 110], [420, 215], [610, -70]].map(toLabel), (24 / 640) * lw, 10, 0.12);
    const labelTick = new Mesh(this._labelTickGeo, new MeshBasicMaterial({ color: new Color(THEME.bahi ? THEME.closeTick : THEME.tick), toneMapped: false }));
    labelTick.position.z = 0.003;
    label.add(labelTick);

    // --- 26 hero flags, spring-driven, down the fore-edge, each carrying a
    // printed clause tag. At rest they sit wholly inside the block; `out`
    // brings the flat, lettered part of the strip clear of the cover edge.
    // The slots are 0.08 apart for a 0.066 flag, and the three depth lanes
    // are only 0.04 apart in z, and neighbouring slots differ by one lane at
    // most (a wider stagger reads as overlap from a camera above the file),
    // so no two lettered strips overlap in a normal view.
    const flagGeo = makeFlagGeometry();
    this._flagGeo = flagGeo;
    const flags = [];
    const total = FLAG_GROUPS.reduce((s, g) => s + g.count, 0);
    const rndS = mulberry32(977);
    const slots = Array.from({ length: total }, (_, i) => i);
    for (let i = slots.length - 1; i > 0; i--) {
      const j = Math.floor(rndS() * (i + 1));
      [slots[i], slots[j]] = [slots[j], slots[i]];
    }
    let idx = 0;
    FLAG_GROUPS.forEach((g) => {
      const color = (THEME.flags || {})[g.key] ?? g.color;
      const tex = makeFlagTexture(g.key, color, g.flagTag || g.label, { outline: !!g.outline });
      const outlineOpacity = THEME.outlineInk ? 0.85 : 1;
      const mat = new MeshStandardMaterial({ map: tex, transparent: !!g.outline, opacity: g.outline ? outlineOpacity : 1, roughness: 0.55, side: DoubleSide, toneMapped: false });
      for (let i = 0; i < g.count; i++) {
        const mesh = new Mesh(flagGeo, mat);
        const slot = slots[idx];
        const lane = [0, 1, 2, 1][slot % 4]; // neighbours are never more than one lane apart
        const yEdge = MathUtils.lerp(H / 2 - 0.05, -H / 2 + 0.05, slot / (total - 1));
        const z = -0.02 + (lane - 1) * 0.04;
        mesh.userData.rest = new Vector3(W / 2 - 0.31, yEdge, z);
        mesh.userData.axis = 'fore';
        mesh.userData.out = 0.39 + 0.012 * lane;
        mesh.position.copy(mesh.userData.rest);
        mesh.userData.spring = new Spring(170, 18, 1); // the one-time load entrance only
        mesh.userData.delay = idx * 0.045;
        mesh.userData.amt = 0; // scroll-driven out-amount, set by setChapterProgress
        mesh.userData.k = idx;
        group.add(mesh);
        flags.push(mesh);
        idx++;
      }
    });
    this.flags = flags;
    this.heroFlag = flags[0];
    this.heroFlag.material = this.heroFlag.material.clone();
    this.heroFlag.material.transparent = true; // it fades as the working-paper row takes over
    // it flies over everything (the floor's shadow plane would otherwise hide it low in the frame)
    this._heroOn = false;
    this._heroTravel = 0;
    this._heroTilt = 0;
    this._flagList = flags.slice();
  }

  // the one-time load entrance: the springs bring the hero flags out. After
  // it (and from the first scroll past the hero) every flag's out-amount is a
  // pure function of chapterPos, so scrubbing reproduces it exactly.
  fireFlags() {
    this._fired = true;
    this._fireT = this._elapsed;
    this.flags.forEach((f) => (f.userData.spring.target = 1));
  }

  // ---------------------------------------------------------- lazy chapters
  // Each ensure* builds geometry/textures once, the first time it is
  // needed, so a fresh load never pays for chapters the visitor hasn't
  // scrolled to yet.

  ensureAsk() {
    if (this._built.has('ask')) return;
    this._built.add('ask');
    this.renderer.initTexture(this._epTex);
    this._face('ledger');
    const { W, H } = this;
    const PW = W * 0.94, PH = H * 0.94;
    const toPage = ([x, y]) => [(x / 1024 - 0.5) * PW, (0.5 - y / 1320) * PH];
    // the answer to "who is over 90 days" is five figures in a full ageing:
    // pencil a ring round each, then round their total. Hand-drawn: a
    // wobbling polyline whose ends taper like a pencil touching down.
    const ribbonMat = new MeshBasicMaterial({ color: new Color(THEME.tick), toneMapped: false, transparent: true });
    this._askMat = ribbonMat;
    const ribbonW = (5 / 1024) * PW;
    this.askRibbons = ASK_RINGS.map(([cx, cy, rx, ry], k) => {
      const m = new Mesh(makeRibbon(penRing(cx, cy, rx, ry, 71 + k * 13).map(toPage), ribbonW, 1, 0.22), ribbonMat);
      m.position.z = 0.0032;
      m.visible = false;
      this.focalPage.add(m);
      return m;
    });
  }

  _face(mode) {
    if (this._faces[mode]) return this._faces[mode];
    const c = document.createElement('canvas');
    c.width = 1024;
    c.height = 1320;
    const ctx = c.getContext('2d');
    if (mode === 'ledger') drawLedgerFace(ctx, 1024, 1320);
    else drawBankFace(ctx, 1024, 1320);
    const tex = new CanvasTexture(c);
    tex.colorSpace = SRGBColorSpace;
    tex.anisotropy = this.renderer.capabilities.getMaxAnisotropy();
    this.renderer.initTexture(tex);
    this._faces[mode] = tex;
    return tex;
  }

  ensureBank() {
    if (this._built.has('bank')) return;
    this._built.add('bank');
    this._face('bank');
    const { W, H, T } = this;
    const PW = W * 0.94, PH = H * 0.94;
    const toPage = ([x, y]) => [(x / 1024 - 0.5) * PW, (0.5 - y / 1320) * PH];
    const shown = BANK_ROWS.slice(BANK_SHOWN_FROM, BANK_SHOWN_FROM + BANK_SHOWN);
    // cyan: an ask-the-books follow-up, not yet resolved. It turns into the
    // white/ink "resolved" tick once its name is decided (never the 41(1) green).
    const tex = makeFlagTexture('query', THEME.decideInk || CYAN, '?', { textColor: THEME.decideText, rim: THEME.decideInk ? THEME.decideRim : null });
    this._queryTexQ = tex;
    this._queryTexT = makeFlagTexture('resolved', THEME.flagOutlineFill || 0xffffff, '', { tick: true });
    this.renderer.initTexture(tex);
    this.renderer.initTexture(this._queryTexT);
    // beside the three near-miss rows on the statement page itself, in the
    // statement's row order (top to bottom: the same order as the names card and
    // nameResolve), not buried among the 26 hero flags on the fore-edge
    const queryRows = shown.map((r, ri) => ({ r, ri })).filter(({ r }) => r.query);
    this.queryFlags = queryRows.map(({ ri }, i) => {
      const cy = BANK_ROW_Y0 + ri * BANK_ROW_PITCH + 6; // the row's optical middle (narration + date), not its baseline
      const [, py] = toPage([0, cy]);
      const mat = new MeshStandardMaterial({ map: tex, roughness: 0.4, side: DoubleSide, toneMapped: false });
      const mesh = new Mesh(this._flagGeo, mat);
      mesh.userData.rest = new Vector3(PW * 0.5 + 0.07, py, T / 2 + 0.03 + i * 0.002);
      mesh.userData.axis = 'page';
      mesh.userData.amt = 0;
      mesh.userData.isTick = false;
      mesh.position.copy(mesh.userData.rest);
      mesh.rotation.y = -0.3;
      mesh.visible = false;
      this.fileGroup.add(mesh);
      this._flagList.push(mesh);
      return mesh;
    });

    // one wash plane per statement row, drawn once: multiplied over the page, so
    // the row's ink stays black. The three "?" rows wear the cyan bar until their
    // name is decided (it fades to paper with the chip); a row whose slip has
    // lifted goes pale grey-blue. Colours are the sRGB factors of the old baked bar.
    const washGeo = new PlaneGeometry(((968 - 32 + 28) / 1024) * PW, (88 / 1320) * PH);
    const washMat = (hex) => new MeshBasicMaterial({ color: new Color(hex), transparent: true, opacity: 0, blending: MultiplyBlending, premultipliedAlpha: true, toneMapped: false, depthWrite: false });
    let qi = 0, si = 0;
    this.washes = shown.map((row, ri) => {
      const m = new Mesh(washGeo, washMat(row.query ? (THEME.washQuery || '#d4f7fd') : (THEME.washDone || '#e2e8f6')));
      const [wx, wy] = toPage([500, BANK_ROW_Y0 + ri * BANK_ROW_PITCH + 4]);
      m.position.set(wx, wy, 0.0012);
      m.userData = row.query ? { q: qi++ } : { s: si++ };
      m.visible = false;
      this.focalPage.add(m);
      return m;
    });

    // chapter 3, the statement becomes vouchers: one thin slip lifts off each
    // of the page's non-query rows (the three "?" rows never lift), top to
    // bottom. Each carries its voucher type (a credit is a Receipt, a debit a
    // Payment) and the row's amount verbatim. Textures are drawn once here and
    // uploaded now (initTexture), so nothing is drawn or uploaded while
    // scrolling. Path, per slip: the row on the page -> an arc towards the
    // camera -> a pile fanned in the page's own right margin. renderOrder fixes
    // the pile's order (later slip on top) whatever the camera does.
    const slipW = 0.46, slipH = 0.1, sw = 512, sh = 112;
    const slipGeo = new PlaneGeometry(slipW * (sw / (sw - 16)), slipH * (sh / (sh - 16)));
    this.slips = [];
    shown.forEach((row, ri) => {
      if (row.query) return;
      const k = this.slips.length;
      const c = document.createElement('canvas');
      c.width = sw; c.height = sh;
      const cx = c.getContext('2d');
      cx.shadowColor = 'rgba(4,10,40,0.35)';
      cx.shadowBlur = 7;
      cx.shadowOffsetY = 2;
      cx.fillStyle = '#fffefa';
      cx.fillRect(8, 8, sw - 16, sh - 16);
      cx.shadowColor = 'transparent';
      cx.fillStyle = `rgba(${THEME.inkRgb},0.85)`;
      cx.fillRect(8, 8, 8, sh - 16);
      cx.strokeStyle = `rgba(${THEME.inkRgb},0.22)`;
      cx.lineWidth = 2;
      cx.strokeRect(9, 9, sw - 18, sh - 18);
      cx.fillStyle = THEME.ink;
      cx.textBaseline = 'middle';
      cx.font = '700 40px "Bricolage Grotesque Variable", sans-serif';
      cx.fillText(row.cr ? 'Receipt' : 'Payment', 36, sh / 2 + 1);
      cx.font = '600 40px "Geist Mono Variable", monospace';
      cx.textAlign = 'right';
      cx.fillText(row.cr || row.dr, sw - 30, sh / 2 + 1);
      const tx = new CanvasTexture(c);
      tx.colorSpace = SRGBColorSpace;
      tx.anisotropy = 4;
      this.renderer.initTexture(tx);
      const m = new Mesh(slipGeo, new MeshBasicMaterial({ map: tx, transparent: true, toneMapped: false, side: DoubleSide, depthWrite: false }));
      m.renderOrder = 10 + k;
      const cy = BANK_ROW_Y0 + ri * BANK_ROW_PITCH + 4;
      const A = new Vector3(0.16, toPage([0, cy])[1], T / 2 + 0.02);
      const C = new Vector3(PW / 2 + 0.2 + (k % 2 ? 0.01 : -0.01), -0.02 - k * 0.03, T / 2 + 0.06 + k * 0.004);
      const B = A.clone().lerp(C, 0.5).add(new Vector3(0, 0.16, 0.3));
      m.userData = { A, B, C, rz: (k - 2) * 0.05, tuck: new Vector3(W / 2 - 0.06, C.y, 0.02), ri };
      m.visible = false;
      this.fileGroup.add(m);
      this.slips.push(m);
    });
    // the top card of the pile carries a small "139 ready" tag under its edge
    {
      const c = document.createElement('canvas');
      c.width = 224; c.height = 64;
      const cx = c.getContext('2d');
      cx.fillStyle = THEME.ink;
      cx.beginPath();
      if (cx.roundRect) cx.roundRect(2, 2, 220, 60, 14); else cx.rect(2, 2, 220, 60);
      cx.fill();
      cx.fillStyle = '#ffffff';
      cx.font = '650 29px "Geist Mono Variable", monospace';
      cx.textAlign = 'center';
      cx.textBaseline = 'middle';
      cx.fillText('139 matched', 112, 34);
      const tx = new CanvasTexture(c);
      tx.colorSpace = SRGBColorSpace;
      tx.anisotropy = 4;
      this.renderer.initTexture(tx);
      const tag = new Mesh(new PlaneGeometry(0.24, 0.24 * (64 / 224)), new MeshBasicMaterial({ map: tx, transparent: true, toneMapped: false, depthWrite: false }));
      tag.position.set(slipW * 0.5 - 0.15, -slipH * 0.5 - 0.035, 0.001);
      tag.renderOrder = 30;
      this.slips[this.slips.length - 1].add(tag);
      this._slipTag = tag;
    }

    // each lifted slip leaves a pencil tick at its row's right edge (the read-back ribbon style)
    const tickMat = new MeshBasicMaterial({ color: new Color(THEME.tick), toneMapped: false, transparent: true });
    this._slipTickMat = tickMat;
    const tw = (9 / 1024) * PW;
    this.slipTicks = this.slips.map((sl) => {
      const cy = BANK_ROW_Y0 + sl.userData.ri * BANK_ROW_PITCH;
      const m = new Mesh(makeRibbon([[986, cy - 4], [998, cy + 10], [1018, cy - 16]].map(toPage), tw), tickMat);
      m.position.z = 0.0035;
      m.visible = false;
      this.focalPage.add(m);
      return m;
    });

    // chapter 4: the row the approval dialog posts (12 Mar, Nevrika) is bracketed
    // in pencil at 4.02; after the press a red tick is drawn on it
    const rowY = BANK_ROW_Y0 + 2 * BANK_ROW_PITCH;
    const brMat = new MeshBasicMaterial({ color: new Color(THEME.tick), toneMapped: false, transparent: true });
    this._brMat = brMat;
    const bw = (6 / 1024) * PW;
    this.brackets = [
      [[30, rowY - 46], [12, rowY - 46], [12, rowY + 54], [30, rowY + 54]],
      [[994, rowY - 46], [1012, rowY - 46], [1012, rowY + 54], [994, rowY + 54]],
    ].map((pts) => {
      const m = new Mesh(makeRibbon(pts.map(toPage), bw, 4, 0.1), brMat);
      m.position.z = 0.0034;
      m.visible = false;
      this.focalPage.add(m);
      return m;
    });
    this.postTick = new Mesh(makeRibbon([[968, rowY - 2], [979, rowY + 12], [998, rowY - 14]].map(toPage), tw), brMat);
    this.postTick.position.z = 0.0036;
    this.postTick.visible = false;
    this.focalPage.add(this.postTick);
    // compile the programs the washes, ribbons and tag need now, in idle time, not on the frame they first show
    const vis = [...this.washes, ...this.slips, ...this.slipTicks, ...this.brackets, this.postTick, this._slipTag];
    const was = vis.map((o) => o.visible);
    vis.forEach((o) => (o.visible = true));
    this.renderer.compile(this.scene, this.camera);
    vis.forEach((o, k) => (o.visible = was[k]));
  }

  ensureReadback() {
    if (this._built.has('readback')) return;
    this._built.add('readback');
    const { W, H } = this;
    const PW = W * 0.94, PH = H * 0.94;
    const toPage = ([x, y]) => [(x / 1024 - 0.5) * PW, (0.5 - y / 1320) * PH];
    // one pencil tick per statement row, at its right edge, instead of ten
    // floating green chips that didn't map to anything the reader could see
    const ribbonMat = new MeshBasicMaterial({ color: new Color(THEME.tick), toneMapped: false });
    const ribbonW = (9 / 1024) * PW;
    this.readbackTicks = BANK_ROWS.slice(BANK_SHOWN_FROM, BANK_SHOWN_FROM + BANK_SHOWN).map((row, ri) => {
      const cy = BANK_ROW_Y0 + ri * BANK_ROW_PITCH;
      const pts = [[986, cy - 4], [998, cy + 10], [1018, cy - 16]].map(toPage);
      const m = new Mesh(makeRibbon(pts, ribbonW), ribbonMat);
      m.position.z = 0.0035;
      m.visible = false;
      this.focalPage.add(m);
      return m;
    });
  }

  ensureReceipt() {
    if (this._built.has('receipt')) return;
    this._built.add('receipt');
    const { W } = this;
    const rw = 512, rh = 640;
    const c = document.createElement('canvas');
    c.width = rw; c.height = rh;
    const ctx = c.getContext('2d');
    drawReceiptFace(ctx, rw, rh);
    const tex = new CanvasTexture(c);
    tex.colorSpace = SRGBColorSpace;
    tex.anisotropy = this.renderer.capabilities.getMaxAnisotropy();
    const gw = W * 0.42, gh = gw * (rh / rw);
    const geo = new PlaneGeometry(gw, gh, 1, 12);
    geo.translate(0, -0.5 * gh, 0); // pivot at the top: unrolls downward
    // a gentle bend along its length, so it reads as a curled till roll
    // rather than a flat card
    const pos = geo.attributes.position;
    for (let vi = 0; vi < pos.count; vi++) {
      const v = (pos.getY(vi) + gh) / gh; // 0 at the free end .. 1 at the pivot
      pos.setZ(vi, pos.getZ(vi) + 0.04 * Math.sin(v * Math.PI));
    }
    geo.computeVertexNormals();
    const mesh = new Mesh(geo, new MeshStandardMaterial({ map: tex, roughness: 0.8, side: DoubleSide, transparent: true, alphaTest: 0.5, emissive: 0xffffff, emissiveMap: tex, emissiveIntensity: 0.35 }));
    // it starts between the page and the block, wholly behind the focal page
    // (which hides it while it slides), comes out past the page's right edge and
    // rests clear of the statement and of the text column (_rcXEnd, fitted by measure())
    this._rcXStart = 0;
    if (this._rcXEnd === undefined) this._rcXEnd = this.W * 0.47 + 0.26 + (gw * RECEIPT_SCALE) / 2;
    mesh.position.set(this._rcXStart, this.H * 0.36, this._pageZ - 0.05);
    mesh.scale.y = 0.6;
    mesh.visible = false; // built in idle time; hidden until its chapter
    mesh.castShadow = false; // an alpha-tested caster needs its own depth program, compiled on first sight; nothing here needs its shadow
    this.renderer.initTexture(tex);
    this.fileGroup.add(mesh);
    this.receiptRoll = mesh;
    // compile its programs now, in idle time, not on the frame it first shows
    mesh.visible = true;
    this.renderer.compile(this.scene, this.camera);
    mesh.visible = false;
  }

  // called ahead of the chapter that needs it, from an idle callback
  ensureChapter(i) {
    if (i === 1) this.ensureAsk();
    else if (i === 3) this.ensureBank();
    else if (i === 5) this.ensureReadback();
    else if (i === 6) this.ensureReceipt();
  }

  // -------------------------------------------------------------- redraws
  // quantised: only touches the canvas when the value actually moved, so
  // scrubbing doesn't redraw a 1024px canvas on every wheel tick.
  _quant(key, v, step = 0.02) {
    const q = Math.round(v / step) * step;
    if (this._lastQuant[key] === q) return null;
    this._lastQuant[key] = q;
    return q;
  }

  // ---------------------------------------------------------- master state
  // chapterPos is continuous, 0..CHAPTER_COUNT. This is the one function
  // the whole page drives: it derives every object's transform from a
  // single scroll-fraction input, so nothing here is path-dependent. Nothing
  // below allocates: it runs at 120+ Hz. Timeline,
  // one thing at a time:
  //   1: flags retract 1.02-1.16, cover opens 1.12-1.60 (sine), rings 1.50-1.70, total 1.72-1.85
  //   2: page turn 2.04-2.34 (sine), yellow flags out 2.28-2.38, ticks 2.38-2.50, the 40A(3)
  //      flag presents 2.38-2.46 and travels 2.46-2.70; then one thing at a time: the working
  //      paper card out 2.90-2.98, the flags in 2.94-3.02, the ticks fade 2.98-3.06
  //   3: page turn 3.10-3.46 (after the camera's fastest point), slips 3.40-3.72, query flags
  //      3.58-3.71, names 3.72-3.88 (the counter rises 139 -> 142 with them), names card and query flags out 3.90-3.98
  //   4: bracket 4.02-4.08, dialog rises 4.04-4.30 (app.js), pressed 4.44-4.50, hands back 4.50-4.70, red tick 4.62-4.70
  //   5: read-back ticks 5.25-5.85    6: receipt out 6.12-6.50
  //   7: receipt tucks 6.72-7.02, cover shuts 7.04-7.46 (sine), tick 7.52-7.88, a slow push-in to the release
  setChapterProgress(chapterPos) {
    const p = Math.max(0, Math.min(CHAPTER_COUNT - 0.001, chapterPos));
    const i = Math.floor(p);
    const local = p - i;
    if (!this._probing) this.ensureChapter(i);
    this._p = p;

    // openAmount: closed through the hero, opens 1.16-1.46 (once the flags are
    // in), holds open, and shuts 7.08-7.42 (power2InOut) once the flags are in
    // and the receipt is tucked, so the file closes clean.
    // yaw: the file's yaw follows the plain ease (not the arc-length warp), so the page keeps moving
    // at a steady rate while the cover passes edge-on (the warp slows the angle exactly there)
    let open, yawOpen;
    if (p < COVER.open0) open = yawOpen = 0;
    else if (p < COVER.open1) {
      yawOpen = trapEase((p - COVER.open0) / (COVER.open1 - COVER.open0), COVER.r);
      open = arcInv(COVER_ARC_OPEN, yawOpen);
    } else if (p < COVER.shut0) open = yawOpen = 1;
    else if (p < COVER.shut1) {
      yawOpen = 1 - trapEase((p - COVER.shut0) / (COVER.shut1 - COVER.shut0), COVER.r);
      open = 1 - arcInv(COVER_ARC_SHUT, 1 - yawOpen);
    } else open = yawOpen = 0;
    this._openAmount = open;
    const openOK = open >= 0.6; // the ribbons on the page are drawn only while the file is open

    // opened well past edge-on (-2.95 rad) so the cover lies back and left,
    // instead of standing as a dark slab facing the camera
    this.coverPivot.rotation.y = MathUtils.lerp(0, -2.95, open);
    // the fan of loose sheets under the cover: invisible while the file is open
    // (their small rotation would cover the page), opaque once it is nearly shut
    const fanA = smoothstep(0.3, 0.22, open);
    for (let fi = 0; fi < this.fanPages.length; fi++) {
      const pivot = this.fanPages[fi];
      const t = clamp01((open - fi * 0.05) / 0.45);
      pivot.rotation.y = MathUtils.lerp(0, -0.22 - fi * 0.02, t);
      const mesh = pivot.children[0];
      if (mesh) {
        mesh.material.opacity = fanA;
        mesh.visible = fanA > 0.004; // a faded sheet still writes depth
      }
    }

    // camera: one continuous path (see _buildCameraPath). Chapter 0 answers the
    // first scroll: the file turns -0.46 -> -0.32 (mobile -0.20) over 0-0.95 as the camera pushes in.
    this._camAt(p);
    const rotEnd = this.mobile ? -0.2 : -0.32;
    const heroRot = p < 1 ? MathUtils.lerp(this.baseRotY, rotEnd, sineInOut(p / 0.95)) : rotEnd;
    this._openRotY = MathUtils.lerp(heroRot, this.closeRotY, yawOpen);

    // page content per chapter: pre-rendered faces are swapped, never
    // redrawn; entering chapters 2 and 3 the old page turns over the spine
    // (2.06-2.28, 3.04-3.22). The turn is a deformation of one sheet: it
    // shrinks across the spine and lifts at most ~0.09 in z (plus a 0.07 bow),
    // so it happens on the book, not in front of the lens.
    const TURN0 = i === 3 ? 0.1 : 0.04, TURN = i === 3 ? TURN_LEN[1] : TURN_LEN[0];
    const turning = (i === 2 || i === 3) && local < TURN0 + TURN && open > 0.9;
    this.focalPage.material.map = this._face(faceAt(p));
    if (turning) {
      const turnT = sineInOut((local - TURN0) / TURN);
      this._turnFrontMat.map = this._face(faceAt(i - 0.001));
      this._turnFrontMat.color.setScalar(1 - 0.1 * Math.sin(turnT * Math.PI)); // the sheet shades as it bows
      this.turnPivot.visible = true;
      const th = Math.PI * 0.985 * turnT, ct = Math.cos(th), st = Math.sin(th);
      const bendAmt = 0.07 * Math.sin(turnT * Math.PI);
      const pos = this._turnGeo.attributes.position;
      const base = this._turnBasePos;
      const pw = this.W * 0.94;
      for (let vi = 0; vi < pos.count; vi++) {
        const bx = base[vi * 3], by = base[vi * 3 + 1];
        const u = bx / pw;
        pos.setXYZ(vi, bx * ct, by, 0.09 * u * st + Math.sin(u * Math.PI) * bendAmt);
      }
      pos.needsUpdate = true;
      this._turnGeo.computeVertexNormals();
      this._turnShadow.material.opacity = 0.2 * Math.sin(turnT * Math.PI);
    } else {
      this.turnPivot.visible = false;
      this._turnShadow.material.opacity = 0;
    }

    // chapter 2: the voucher's two pencil ticks draw 2.38-2.50, and fade out
    // 2.94-3.03 (the turn sheet that follows carries no ticks)
    const inScrutiny = p >= 2 && p < 3.1 && openOK;
    const tickT = clamp01((p - 2.38) / 0.12);
    this._tickMat.opacity = 1 - smoothstep(2.98, 3.06, p);
    for (let k = 0; k < this.tickRibbons.length; k++) {
      const m = this.tickRibbons[k];
      m.visible = inScrutiny;
      setRibbonProgress(m.geometry, clamp01(tickT * 2 - k));
    }

    // chapter 1: ask the books. The answer is the five underlined rows on the
    // page itself, ringed one by one over 1.50-1.70, then their total 1.72-1.85;
    // they stay drawn until the page turns (faded 1.94-2.04).
    if (this.askRibbons) {
      const n = this.askRibbons.length;
      this._askMat.opacity = 1 - smoothstep(1.94, 2.04, p);
      for (let ri = 0; ri < n; ri++) {
        const m = this.askRibbons[ri];
        const local2 = ri < n - 1 ? clamp01((p - (1.5 + ri * 0.035)) / 0.06) : clamp01((p - 1.72) / 0.13);
        setRibbonProgress(m.geometry, local2);
        m.visible = p >= 1 && p < 2.1 && openOK && local2 > 0.001;
      }
    }

    // ---- flags: every out-amount is a pure function of chapterPos.
    // Hero: all 26 out. They retract 1.02-1.16 (each takes 0.10, stagger 0.0016)
    // while the camera keeps moving, before the cover opens. Chapter 2: only the
    // 7 yellow 40A(3) flags come out (2.28-2.38) and go back in (2.84-2.96, stagger 0.012).
    const fl = this.flags;
    for (let k = 0; k < fl.length; k++) {
      let a = 1 - smoothstep(1.02 + 0.0016 * k, 1.12 + 0.0016 * k, p);
      if (k < 7) a += smoothstep(2.28 + 0.0017 * k, 2.37 + 0.0017 * k, p) * (1 - smoothstep(2.94 + 0.006 * k, 2.99 + 0.006 * k, p));
      fl[k].userData.amt = a;
    }
    if (p > 1.02) this._entranceDone = true;

    // scrutiny payoff: the 40A(3) hero flag comes out with the others (2.28-2.38),
    // presents in the voucher's right margin (2.38-2.46), travels to the working
    // paper's clause cell (2.46-2.70,  placed each frame from the camera:
    // _placeHeroTravel) and crossfades into that cell's chip (2.69-2.73). On a
    // phone there is no travel: the flag fades and the row slides up (app.js).
    const h = this.heroFlag;
    this._heroOn = p >= 2.38 && p < 2.76;
    this._heroTravel = this.mobile ? 0 : clamp01((p - 2.46) / 0.24);
    h.visible = p < 2.76;
    h.material.depthTest = !this._heroOn; // in flight it is drawn over everything (the floor's shadow plane would hide it low in the frame)
    h.renderOrder = this._heroOn ? 40 : 0;
    h.material.opacity = this.mobile ? 1 - smoothstep(2.56, 2.64, p) : 1 - smoothstep(2.69, 2.73, p);
    if (this._heroOn && this._heroTravel === 0) {
      const e = smoothstep(2.38, 2.46, p), u = h.userData;
      this._v0.copy(u.rest);
      this._v0.x += u.out;
      h.position.lerpVectors(this._v0, this.flagPresentPos, e);
      h.rotation.set(0, MathUtils.lerp(-0.12, -0.08, e), 0.03 * e);
      h.scale.setScalar(1);
    }

    if (this.slips) this._updateBank(p, open, openOK);

    // chapter 5: read back, ticks wipe on in a row, in step with the counter (5.25-5.85)
    if (this.readbackTicks) {
      const t = clamp01((p - 5.25) / 0.6);
      const n = this.readbackTicks.length;
      for (let ti = 0; ti < n; ti++) {
        const local2 = clamp01(t * n - ti);
        const mesh = this.readbackTicks[ti];
        setRibbonProgress(mesh.geometry, local2);
        mesh.visible = local2 > 0.005 && openOK;
      }
    }

    // chapter 6: the receipt slides out from between the page and the block
    // (6.12-6.50, behind the page plane until it clears the page's edge), and
    // tucks back in (6.80-7.08).
    if (this.receiptRoll) {
      const u = smoothstep(6.12, 6.5, p) * (1 - smoothstep(6.72, 7.02, p));
      const r = this.receiptRoll;
      r.visible = u > 0.002;
      r.position.x = this._rcXStart + (this._rcXEnd - this._rcXStart) * u;
      r.position.z = this._pageZ - 0.05 + 0.14 * smoothstep(0.95, 1, u); // behind the page plane (and 0.04 of curl) while it slides, then a little forward; the depth follows the page's, not a fixed 0.32 (the red book is slimmer, and the slip was drawn over its statement at 7.0)
      r.scale.set(RECEIPT_SCALE, (0.6 + 0.4 * u) * RECEIPT_SCALE, 1);
      r.rotation.y = 0.18 * smoothstep(0.9, 1, u);
    }

    // chapter 7: after the cover shuts (7.08-7.42) the label's tick draws 7.50-7.80
    setRibbonProgress(this._labelTickGeo, (p - 7.52) / 0.36);
  }

  // chapter 3 and 4 objects on the statement: washes, slips, ticks, query flags,
  // the pencil bracket and the post tick. Split out only to keep the master
  // function readable; still a pure function of p, still no allocation.
  _updateBank(p, open, openOK) {
    const gone = smoothstep(3.92, 4.02, p); // slips tuck back into the file 3.92-4.02
    const onPage = p >= 3 && openOK;
    // cyan washes fade to paper as each name is decided; done rows go pale
    for (let ri = 0; ri < this.washes.length; ri++) {
      const w = this.washes[ri], d = w.userData;
      let a;
      if (d.q !== undefined) a = 1 - nameResolve(p, d.q);
      else {
        const sl = this.slips[d.s];
        a = smoothstep(0, 0.3, clamp01((p - (SLIP_WINDOW[0] + SLIP_STAG * d.s)) / SLIP_DUR)) * (1 - gone);
        sl.userData.wash = a;
      }
      w.material.opacity = a;
      w.visible = onPage && a > 0.004;
    }
    // slips: 0.19 each, staggered 0.03 (five inside 3.40-3.72)
    this._slipTickMat.opacity = 1 - gone;
    for (let k = 0; k < this.slips.length; k++) {
      const m = this.slips[k], d = m.userData;
      const u = clamp01((p - (SLIP_WINDOW[0] + SLIP_STAG * k)) / SLIP_DUR);
      const e = 0.3 * u + 0.7 * u * u * (3 - 2 * u); // mostly smoothstep, some linear: peak 1.35 x mean
      const a = smoothstep(0, 0.2, u) * (1 - smoothstep(0.4, 1, gone));
      m.visible = a > 0.004 && p >= SLIP_WINDOW[0] - 0.02;
      const tk = this.slipTicks[k];
      setRibbonProgress(tk.geometry, u / 0.4);
      tk.visible = onPage && u > 0.001 && gone < 0.99;
      if (!m.visible) continue;
      const w0 = (1 - e) * (1 - e), w1 = 2 * (1 - e) * e, w2 = e * e;
      m.position.set(d.A.x * w0 + d.B.x * w1 + d.C.x * w2, d.A.y * w0 + d.B.y * w1 + d.C.y * w2, d.A.z * w0 + d.B.z * w1 + d.C.z * w2);
      if (gone > 0) m.position.lerp(d.tuck, gone);
      m.scale.set(1 - 0.5 * gone, (0.35 + 0.65 * smoothstep(0, 0.3, u)) * (1 - 0.5 * gone), 1);
      m.rotation.set(0, 0.3 * Math.sin(Math.PI * e), d.rz * e);
      m.material.opacity = a;
      // the tag fades in only as the on-page counter reaches 139 (app.js: round(139 x slipWindow) first reads 139 at 3.709), not with the slip's own travel
      if (k === this.slips.length - 1) this._slipTag.material.opacity = smoothstep(3.708, 3.724, p) * a;
    }

    // query flags: scale in 3.58-3.71 (row order, stagger 0.025); each is decided
    // with its own chip (nameResolve): it shrinks to 0.1, swaps to the tick,
    // and grows back. They retract with the names card (3.90-3.98): by then their
    // meaning has moved to the chips, and no flag can cross the receipt or double a tick.
    const outAll = 1 - smoothstep(3.9, 3.98, p);
    for (let k = 0; k < this.queryFlags.length; k++) {
      const f = this.queryFlags[k], r = nameResolve(p, k);
      const mul = r < 0.5 ? 1 - 0.9 * smoothstep(0, 0.5, r) : 0.1 + 0.9 * smoothstep(0.5, 1, r);
      const tick = r >= 0.5;
      if (tick !== f.userData.isTick) {
        f.userData.isTick = tick;
        f.material.map = tick ? this._queryTexT : this._queryTexQ;
      }
      const a = smoothstep(3.58 + 0.025 * k, 3.66 + 0.025 * k, p) * outAll * mul;
      f.userData.amt = a;
      f.visible = a > 0.004;
    }

    // chapter 4: the row being posted is bracketed in pencil (4.02-4.08); after
    // the press the dialog hands back to it (4.50-4.70, app.js) and a tick is
    // drawn on it (4.62-4.70); both fade out before the read-back (4.90-5.00)
    this._brMat.opacity = 1 - smoothstep(4.9, 5, p);
    const bp = clamp01((p - 4.02) / 0.06);
    for (let k = 0; k < 2; k++) {
      setRibbonProgress(this.brackets[k].geometry, bp);
      this.brackets[k].visible = onPage && bp > 0.001 && p < 5.02;
    }
    setRibbonProgress(this.postTick.geometry, (p - 4.62) / 0.08);
    this.postTick.visible = onPage && p > 4.62 && p < 5.02;
  }

  // ---- layout probes: run on load and on resize, never per frame. Each one
  // evaluates the scene at a chapterPos, projects points to canvas CSS px, and
  // puts the scene back exactly as it was (every state is a pure function of p).
  // limitRight: the x (canvas px) the receipt's right edge may not pass at rest.
  // Returns { dlg: {x, y} (the 12 Mar row, where the dialog rises from),
  //           book: {l, t, r, b} (the file's outline while the bubbles are up) }.
  measure(limitRight) {
    this._resizeToDisplay();
    const g = this.fileGroup, cam = this.camera;
    const keep = { p: this._p ?? 0, py: g.position.y, sc: g.scale.x, rx: g.rotation.x, ry: g.rotation.y };
    const { W, H } = this, PW = W * 0.94, PH = H * 0.94;
    const v = new Vector3();
    const pose = (p) => {
      this._probing = true;
      this.setChapterProgress(p);
      this._probing = false;
      g.position.y = this._entranceBaseY;
      g.scale.setScalar(this._entranceBaseScale);
      g.rotation.set(0, this._openRotY, 0);
      cam.lookAt(this._look);
      cam.updateMatrixWorld();
      g.updateMatrixWorld(true);
    };
    const proj = (obj, x, y, z) => {
      v.set(x, y, z);
      obj.localToWorld(v);
      v.project(cam);
      return [((v.x + 1) / 2) * this._cw, ((1 - v.y) / 2) * this._ch];
    };
    pose(4.1);
    const rowY = BANK_ROW_Y0 + 2 * BANK_ROW_PITCH + 4;
    const [dx, dy] = proj(this.focalPage, (300 / 1024 - 0.5) * PW, (0.5 - rowY / 1320) * PH, 0);
    let l = 1e9, t = 1e9, r = -1e9, b = -1e9;
    const per = {};
    for (const p of [1.06, 1.5, 1.8]) {
      pose(p);
      let pl = 1e9, pt = 1e9, pr = -1e9, pb = -1e9;
      const sets = [[this.focalPage, PW / 2, PH / 2], [this.backCover, (W + 0.06) / 2, (H + 0.06) / 2]];
      if (this._openAmount < 0.5) sets.push([this.coverPivot.children[0], W / 2, H / 2]);
      if (p < 1.2) { per[p] = null; }
      for (const [obj, hx, hy] of sets) {
        for (const [sx, sy] of [[-1, 1], [1, 1], [1, -1], [-1, -1]]) {
          const [x, y] = proj(obj, sx * hx, sy * hy, 0);
          if (p >= 1.2) { l = Math.min(l, x); t = Math.min(t, y); r = Math.max(r, x); b = Math.max(b, y); }
          pl = Math.min(pl, x); pt = Math.min(pt, y); pr = Math.max(pr, x); pb = Math.max(pb, y);
        }
      }
      per[p] = [pl, pt, pr, pb].map(Math.round);
    }
    // the receipt at rest: just clear of the page's right edge, but its right edge
    // stays left of limitRight (the text column less 48 px) at every aspect
    pose(6.7);
    const half = (W * 0.42 * RECEIPT_SCALE) / 2;
    const scr = (x) => proj(g, x + half, H * 0.06, 0.5)[0];
    let x1 = PW / 2 + 0.26 + half; // left edge just past the query flags' tips (page edge + 0.26)
    if (limitRight && scr(x1) > limitRight) {
      let lo = 0, hi = x1;
      for (let k = 0; k < 30; k++) {
        const mid = (lo + hi) / 2;
        if (scr(mid) > limitRight) hi = mid; else lo = mid;
      }
      x1 = lo;
    }
    this._rcXEnd = x1;
    g.position.y = keep.py;
    g.scale.setScalar(keep.sc);
    g.rotation.set(keep.rx, keep.ry, 0);
    this._probing = true;
    this.setChapterProgress(keep.p);
    this._probing = false;
    cam.lookAt(this._look);
    cam.updateMatrixWorld();
    g.updateMatrixWorld(true);
    return { dlg: { x: dx, y: dy }, book: { l, t, r, b }, per };
  }

  // The chip's lettering centre in canvas CSS px (app.js measures the working
  // paper's clause cell on load and resize, not per frame, so no layout is read
  // while scrolling).
  setRowTarget(x, y) {
    this._rowPx.x = x;
    this._rowPx.y = y;
    this._rowPx.ok = true;
  }

  // Travel of the hero flag (2.50-2.78), planned in SCREEN space so that it can
  // be steered: a quadratic bezier from the present point (projected each frame,
  // the camera is moving) out to the right of the page and down into the clause
  // cell, at constant depth along the view axis, at a near-constant speed
  // (trapEase: peak = 1.25 x mean). The flag grows until its lettering is
  // HERO_FONT px, the chip's size, and turns to face the camera, so it lands on
  // the chip and is replaced by it, never by a blank block.
  _placeHeroTravel() {
    const cam = this.camera, f = this.heroFlag, g = this.fileGroup, rp = this._rowPx;
    cam.updateMatrixWorld();
    g.updateMatrixWorld();
    const t = trapEase(this._heroTravel, 0.06), ts = smoothstep(0, 1, this._heroTravel);
    const P0 = this._v0.copy(this.flagPresentPos);
    g.localToWorld(P0);
    const fwd = cam.getWorldDirection(this._v1);
    const d0 = this._v2.copy(P0).sub(cam.position).dot(fwd);
    const q = this._v2.copy(P0).project(cam);
    const cw = this._cw, ch = this._ch;
    const sx = ((q.x + 1) / 2) * cw, sy = ((1 - q.y) / 2) * ch;
    const ex = rp.ok ? rp.x : sx, ey = rp.ok ? rp.y : sy + 300;
    const cx = Math.max(sx, ex) + ch * 0.04, cy = sy + (ey - sy) * 0.5;
    const a = 1 - t;
    const x = a * a * sx + 2 * a * t * cx + t * t * ex, y = a * a * sy + 2 * a * t * cy + t * t * ey;
    const w = this._v2.set((x / cw) * 2 - 1, 1 - (y / ch) * 2, 0.5).unproject(cam).sub(cam.position);
    w.multiplyScalar(d0 / w.dot(fwd)).add(cam.position);
    f.position.copy(w);
    g.worldToLocal(f.position);
    this._qa.setFromEuler(this._eul.set(0, -0.08, 0.03));
    g.getWorldQuaternion(this._qb);
    this._qb.invert().multiply(cam.quaternion);
    f.quaternion.slerpQuaternions(this._qa, this._qb, ts);
    const kpx = ch / (2 * Math.tan((cam.fov * Math.PI) / 360) * d0); // px per world unit at the flag's depth
    f.scale.setScalar(1 + (HERO_FONT / (FLAG_LETTER * kpx) - 1) * ts);
  }

  _applyFlag(f) {
    const u = f.userData;
    if (u.axis === 'page') {
      // query flags glued flat to a page scale in rather than slide, since
      // there is no solid geometry for them to spring clear of
      f.scale.setScalar(Math.max(0.001, u.amt));
      return;
    }
    if (f === this.heroFlag) {
      if (this._heroOn) return;
      f.scale.setScalar(1);
    }
    // fore-edge flags: userData.out (0.39-0.41 by lane) is the full
    // world-space distance needed to clear the closed cover's edge. Until the
    // load entrance is done the spring scales it; afterwards it is 1.
    const ent = this._entranceDone ? 1 : this._elapsed > u.delay ? u.spring.pos : 0;
    f.position.set(u.rest.x + u.amt * ent * u.out, u.rest.y, u.rest.z);
    f.rotation.set(0, -0.12, 0);
  }

  _applyAllFlags() {
    const L = this._flagList;
    for (let k = 0; k < L.length; k++) this._applyFlag(L[k]);
  }

  setPointer(nx, ny) {
    this.pointerTarget.x = nx;
    this.pointerTarget.y = ny;
  }

  // one composed still per chapter, used only for prefers-reduced-motion.
  // Each is the chapter's payoff (index + 0.85), except chapter 2, whose
  // payoff is the flag presented beside the amount (2.50), before it becomes a row.
  renderStill(chapterIndex) {
    this.ensureChapter(chapterIndex);
    this._entranceDone = true;
    this.setChapterProgress(chapterIndex === 2 ? 2.5 : chapterIndex === 3 ? 3.71 : chapterIndex + 0.85);
    this._applyAllFlags();
    this.fileGroup.rotation.x = 0;
    this.fileGroup.rotation.y = this._openRotY ?? this.baseRotY;
    this.fileGroup.position.y = this._entranceBaseY;
    this.fileGroup.scale.setScalar(this._entranceBaseScale);
    this._placeContactShadow(1);
    this.camera.lookAt(this._look);
    this._resizeToDisplay();
    if (this._heroOn && this._heroTravel > 0) this._placeHeroTravel();
    this.renderer.render(this.scene, this.camera);
  }

  // The canvas may be wider than its box (desktop: it spans the whole pinned
  // story so nothing is clipped at the box edge). Then the projection is
  // off-centre: the frustum of a canvas as wide as the box, centred on the box,
  // is a window into a wider virtual image, so the object keeps its old size
  // and place and the extra width shows what used to be cut. W = canvas width,
  // f = box/canvas: fullW = W(2-f), offset W(1-f). With no split, a plain projection.
  _resizeToDisplay() {
    const c = this.canvas;
    const w = c.clientWidth || 1;
    const h = c.clientHeight || 1;
    const par = c.parentElement;
    const bw = par ? par.clientWidth : w;
    const f = bw > 0 && bw < w * 0.98 ? bw / w : 1;
    const ratio = this.renderer.getPixelRatio();
    if (c.width !== Math.round(w * ratio) || this._lastH !== h || this._lastF !== f) {
      this.renderer.setSize(w, h, false);
      const cam = this.camera;
      if (f < 1) {
        const fw = w * (2 - f);
        cam.aspect = fw / h;
        cam.setViewOffset(fw, h, w * (1 - f), 0, w, h);
      } else {
        cam.clearViewOffset();
        cam.aspect = w / h;
      }
      cam.updateProjectionMatrix();
      this._lastH = h;
      this._lastF = f;
      this._cw = w;
      this._ch = h;
    }
  }

  // One frame of the scene. The page drives this from GSAP's ticker, right
  // after ScrollTrigger updates, so the file and the scroll never
  // drift a frame apart. Without a ticker, start() runs its own rAF loop.
  frame() {
    if (!this._running) return;
    const dt = this.clock.getDelta();
    this._elapsed += dt;
    const e = this._entranceSpring.update(dt);
    this.fileGroup.position.y = this._entranceBaseY - 0.25 * (1 - e);
    this.fileGroup.scale.setScalar(this._entranceBaseScale * (0.96 + 0.04 * e));
    if (!this._entranceDone) {
      if (this._fired && this._elapsed > this._fireT + 3) this._entranceDone = true;
      else for (let k = 0; k < this.flags.length; k++) this.flags[k].userData.spring.update(dt);
    }
    this._applyAllFlags();
    // time-based smoothing (frame-rate independent), scaled down and capped
    // once the file is open: parallax that tilts the voucher while it's
    // being read swims the very text the reader is trying to hold still.
    const lerpF = 1 - Math.exp(-dt * 6);
    this.pointer.x += (this.pointerTarget.x - this.pointer.x) * lerpF;
    this.pointer.y += (this.pointerTarget.y - this.pointer.y) * lerpF;
    const parallax = 1 - 0.8 * this._openAmount;
    this.fileGroup.rotation.x = MathUtils.clamp(this.pointer.y * 0.08 * parallax, -0.035, 0.035);
    this.fileGroup.rotation.y = (this._openRotY ?? this.baseRotY) + MathUtils.clamp(this.pointer.x * 0.1 * parallax, -0.035, 0.035);
    this._placeContactShadow(e);
    this.camera.lookAt(this._look);
    this._resizeToDisplay();
    if (this._heroOn && this._heroTravel > 0) this._placeHeroTravel();
    this.renderer.render(this.scene, this.camera);
    if (!this._firstFrame) {
      this._firstFrame = true;
      if (this.onFirstFrame) this.onFirstFrame();
    }
  }

  start() {
    if (this._running) return;
    this._running = true;
    this.clock.start();
    if (!this._enteredOnce) {
      this._enteredOnce = true;
      this._entranceSpring.target = 1; // fires once; a later stop/start (e.g. scrolling the canvas off-screen) never replays it
    }
    if (this.externalTicker) return;
    const loop = () => {
      if (!this._running) return;
      this.frame();
      this._raf = requestAnimationFrame(loop);
    };
    this._raf = requestAnimationFrame(loop);
  }

  stop() {
    this._running = false;
    if (this._raf) cancelAnimationFrame(this._raf);
    this._raf = null;
  }

  dispose() {
    this.stop();
    this.renderer.dispose();
  }
}
