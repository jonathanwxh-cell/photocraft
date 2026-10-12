# PR #4257: rotated canvas auto-scroll

The regression test `rotated_marquee_scrolls_on_the_axis_that_is_visibly_clipped`
fails on PR head `3ff1e4650512b0e102889b79325003ebda4e2174`: the camera moves
zero document pixels while a marquee is held below a 90-degree rotated canvas.
The same test passes with the rotation follow-up, with Overscroll both enabled
and disabled. It checks movement, the final image-edge limit, and stability after
release. The two original marquee integration tests also pass.

These are offscreen renders from the real PhotoCraft app using egui pointer
events and wgpu's DX12 backend on Windows. The image is a synthetic white
document. This is automated verification, not a physical mouse/pen user study.

- `before.png`: original PR plus the new test, just before its failing assertion.
- `after-overscroll-on.png`: corrected auto-scroll with Overscroll enabled.
- `after-overscroll-off.png`: corrected auto-scroll with Overscroll disabled.

The before/after marquee size tooltip changes from 301 to 379 document pixels;
the drag now reaches content that was outside the original viewport.

Reproduce the integration checks:

```powershell
$env:WGPU_BACKEND = 'dx12'
$env:CRAFT_UI_VISUAL_FIXTURES = '<absolute output directory>'
cargo test -p photocraft-ui-egui --test marquee_autoscroll -- --nocapture --test-threads=1
```

The ten helper unit tests cover speed/limits, fitted axes, overscroll, invalid
input, 45-degree movement, mirrored quarter turns, high-DPI scale, and pixel
aspect correction. Their normal Cargo entry point is:

```text
cargo test -p photocraft-ui-egui --lib canvas_autoscroll::tests
```

For this follow-up they were executed by compiling the actual helper source as
a small test harness linked to the newly built UI library. All ten passed. This
avoided a second build of the entire UI crate as a test library. The full UI
crate/workspace suite was not rerun for this follow-up.
