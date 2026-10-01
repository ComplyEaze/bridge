// app.js — Direction C "Flagged" v2: one scroll-driven master update
// drives the 3D scene, the chapter panels and the HTML "props" together.
// Perf pass (owner, 29 Sep): the hero is plain HTML, visible at first
// paint; the WebGL scene builds only after window `load`, inside an idle
// callback, behind a CSS poster, and fades in once its first frame renders.

import { FileScene, CHAPTER_COUNT, setTheme, nameResolve, slipWindow } from './scene.js';

const REDUCED = window.matchMedia('(prefers-reduced-motion: reduce)').matches;
const ric = window.requestIdleCallback
  ? window.requestIdleCallback.bind(window)
  : (cb) => setTimeout(() => cb({ didTimeout: true, timeRemaining: () => 0 }), 100);

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
function afterLoad(fn) {
  if (document.readyState === 'complete') fn();
  else window.addEventListener('load', fn, { once: true });
}

// Text changes in sequence, never double-exposed (motion spec v2, section B):
// the outgoing panel fades over 0.00-0.035 of a chapter past the boundary with
// a 14 px lift, then the incoming one fades in over 0.03-0.10 from 14 px below.
// The two opacities never sum above 1.1 (checked by _lh/builderM2/opsum.py);
// the only near-blank window is 0.03-0.035. The last panel (heading, proof and
// the Download button) holds full opacity through the pin's release and scrolls away with it.
const OUT_END = 0.035;
const IN_START = 0.03;
const IN_END = 0.1;
function panelOpacity(chapterPos, i) {
  const d = chapterPos - i;
  const fadeIn = i === 0 ? 1 : smoothstep(IN_START, IN_END, d);
  const fadeOut = i === CHAPTER_COUNT - 1 ? 1 : 1 - smoothstep(1, 1 + OUT_END, d);
  return clamp01(fadeIn) * clamp01(fadeOut);
}

// 142 statement lines: 139 vouchers are ready when the slips have lifted (the pile's own
// "139 ready" tag), and the three held rows join as their names are decided on screen
// (3.72-3.88), one by one, so the chapter-3 counter ends at 142; the read-back finds all 142
const BANK_READY = 139;
const BANK_TOTAL = 142;

