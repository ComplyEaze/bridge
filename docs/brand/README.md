# ComplyEaze Bridge logo: usage (one page)

The mark is the wordmark **ComplyEaze** with the **y of Comply drawn as an auditor's tick** (the
"tick-y"). The product lockup is **ComplyEaze Bridge**. The tick-y alone, in a rounded-square tile,
is the icon. Always use the supplied files; never retype the name or set the "y" from a font.

## Clear space and minimum size

- **Clear space:** keep empty space on every side at least the height of the lowercase "o" (the
  x-height) of the wordmark as used. For the icon, keep 1/8 of the tile width clear.
- **Minimum size, screen:** the lockup at 16 px cap height; the wordmark alone at 80 px wide; the icon at 16 px, using only `favicon.svg` or the
  16 px slot of the `.ico` (drawn on the pixel grid). Below 32 px, never use the auto-scaled tile.
- **Minimum size, print:** the lockup at 25 mm wide; the icon at 6 mm.

## Colours

| World | Use | Hex | Contrast |
| --- | --- | --- | --- |
| Cobalt | wordmark and tick on light | cobalt `#1f3fbf` on paper `#f6f8fb` | 7.8:1 |
| Cobalt | on dark | paper `#f6f8fb` on deep `#0e1d5c` | tick stays paper (no tint) |
| Cobalt | icon | paper `#f6f8fb` tick-y on a cobalt `#1f3fbf` tile | 7.8:1 |
| Red (bahi-khata) | header | cream `#fff5e8` on `#2a0907` | 17.1:1 |
| Red | cream page | ink `#2b0e0c` wordmark (15.5:1), pencil-red tick `#b3261e` (5.66:1) on `#fbeec9` | |
| Red | icon on light surfaces | cream tick-y on a cloth `#5a1410` tile | 12.6:1 |
| Red | icon on dark surfaces | cloth `#5a1410` tick-y on a cream `#fff5e8` tile | the cloth tile vanishes on dark (1.35:1) |
| Cobalt | reversed icon (on the dark cobalt banner) | cobalt tick-y on a paper `#f6f8fb` tile (`mark-reverse.svg`) | 7.8:1 |
| Any | one colour | black, white, cobalt, or ink `#0b1436`, used for the whole mark | ink on paper 16.9:1 |
| Supporting (not the mark) | taglines and rules on banners and social previews | `#465867` / `#d0d7de` (light), `#c9d2ec` / `#1c2d78` (cobalt dark), `#ead7c4` (red) | text only |

The pencil red `#b3261e` is the only red used for the tick. It is ΔE2000 11.5 from Tally's legacy red
`#ED1C24`; brighter reds such as `#e03a2f` sit within ΔE2000 3 of it, so they are not used in the mark.

## Do

- Use the SVG masters in `docs/brand/svg/`.
- Keep the tick the same colour as the word, except the pencil-red tick on the red world's cream page.
- Write the name as "ComplyEaze Bridge" in text; never bare "Bridge".
- Mention TallyPrime and Claude only in plain text ("for TallyPrime", "works with Claude"), never inside the mark.

## Don't

- **The tick never breaks out of its tile**, and it is never enlarged, rotated or detached from the word.
- **No saffron or green ticks**, no upside-down tick, and never set the mark next to "CA".
- No finding-flag colours (yellow, pink, green, orange, violet, cyan) anywhere in the mark; no gold.
- No stretching, outlines, shadows, gradients, glows or effects; no other typefaces for the wordmark.
- No separate cut or gap in the y (the "cut" variant is retired), no round-ended tick (retired).
- Never use the old Canva elephant or the "CE" stamp (both retired).
- Never use Tally's or Anthropic's logos, colours or shapes with the mark.

## Files

In this repository:
- `docs/brand/svg/`: the masters. `complyeaze-bridge-lockup-{cobalt,white,red-cream,red-page,ink,black}.svg`,
  `complyeaze-wordmark-*.svg`, `mark-{cobalt,red,red-ondark,reverse,black}.svg` (icon tiles),
  `glyph-{cobalt,white}.svg` (the tick-y alone), `favicon.svg` and `favicon-red.svg` (drawn on the 16 px grid),
  and `app-icon-macos.svg` (a macOS-style tile with margins, for a future native macOS icon).
- `src-tauri/app-icon.svg`: the desktop app icon source. Every file in `src-tauri/icons/` is rendered from it,
  except the 16 px entries of `icon.ico` and `icon.icns`, which use `docs/brand/svg/favicon.svg`.

Exported PNG and ICO files for the website, the Claude Desktop extension and GitHub (avatar, social preview)
are rendered from these SVGs and added where each surface is wired up.

## Type and licence

The letters are **Outfit** (SIL Open Font License 1.1, © The Outfit Project Authors), converted to
outlines; the tick-y is drawn geometrically for this mark.

ComplyEaze™ and ComplyEaze Bridge™, and the related logos and visual identity, are trade marks of SPMS Comply Eaze Solutions LLP (ComplyEaze). The Apache License 2.0 does not grant any right to use them (section 6 of the license); see TRADEMARKS.md. The logo and icon files — docs/brand/, src-tauri/app-icon.svg, src-tauri/icons/ and packaging/mcpb/icon.png — are not licensed under Apache-2.0: all rights in them are reserved. They are included so that official builds carry the ComplyEaze Bridge identity. A fork or modified build must replace them and use a different name.
