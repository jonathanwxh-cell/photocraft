# Pattern Preview Clone Stamp visual verification

Fixed source: 29cf3f7063ea89a6691e575945b5d24b9aad01c8.
Unmodified upstream source: 6f85054288a564f9fbbccb99268ffdb34e581584.

Synthetic 100x60 RGB document with a red source patch. Clone Stamp source (10,10), destination (98,30), 16px brush, Pattern Preview enabled, zoom 5. Native egui offscreen snapshot, Windows/DX12, 1200x850, scale 1. Both builds received the same scripted stroke.

- upstream-after.png: unmodified upstream clips the cloned mark at the repeating tile boundary.
- before.png: fixed build, before drawing.
- live.png: fixed build, pointer down; the mark crosses the tile seam.
- after.png: fixed build, pointer released; committed pixels preserve the live shape.

The inputs use the native control path, not physical mouse testing. No exact Photoshop pixel parity is claimed. Images were visually inspected.

Screenshots contain only application UI and a synthetic image, with no personal documents or third-party image assets.