async function boot() {
  const mobile = window.matchMedia('(max-width: 760px)').matches;

  if (REDUCED) {
    document.getElementById('story').hidden = true;
    afterLoad(() => buildStaticStory(mobile));
    document.addEventListener('ce:theme', () => buildStaticStory(mobile));
    return;
  }

  const panels = Array.from(document.querySelectorAll('.panel'));
  if (mobile) {
    // the synthetic-data line moves into each chapter's panel (chapters 1-7), one 12 px line under its words
    const cap = document.querySelector('.demo-caption');
    panels.forEach((panel) => {
      if (panel.dataset.chapter === '0') return;
      const p = document.createElement('p');
      p.className = 'panel-synthetic';
      p.setAttribute('aria-hidden', 'true');
      p.innerHTML = cap.innerHTML; // keeps the no-break span around "FY 2025-26"
      panel.appendChild(p);
    });
  }
  // a panel that cannot be seen must not be reachable by keyboard: it is `inert` (in the HTML for
  // chapters 1-7, so before this script runs too) while its opacity is under 0.05, toggled only when that changes
  const INERT_BELOW = 0.05;
  const panelInert = panels.map((panel) => panel.hasAttribute('inert'));
  const panelMeta = panels.map((panel) => ({
    panel,
    i: Number(panel.dataset.chapter),
    heading: panel.querySelector('h1, h2'),
    body: Array.from(panel.querySelectorAll('.proof, .proof-sub, .coming-line, .cta-row, .fineprint')),
  }));
  const propQuestion = document.getElementById('propQuestion');
  const propAgeing = document.getElementById('propAgeing');
  const propNames = document.getElementById('propNames');
  const propNameChips = propNames ? Array.from(propNames.querySelectorAll('.names-list li')) : [];
  const propDialog = document.getElementById('propDialog');
  const dialogPostBtn = propDialog ? propDialog.querySelector('.dbtn-post') : null;
  const storyEl = document.getElementById('story');
  let seamShown = -1;
  const propScrim = document.getElementById('propScrim');
  const counterBank = document.getElementById('counterBank');
  const counterReadback = document.getElementById('counterReadback');
  const propRow = document.getElementById('propRow');
  const rowSheet = propRow.querySelector('.row-sheet');
  const rowCells = Array.from(propRow.querySelectorAll('tbody .cell')); // left to right; the last is the clause prefix
  const rowSec = propRow.querySelector('.wp-sec');
  const canvasBox = document.getElementById('canvasBox');
  const panelsEl = document.getElementById('panels');
  const captionEl = canvasBox.querySelector('.demo-caption');
  // the synthetic-data line fades out as the pin releases, so it never rides up under the header over the install card
  const synthNotes = mobile ? Array.from(document.querySelectorAll('.panel-synthetic')) : [];
  let capBase = 1;
  let capExit = 0;
  function applyCaption() {
    setStyle(captionEl, 'opacity', String(Math.round(capBase * (1 - capExit) * 100) / 100));
    synthNotes.forEach((el) => setStyle(el, 'opacity', String(1 - capExit)));
  }

  let scene = null;
  let lastChapterPos = 0;
  const flags = { dom: true }; // the diagnostic overlay's "text/props" switch
  const scheduled = new Set();
  // counters are pure functions of chapterPos (the chapter-3 count rides the
  // slips lifting, 3.40-3.72, then the three decisions; read-back rides the ticks, 5.25-5.85), so
  // scrubbing back and forth moves the number with the scroll
  let bankShown = -1;
  let readbackShown = -1;
  function setCount(el, prev, v) {
    if (v !== prev) el.textContent = String(v);
    return v;
  }
  // write a style property only when its value changed (the scroll hot path)
  const lastStyle = new WeakMap();
  function setStyle(el, prop, val) {
    let m = lastStyle.get(el);
    if (!m) lastStyle.set(el, (m = {}));
    if (m[prop] === val) return;
    m[prop] = val;
    if (prop.startsWith('--')) el.style.setProperty(prop, val);
    else el.style[prop] = val;
  }

  // Layout that the scene needs, measured on load and resize (never per frame):
  // the chip in the working paper's clause cell (the flag lands on it), the
  // bubbles' places (from the file's projected outline), the dialog's origin (the
  // 12 Mar row on the statement) and the text column's edge (the receipt's limit).
  function measureLayout() {
    if (!scene) return;
    const box = canvasBox.getBoundingClientRect();
    const cs = getComputedStyle(panelsEl.querySelector('.panel'));
    const colLeft = mobile ? box.width : panelsEl.getBoundingClientRect().left - box.left + parseFloat(cs.paddingLeft);
    const m = scene.measure(mobile ? box.width - 12 : colLeft - 48);
    const cr = rowSec.getBoundingClientRect();
    // the flag's lettering sits a little right of the chip's centre (43 px pad + half the text)
    scene.setRowTarget(cr.left - box.left + cr.width * 0.536, cr.top - box.top + cr.height / 2);
    // the dialog rises from, and shrinks back into, the row it posts
    const dw = propDialog.offsetWidth, dh = propDialog.offsetHeight;
    propDialog.style.transformOrigin = `${(m.dlg.x - (propDialog.offsetLeft - dw / 2)).toFixed(1)}px ${(m.dlg.y - (propDialog.offsetTop - dh / 2)).toFixed(1)}px`;
    // the question above-right of the file, the answer below-right, 16 px clear of it
    // (desktop; on a phone the props keep their CSS places)
    if (!mobile) {
      const xr = Math.min(m.book.r + 40, colLeft - 24);
      const hdr = (document.getElementById('ceHeader') || { getBoundingClientRect: () => ({ bottom: 64 }) }).getBoundingClientRect().bottom;
      const qh = propQuestion.offsetHeight, qw = propQuestion.offsetWidth;
      propQuestion.style.left = `${(xr - qw).toFixed(1)}px`;
      propQuestion.style.right = 'auto';
      propQuestion.style.top = `${Math.max(hdr + 8, m.book.t - 16 - qh).toFixed(1)}px`;
      const aw = Math.min(320, xr - 20);
      propAgeing.style.width = `${aw.toFixed(1)}px`;
      propAgeing.style.left = `${(xr - aw).toFixed(1)}px`;
      propAgeing.style.bottom = 'auto';
      propAgeing.style.top = `${(m.book.b + 16).toFixed(1)}px`;
    }
    window.__ceLayout = m;
  }

  function masterUpdate(progress) {
    const chapterPos = progress * CHAPTER_COUNT;
    window.__ceProgress = progress; // read by the local scroll-latency test only
    lastChapterPos = chapterPos;

    if (scene) {
      scene.setChapterProgress(chapterPos);
      const next = Math.min(CHAPTER_COUNT - 1, Math.floor(chapterPos) + 1);
      if (!scheduled.has(next)) {
        scheduled.add(next);
        ric(() => { if (scene) scene.ensureChapter(next); });
      }
    }
    if (!flags.dom) return;

    // text: out with a 14 px lift, then in from 14 px below, heading then body
    // staggered by 0.02 of a chapter
    panelMeta.forEach(({ panel, i, heading, body }, k) => {
      const d = chapterPos - i;
      const op = panelOpacity(chapterPos, i);
      panel.style.opacity = String(op);
      const off = op < INERT_BELOW;
      if (off !== panelInert[k]) {
        panelInert[k] = off;
        panel.inert = off;
      }
      const headEnter = i === 0 ? 1 : smoothstep(IN_START, IN_END, d);
      const bodyEnter = i === 0 ? 1 : smoothstep(IN_START + 0.02, IN_END + 0.02, d);
      const exit = i === CHAPTER_COUNT - 1 ? 0 : smoothstep(1, 1 + OUT_END, d);
      if (heading) heading.style.transform = `translateY(${((1 - headEnter) * 14 - exit * 14).toFixed(1)}px)`;
      body.forEach((el) => { el.style.transform = `translateY(${((1 - bodyEnter) * 14 - exit * 14).toFixed(1)}px)`; });
    });

    const p = chapterPos;
    // the caption stays with the story's last words and scrolls away with them at the pin's release; on a phone the panels carry it
    capBase = mobile ? 1 - smoothstep(0.95, 1.02, p) : 1;
    applyCaption();
    // chapter 1: the question comes in as the flags fold away (1.06-1.14), the
    // answer once the five rings are drawn (1.70-1.82); both leave with the
    // ageing page (1.90-2.02)
    const askOut = 1 - smoothstep(1.9, 2.02, p);
    setStyle(propQuestion, 'opacity', String(smoothstep(1.06, 1.14, p) * askOut));
    const ageIn = smoothstep(1.7, 1.82, p);
    setStyle(propAgeing, 'opacity', String(ageIn * askOut));
    setStyle(propAgeing, 'transform', `translateY(${((1 - ageIn) * 22).toFixed(1)}px)`);

    // chapter 2: the working paper. A card under the voucher, at full paper opacity from 2.40
    // with its dashed empty row (the emptiness says "waiting"); the 40A(3) flag flies into its
    // clause cell (2.46-2.70, scene.js) and crossfades into the chip there (2.69-2.73); the
    // row's other cells reveal from that cell, each cell's text start first (clip-path from the
    // right, 2.72-2.80, never a scale); the card then holds, complete, until 2.90 and is the first thing to
    // leave (2.90-2.98, sliding 16 px down), then the flags, then the page turn. On a phone a
    // row slides up instead.
    const cardOut = smoothstep(2.9, 2.98, p);
    const rowExit = 1 - cardOut;
    const reveal = smoothstep(2.72, 2.80, p);
    if (mobile) {
      const slide = smoothstep(2.5, 2.66, p);
      setStyle(propRow, 'opacity', String(slide * rowExit));
      setStyle(propRow, 'transform', `translateY(${((1 - slide) * 40 + cardOut * 16).toFixed(1)}px)`);
      setStyle(rowSec, 'opacity', '1');
    } else {
      setStyle(propRow, 'opacity', (smoothstep(2.4, 2.46, p) * rowExit).toFixed(3));
      setStyle(propRow, 'transform', `translateY(${(cardOut * 16).toFixed(1)}px)`);
      setStyle(rowSec, 'opacity', smoothstep(2.69, 2.73, p).toFixed(3));
      for (let k = 0; k < rowCells.length; k++) {
        // the last cell (the clause prefix) is nearest the chip: it reveals first
        const g = clamp01(reveal * 2.2 - (rowCells.length - 1 - k) * 0.3);
        setStyle(rowCells[k], 'clipPath', g >= 1 ? 'none' : `inset(0 ${((1 - g) * 100).toFixed(1)}% 0 0)`);
      }
    }
    if (reveal >= 1 !== rowSheet.classList.contains('is-filled')) rowSheet.classList.toggle('is-filled', reveal >= 1);

    // chapter 3: the names card (3.64-3.72), then the three names are decided one
    // after another (3.72-3.88), in the statement's row order. Every visual is a
    // smoothstep of chapterPos (--r: 0 undecided .. 1 decided), so the chips
    // reverse at scroll speed like everything else.
    const namesIn = smoothstep(3.64, 3.72, p);
    if (propNames) {
      // leaves (3.90-3.98, with the white flags) before the approval dialog arrives (4.04)
      setStyle(propNames, 'opacity', String(namesIn * (1 - smoothstep(3.9, 3.98, p))));
      setStyle(propNames, 'transform', `translateY(${((1 - namesIn) * 18).toFixed(1)}px)`);
    }
    propNameChips.forEach((li, k) => setStyle(li, '--r', nameResolve(p, k).toFixed(3)));

    // chapter 4: the approval dialog rises from the 12 Mar row (4.04-4.30, its
    // transform-origin is that row's projected point, measured in measureLayout), on a sine ease,
    // from 0.4 scale and fading in over the first half of the rise; it holds (4.30-4.44), is
    // pressed (4.44-4.50), and hands back to the row (4.50-4.70): it fades out while still at
    // 0.65+ of its size, so no miniature is ever parked on the statement, and the red tick takes over
    const rise = sineInOut((p - 4.04) / 0.26);
    const shrink = sineInOut((p - 4.5) / 0.2);
    const dlgOpacity = smoothstep(0, 0.5, rise) * (1 - smoothstep(0.25, 0.6, shrink));
    setStyle(propDialog, 'opacity', dlgOpacity.toFixed(3));
    if (dlgOpacity > 0 || p > 4) setStyle(propDialog, 'transform', `translate(-50%, -50%) scale(${(0.4 + 0.6 * rise * (1 - shrink)).toFixed(3)})`);
    if (propScrim) setStyle(propScrim, 'opacity', (dlgOpacity * 0.35).toFixed(3));
    const pressed = p >= 4.44 && p < 4.5;
    if (dialogPostBtn && pressed !== dialogPostBtn.classList.contains('is-pressed')) dialogPostBtn.classList.toggle('is-pressed', pressed);

    // the pin-exit seam (28vh wash into the close colour) exists only for the
    // exit: it greyed the 3D object through the whole story when always on
    const seam = Math.round(smoothstep(7.8, 8, chapterPos) * 100) / 100;
    if (seam !== seamShown) {
      seamShown = seam;
      storyEl.style.setProperty('--seam', String(seam));
    }

    // 0 -> 139 with the slips, then 140, 141, 142 as each name is decided (all scroll-driven and reversible)
    bankShown = setCount(counterBank, bankShown, Math.round(BANK_READY * slipWindow(chapterPos) + nameResolve(chapterPos, 0) + nameResolve(chapterPos, 1) + nameResolve(chapterPos, 2)));
    readbackShown = setCount(counterReadback, readbackShown, Math.round(BANK_TOTAL * smoothstep(5.25, 5.85, chapterPos)));
  }

  if (location.search.includes('debug=1')) window.__ceMaster = (chapterPos) => masterUpdate(chapterPos / CHAPTER_COUNT); // local checks only


  // ?smooth=<ms> (A/B, 30 Sep): the owner's numbers showed perfect frames at
  // ~130 Hz and zero trail, yet the motion still felt rough; 1:1 mapping hands
  // every uneven trackpad delta straight to the camera and props. With
  // smooth > 0, the ANIMATED position follows the scroll through a critically
  // damped spring (SmoothDamp, no overshoot) with that smoothing time. The
  // scroll itself and the pin stay native, unlike Lenis, which delayed the
  // scroll and made the whole page trail. smooth=0 (default) is 1:1.
  const smoothMs = Math.max(0, Math.min(400, Number(new URLSearchParams(location.search).get('smooth')) || 0));
  const follow = { x: 0, v: 0, target: 0, primed: false };
  function onProgress(progress) {
    follow.target = progress * CHAPTER_COUNT;
    if (!smoothMs || !follow.primed) {
      follow.primed = true;
      follow.x = follow.target;
      follow.v = 0;
      masterUpdate(progress);
    }
  }
  const st = smoothMs / 1000;
  function stepFollow(deltaMs) {
    {
      if (!follow.primed) return;
      const change = follow.x - follow.target;
      if (Math.abs(change) < 1e-5 && Math.abs(follow.v) < 1e-4) return;
      const dt = Math.min(deltaMs, 50) / 1000;
      const omega = 2 / st;
      const k = omega * dt;
      const decay = 1 / (1 + k + 0.48 * k * k + 0.235 * k * k * k);
      const temp = (follow.v + omega * change) * dt;
      follow.v = (follow.v - omega * temp) * decay;
      let out = follow.target + (change + temp) * decay;
      if (change < 0 === out > follow.target) {
        out = follow.target; // never overshoot the scroll
        follow.v = 0;
      }
      follow.x = out;
      masterUpdate(out / CHAPTER_COUNT);
    }
  }

  // Native scrolling and CSS sticky pinning (no GSAP, no ScrollTrigger, 30 Sep):
  // the OS supplies trackpad momentum, the compositor holds the pin, and one
  // rAF loop reads scrollY, drives the story (through the ?smooth follower
  // when set) and then renders the WebGL frame, in that order. `story` keeps
  // the start/end/progress shape the diagnostic overlay reads.
  const stickyEl = storyEl.querySelector('.story-sticky');
  const story = { start: 0, end: 1, progress: 0 };
  function measureStory() {
    const top = storyEl.getBoundingClientRect().top + window.scrollY;
    story.start = top;
    story.end = top + Math.max(1, storyEl.offsetHeight - stickyEl.offsetHeight);
  }
  measureStory();
  let sceneFrame = null;
  let lastTs = 0;
  let lastRead = -1;
  function tick(ts) {
    const deltaMs = lastTs ? ts - lastTs : 16.7;
    lastTs = ts;
    const p = clamp01((window.scrollY - story.start) / (story.end - story.start));
    if (p !== lastRead) {
      lastRead = p;
      story.progress = p;
      onProgress(p);
    }
    // past the pin's release (p is clamped at 1): fade the synthetic-data line over the next 30% of a screen
    const exit = Math.round(clamp01((window.scrollY - story.end) / (0.3 * window.innerHeight)) * 100) / 100;
    if (exit !== capExit) {
      capExit = exit;
      applyCaption();
    }
    if (smoothMs) stepFollow(deltaMs);
    if (sceneFrame) sceneFrame();
    requestAnimationFrame(tick);
  }
  requestAnimationFrame(tick);
  window.addEventListener('resize', measureStory);

  // the install card is already in (opaque) as its top comes within 30% of the viewport's bottom,
  // before the pin releases, so it rises with the scroll from the first pixel (about 8.05) and the
  // page never lacks a call to action while panel 7 scrolls away
  const downloadCta = document.getElementById('downloadCta');
  const ctaIo = new IntersectionObserver((entries) => {
    if (entries.some((e) => e.isIntersecting)) {
      downloadCta.classList.add('is-in');
      ctaIo.disconnect();
    }
  }, { rootMargin: '0px 0px 30% 0px' });
  ctaIo.observe(downloadCta);

  // ---------------------------------------------------------- WebGL, deferred
  // One scene per colour world: the book's textures and lights are drawn for the theme in force
  // when it is built. Switching the theme (chrome.js announces it as "ce:theme") builds a new
  // scene on a fresh canvas, at the chapter the visitor is on, and drops the old one.
  const hoverPointer = window.matchMedia('(hover: hover) and (pointer: fine)').matches;
  let canvasIo = null;
  let buildSeq = 0;
  async function buildScene() {
    const seq = ++buildSeq;
    setTheme(document.documentElement.dataset.theme);
    let canvas = document.getElementById('fileCanvas');
    if (scene) {
      const old = scene;
      scene = null;
      sceneFrame = null;
      old.dispose();
      old.renderer.forceContextLoss();
      const fresh = canvas.cloneNode(false);
      canvas.replaceWith(fresh);
      canvas = fresh;
      canvasBox.classList.remove('is-ready');
    }
    const built = new FileScene(canvas, { mobile, externalTicker: true });
    scene = built;
    measureLayout();
    built.setChapterProgress(lastChapterPos);
    // the first render would compile every visible shader in the same task
    // as the build (one ~420 ms long task at 4x CPU): yield, then compile
    // them in parallel off the main thread (KHR_parallel_shader_compile)
    // before the first frame; the CSS poster holds the space meanwhile
    await new Promise((resolve) => setTimeout(resolve, 0));
    try {
      await built.renderer.compileAsync(built.scene, built.camera);
    } catch (e) {
      /* no async compile: the first frame compiles them, as before */
    }
    if (seq !== buildSeq) return; // the theme changed again while this one compiled
    sceneFrame = () => built.frame();
    built.onFirstFrame = () => canvasBox.classList.add('is-ready');
    built.start();
    // fire 250ms after the entrance spring settles (~700ms), not as soon
    // as the scene exists: the object should land before it starts flagging
    window.setTimeout(() => built.fireFlags(), 950);
    // build every later chapter's textures in idle time now, one per idle
    // slot, so nothing is created or uploaded while the visitor scrolls
    [1, 3, 5, 6].forEach((c, k) => window.setTimeout(() => ric(() => { if (scene === built) built.ensureChapter(c); }), 250 + k * 250));

    if (canvasIo) canvasIo.disconnect();
    canvasIo = new IntersectionObserver((entries) => {
      entries.forEach((entry) => (entry.isIntersecting ? built.start() : built.stop()));
    }, { threshold: 0.01 });
    canvasIo.observe(canvas);
  }

  afterLoad(() => {
    ric(async () => {
      try {
        await document.fonts.ready;
      } catch (e) {
        /* fonts API unavailable; proceed with fallback stack */
      }
      await buildScene();
      if (hoverPointer) {
        window.addEventListener('pointermove', (e) => {
          if (!scene) return;
          const nx = (e.clientX / window.innerWidth) * 2 - 1;
          const ny = (e.clientY / window.innerHeight) * 2 - 1;
          scene.setPointer(nx, ny);
        });
      }
      window.addEventListener('resize', measureLayout);
      document.addEventListener('ce:theme', () => buildScene());
    });
  });
}

