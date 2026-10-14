# Clipboard read failure and retry

Screenshots rendered by the ignored clipboard_error_and_retry_visual_evidence test at5e2b2b813d56020ae93968b734721040669cbb7d in run38582914206. Inspected: busy paste leaves destination unchanged and displays the read error; retry pastes an editable blue Shape and clears the error. This UI harness simulates read failure; actual Windows ClipboardOccupied is covered separately by busy_native_clipboard_preserves_copied_shape in full run38573558049.

Full verification base7250e6e and commit63535cd. Final sourceff92cb3 matches5e2b2b8 in all four changed Rust files. The only difference from full-tested Rust source is an additional empty-status assertion and explicit repaint in the visual test. No production changes followed the full run.

Original synthetic geometry; no external artwork. Screenshots generated for this contribution by jonathnwxh with OpenAI Codex assistance; CC0-1.0.
