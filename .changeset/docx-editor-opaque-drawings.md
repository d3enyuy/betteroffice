---
"@betteroffice/docx": patch
"@betteroffice/docx-react": patch
"@betteroffice/rust-crates": patch
---

Keep unmodeled inline drawings (OLE objects without image content, unreadable chart placements, drawing-owned compatibility wrappers) as their original markup through editor saves instead of dropping them, and save sessions whose chart runs predate drawing replay by restoring the placement from the source part or keeping the run out with a warning.