// ---------------------------------------------------------- reduced motion
// One WebGL render per chapter, captured as a still image, then the
// renderer is torn down: the page that ships to the visitor is pure HTML
// and eight <img> stills, no canvas, no scroll listener, no rAF loop. Each
// still sits beside a clone of the chapter's own live .panel, so the hero
// headline, the Coming tags and the download button are the page's own.
async function buildStaticStory(mobile) {
  try {
    await document.fonts.ready;
  } catch (e) {
    /* proceed with fallback fonts */
  }
  setTheme(document.documentElement.dataset.theme);
  const host = document.getElementById('staticStory');
  host.replaceChildren();
  const panels = Array.from(document.querySelectorAll('#story .panel'));
  const tmpCanvas = document.createElement('canvas');
  const scene = new FileScene(tmpCanvas, { mobile });
  tmpCanvas.style.width = mobile ? '100%' : '640px';
  tmpCanvas.style.height = mobile ? '46vh' : '520px';
  document.body.appendChild(tmpCanvas);

  const frag = document.createDocumentFragment();
  for (let i = 0; i < CHAPTER_COUNT; i++) {
    scene.renderStill(i);
    const img = document.createElement('img');
    img.src = tmpCanvas.toDataURL('image/png');
    img.alt = '';
    img.setAttribute('aria-hidden', 'true');
    img.className = 'static-still';
    const fig = document.createElement('figure');
    fig.className = 'static-figure';
    fig.appendChild(img);
    const cap = document.createElement('figcaption');
    const copy = panels[i].cloneNode(true);
    copy.removeAttribute('style');
    copy.removeAttribute('inert'); // the stills page shows every chapter at once
    copy.querySelectorAll('[style]').forEach((el) => el.removeAttribute('style'));
    // counters finish where the animated page leaves them (chapter 3's still is taken with the
    // 139 slips up and the three names still waiting, so its counter reads 139; the read-back
    // reads 142 of 142); ids must stay unique
    copy.querySelectorAll('.counter').forEach((el) => {
      el.textContent = String(el.id === 'counterReadback' ? BANK_TOTAL : BANK_READY);
      el.removeAttribute('id');
    });
    cap.appendChild(copy);
    if (i === 2) {
      // the chapter's Coming feature, shown: the working paper's row 1, filled (the live card, cloned)
      const wp = document.getElementById('propRow');
      if (wp) {
        const row = wp.cloneNode(true);
        row.removeAttribute('id');
        row.removeAttribute('style');
        row.removeAttribute('aria-hidden');
        row.querySelectorAll('[style]').forEach((el) => el.removeAttribute('style'));
        row.querySelector('.row-sheet').classList.add('is-filled');
        cap.appendChild(row);
      }
    }
    if (i === 4) {
      // the approval dialog, so the still-only page shows what you approve (the live replica, cloned)
      const dlg = document.querySelector('#propDialog .dialog-replica');
      if (dlg) cap.appendChild(dlg.cloneNode(true));
    }
    fig.appendChild(cap);
    frag.appendChild(fig);
  }
  host.appendChild(frag);
  host.hidden = false;
  scene.dispose();
  document.body.removeChild(tmpCanvas);

  const cta = document.getElementById('downloadCta');
  cta.style.opacity = '1';
  cta.style.transform = 'none';
}

boot();
