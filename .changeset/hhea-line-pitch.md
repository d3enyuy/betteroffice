---
'@betteroffice/docx': patch
'@betteroffice/rust-crates': patch
---

Pitch single-spaced lines on the font's hhea line height. Word's line pitch is `hhea(ascender - descender + lineGap)` in both directions of divergence — measured on Word 16.112 across ten faces — so the delta that lifts the OS/2 win box onto it is signed rather than clamped at zero like GDI's `tmExternalLeading`. Faces whose win box overruns their hhea line height no longer measure too tall; Caladea, the bundled Cambria substitute, drops from 1.3000 em to 1.1500 em against real Cambria's 1.1724 em.
