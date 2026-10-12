# 24 MP selected-channel preview measurements

Executed on 12 October 2026 using PhotoCraft's native release snapshot example. Both versions ran sequentially on Windows using Microsoft Basic Render Driver, DX12 WARP (software GPU). Round 0 was a warm-up for each executable; rounds 1-3 were recorded. Order alternated by round: before/after on even rounds, after/before on odd rounds. Raw logs include warm-ups; measurements.json excludes them. Background system load was not instrumented.

The synthetic input is a 6000x4000 RGB/U8 document, with an alpha channel containing a vertical half-white/half-black boundary. A Gaussian Blur dialog is opened and radius values are set to 30, 31, 31, 32, 32, followed by two UI inspections. Default reduced previews give a 1500x1000 proxy; every recorded final perf record confirms 1,500,000 pixels. Setup rows are excluded from the reported dialog medians.

| Median milliseconds (three recorded runs) | Before | After |
|---|---:|---:|
| Radius 30 | 139.2 | 122.4 |
| Radius 31 | 145.7 | 124.8 |
| Unchanged radius 31 | 4.9 | 2.9 |
| Radius 32 | 264.3 | 175.2 |
| Unchanged radius 32 | 4.7 | 4.6 |

Snapshot's default inline jobs were used (no `--background-jobs`). Each timing row includes command execution and four UI steps. These are command-plus-preview/frame costs, not pure filter-kernel time, physical-GPU performance or interactive pointer latency. The before version incorrectly filters RGB and displays stale alpha; the after version does different, correct work. These numbers do not establish a like-for-like speedup. Repeated unchanged values reuse the completed preview and texture.

## Reproduction

Build `cargo build --release -p photocraft-ui-egui --example snapshot` in two checkouts, one at upstream `c20b81b25168057e9cbffa2f45c36ec02dacf756`, the other at fix commit `a92e4bada9b8129c0264fbc089f5ce327602e4e7` from `jonathanwxh-cell/photocraft`. The recorded final executable came from the combined validation tree also containing the independent transform and grayscale-export changes, which do not affect this fixture. Build both before measuring; use the same toolchain and stop competing builds. Hardware and environment changes will affect timings.

Set these paths to your two binaries and the included script, then run the following PowerShell in a scratch output directory. This creates only synthetic documents and local logs/screenshots.

```powershell
$before = '/path/to/before/target/release/examples/snapshot.exe'
$after = '/path/to/after/target/release/examples/snapshot.exe'
$fixture = Get-Content '/path/to/evidence/preview-24mp/script.json' -Raw
$env:SNAPSHOT_TIMING = '1'
try {
    foreach ($round in 0..3) {
        $order = if ($round % 2 -eq 0) { @('before','after') } else { @('after','before') }
        foreach ($version in $order) {
            $exe = if ($version -eq 'before') { $before } else { $after }
            & $exe --out "$version.png" --size 1440x900 --scale 1 --script $fixture *> "$version-$round.log"
            if ($LASTEXITCODE -ne 0) { throw 'Snapshot failed' }
        }
    }
} finally { Remove-Item Env:SNAPSHOT_TIMING }
```

Inspect both images: before retains a sharp alpha edge despite Preview being enabled; after displays the blur. CPU canvas testing would require `--safe-gpu` on both binaries; it was not used for these measurements.

Original script/fixture: jonathnwxh, 2026, prepared with OpenAI Codex assistance; contributed under MIT (see ../LICENSE-MIT). PhotoCraft source retains its existing licenses and attribution. Logs contain only synthetic task output and public renderer details; local output filenames are retained as relative paths.

Published log copies have trailing whitespace trimmed; timing values and perf payloads are unchanged.
