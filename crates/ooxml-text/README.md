# betteroffice-ooxml-text

Text shaping and measurement for the OOXML layout engines. Turns run text into
positioned glyphs and line-break decisions with no browser APIs in the loop.

- `font_store` — registry over raw font **bytes**, never font names, with
  `head`/`hhea`/`OS/2` metrics, cmap lookup, advance widths, and an ordered
  fallback-chain resolver
- `shape` — OpenType shaping through rustybuzz, returning cluster-mapped glyphs
  with advances and offsets scaled to the requested size
- `line_break` — UAX-14 break opportunities, surrogate-safe and CJK-aware
- `bidi` — paragraph-level Unicode Bidirectional Algorithm runs
- `word_metrics` — the Word-specific rules: single-spacing line boxes from OS/2
  win metrics, auto/exact/atLeast line rules, justification gating and space
  stretch, the `w:kern` threshold, and the `settings.xml` compat flags that feed
  them
- `outline` — glyph outline extraction as font-unit path commands, from the same
  skrifa bytes the metrics came from

Callers hand this crate font bytes plus a fallback chain of `FontId`s.
Resolving a `w:rFonts` name to bytes — embedded `.odttf`, bundled
metric-compatible faces, Local Font Access, browser-measured fallback — stays on
the host side, which keeps results deterministic and identical across web and
native shells.

No `wasm-bindgen` here by design; a thin facade can wrap it.

## The document grid

`w:docGrid` (ECMA-376 Part 1 §17.6.5) fits a line's content box — the
single-spaced box, before the `w:spacing` multiple scales it — to the section's
row pitch. The host supplies the pitch only for an activating grid type
(`lines`, `linesAndChars`, `snapToChars`), and paragraph/run `w:snapToGrid`
opt-outs plus pinned `exact`/`atLeast` heights suppress the fitting.

Word's own rule is a `ceil` to whole rows. Measured off Word 16.112's rasters
of two Chinese theses on a `linesAndChars` grid (326 and 312 twips):
single-spaced body lines sit at 1.00 grid rows, `w:line="360"` body lines at
1.50 — so the multiple scales the fitted row rather than rounding after it —
and cover-page lines whose content outgrows one row at exactly 2.00, 3.00 and
4.00 rows.

The two fittings differ in whether the height they are handed is trustworthy:

- `fill_grid_row_box` / `fill_grid_row` — **text**. Fills up to one row and
  leaves a taller content box at its natural height, deliberately stopping
  short of Word's `ceil`. A document's `linePitch` is authored against the real
  CJK face; the substituted faces this crate ships measure a few percent
  taller, which straddles the row boundary, and `ceil` would turn that
  few-percent metric error into a doubled line. Measured: the literal `ceil`
  took beihang-university-thesis to 45 pages and southeast-university-thesis to
  37, against Word's 39 and 31; the one-row fill lands on both exactly. Capping
  keeps what the grid is for — a short line filling its row — without betting
  page counts on a metric we do not have. Revisit once the shipped CJK faces
  carry Word's metrics.
- `snap_to_grid_rows` — an **image-dictated** box, where the image's footprint
  exceeds the ruled text height. Rounds up to whole rows, Word's literal rule.
  The height comes from `wp:extent`, so none of the font-metric uncertainty
  above applies, and leaving a 3.4-row image unrounded would push every
  following line off the grid.

Which fitting applies is decided by which metric determines the box: text while
the text box is the taller, `wp:extent` from the moment the image outgrows it.
The font-derived descent buffer added under an image rides along inside the
`ceil` because it is part of the box Word rounds.

Absolute grid-phase alignment against the page origin is not modelled.

Part of [BetterOffice](https://betteroffice.dev). Apache-2.0.
