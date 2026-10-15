import json
import pathlib
import subprocess

out = pathlib.Path("selection-evidence")
out.mkdir(exist_ok=True)
base = [
    ["engine.execute", {"command": "file.new", "params": {"width": 400, "height": 300}}],
    ["engine.execute", {"command": "edit.fill", "params": {"color": "#778899"}}],
    ["ui.menu.invoke", {"id": "view.extras"}],
]
drag = ["ui.pointer", {"tool": "rectMarquee", "events": [
    {"kind": "down", "x": 100, "y": 80},
    {"kind": "move", "x": 240, "y": 190},
    {"kind": "up", "x": 240, "y": 190}
]}]
for name, script in [
    ("extras-hidden", base),
    ("new-selection-visible", base + [drag]),
    ("selection-hidden-again", base + [drag, ["ui.menu.invoke", {"id": "view.extras"}]]),
]:
    args = ["target/debug/examples/snapshot", "--size", "1200x800", "--scale", "1", "--safe-gpu", "--out", str(out / (name + ".png")), "--script", json.dumps(script)]
    result = subprocess.run(args, text=True, capture_output=True)
    (out / (name + ".log")).write_text(result.stdout + result.stderr)
    result.check_returncode()
    if '"ok":false' in result.stdout + result.stderr:
        raise RuntimeError("snapshot control command failed: " + name)
