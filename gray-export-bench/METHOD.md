# Gray export conversion-stage benchmark

Temporary audit harness, not part of the PR. Rust release build, 6000 x 4000 (24 MP) opaque grayscale F32 codec image tagged with PhotoCraft's linear-gray ICC profile. Horizontal samples repeat 0.05 + x/7000. Input construction is outside timing.

The helpers in src/helpers.rs are copied verbatim from crates/io/src/flat.rs: try_buffer, map_bands, convert_rgb, convert_gray, convert_from. Before and after run in the same compilation unit with the same input. The before branch tries convert_rgb (returns None for grayscale), then encodes unchanged samples to TGA. The after branch adds the actual new convert_gray call targeting Builtin::SGray, then encodes the display-encoded samples to TGA. Both run codec fidelity warnings and encoding; final bytes are black-boxed. The grayscale TGA adjustment helper returns its input without work in production and is omitted here.

Scope: profile conversion plus TGA encoding only. The unchanged document_to_image step, document allocation, UI, filesystem write, and textual warning formatting are not measured. The result is a cost measurement of restoring correct color encoding, not an improvement claim, full export timing, or Photoshop acceptance result.

One warm-up per branch, then six paired samples with order alternated; report median of each six. The standalone manifest resolves its own dependencies and uses the repository codec/CMS source paths. Executed on 12 October 2026 as one sequential release-harness invocation. Recorded medians: unchanged encoding 241.740 ms; corrected encoding 524.155 ms. Raw paired samples are in measurements.log. Background system load was not instrumented.


## Reproduction and provenance

Place this `gray-export-bench` directory beside a checkout named `photocraft` (the manifest's relative dependency paths require these sibling names). Use PhotoCraft upstream commit `c20b81b25168057e9cbffa2f45c36ec02dacf756`, or the grayscale fix commit `f90cf56def5e800e9d04c9d57aff9c4b1777f974` from `jonathanwxh-cell/photocraft`. Codec/CMS sources are unchanged by that fix. With a Rust toolchain installed, run from the parent directory:

```sh
cargo run --release --locked --manifest-path gray-export-bench/Cargo.toml
```

The measured binary was built in the local release profile with the included Cargo.lock. The original validation tree also held independent group-transform and UI-preview fixes; neither is in this harness's dependency path. No source checkout modification is needed. The harness deliberately contains both old and new branches to avoid compiler/profile differences between executables.

`src/helpers.rs` is copied from PhotoCraft's `crates/io/src/flat.rs` at the above baseline. Original synthetic harness code and fixture: jonathnwxh, 2026, prepared with OpenAI Codex assistance. Harness code is contributed under MIT; copied PhotoCraft helpers retain Copyright (c) 2026 ArtCraft Team and the PhotoCraft contributors. See the evidence branch's LICENSE-MIT. Dependencies retain their respective licenses.
