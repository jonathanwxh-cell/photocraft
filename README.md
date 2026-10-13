# Pattern Preview Clone Stamp visual verification

Source: 29cf3f7063ea89a6691e575945b5d24b9aad01c8 in storytold/photocraft contribution branch investigate/pattern-preview-clone.

Synthetic 100x60 RGB document with a red source patch. Clone Stamp source (10,10), destination (98,30), 16px brush, Pattern Preview enabled, zoom 5. Native egui offscreen snapshot, Windows/DX12, 1200x850, scale 1.

- before.png: source patch before drawing the clone stroke.
- live.png: pointer down; periodic painted footprint crosses the tile seam.
- after.png: pointer released; committed pixels preserve the live painted shape.

These are stroke stages on corrected code, not an unfixed/fixed screenshot comparison. The regression test supplies the failing-before evidence. The inputs use the native control path, not physical mouse testing. No exact Photoshop pixel parity is claimed.

Screenshots contain only the application UI and a synthetic image; no personal documents or third-party image assets.
