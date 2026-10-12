# Selected-channel filter preview evidence

Before/after screenshots for PhotoCraft issue [#3949](https://github.com/storytold/photocraft/issues/3949), captured on 12 October 2026 by jonathnwxh with OpenAI Codex assistance.

The original synthetic fixture is a 640x480 document with an alpha channel containing a white left half and black right half. Gaussian Blur radius 30 is open with Preview enabled. The before image shows the unchanged sharp channel edge; the fixed image displays the blurred channel. No personal photos or proprietary Photoshop assets are used. The included public control script reproduces the fixture.

Captured with PhotoCraft's native offscreen snapshot example at 1440x900, scale 1, using Microsoft Basic Render Driver (DX12 WARP software rendering). This is visual regression evidence, not physical-GPU performance or a direct Photoshop comparison.

The baseline UI sources come from upstream c20b81b25168057e9cbffa2f45c36ec02dacf756. The fixed UI and proxy sources match a92e4bada9b8129c0264fbc089f5ce327602e4e7; the local validation tree also included independent group-transform and grayscale-export fixes. Those changes do not affect this synthetic alpha-channel fixture.

| Files | Attribution and rights |
|---|---|
| preview-script.json and original synthetic black/white fixture | jonathnwxh, 2026; contributed under PhotoCraft's MIT license |
| filter-channel-preview-before.png, filter-channel-preview-after.png | Captures of PhotoCraft's UI with that fixture; PhotoCraft code is Copyright 2026 ArtCraft Team and contributors, MIT/Apache-2.0. Existing UI assets and branding retain their original rights; see upstream ATTRIBUTION.md and brand license. |

Evidence is stored separately from the source PR to keep its patch focused.
