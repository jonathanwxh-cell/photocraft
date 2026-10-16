"""Cloud-only visual receipt using synthetic pixels, never personal images."""
import json
import pathlib
import struct
import subprocess
import zlib

# Verification-only instrumentation: expose successful control replies so an absent
# dialog cannot silently turn the intended warp into a normal canvas brush stroke.
snapshot = pathlib.Path('crates/ui-egui/examples/snapshot.rs')
source = snapshot.read_text()
needle = 'let outcome = handle(harness.state_mut(), &ctx, &req);'
assert needle in source
source = source.replace(needle, needle + '\n        if let Outcome::Done(v) = &outcome { println!("CONTROL_REPLY {}", v); }')
snapshot.write_text(source)

out = pathlib.Path('/tmp/5989-visual')
out.mkdir(exist_ok=True)
width, height = 600, 400
rows = bytearray()
for y in range(height):
    rows.append(0)
    for x in range(width):
        cell = (x // 20 + y // 20) % 2
        rows.extend((220 if cell else 40, 100, 40 if cell else 220, 255))

def chunk(kind, data):
    return struct.pack('>I', len(data)) + kind + data + struct.pack('>I', zlib.crc32(kind + data))

fixture = out / 'fixture.png'
fixture.write_bytes(b'\x89PNG\r\n\x1a\n' + chunk(b'IHDR', struct.pack('>IIBBBBB', width, height, 8, 6, 0, 0, 0)) + chunk(b'IDAT', zlib.compress(rows)) + chunk(b'IEND', b''))

def command(name, params):
    return ['engine.execute', {'command': name, 'params': params}]

setup = [command('select.rect', {'x': 0, 'y': 0, 'width': 180, 'height': 400}), ['ui.menu.invoke', {'id': 'filter.liquify'}], ['ui.menu.invoke', {'id': 'filter.liquify', 'params': {'ui': {'tool': 'forwardWarp', 'size': 100}}}]]
for name, start in [('before', None), ('outside-selection', 360), ('inside-selection', 60)]:
    script = list(setup)
    if start is not None:
        script.append(['ui.pointer', {'events': [
            {'kind': 'down', 'x': start, 'y': 200, 'pressure': 1},
            {'kind': 'move', 'x': start + 60, 'y': 200, 'pressure': 1},
            {'kind': 'up', 'x': start + 60, 'y': 200},
        ]}])
    args = ['cargo', 'run', '-p', 'photocraft-ui-egui', '--example', 'snapshot', '--', '--size', '1200x800', '--scale', '1', '--open', str(fixture), '--out', str(out / (name + '.png')), '--script', json.dumps(script)]
    result = subprocess.run(args, text=True, capture_output=True)
    print(result.stdout)
    print(result.stderr)
    if result.returncode or '"ok":false' in result.stdout.replace(' ', '') or '"ok":false' in result.stderr.replace(' ', ''):
        raise SystemExit(f'{name}: snapshot or command failed')
    replies = [json.loads(line.removeprefix('CONTROL_REPLY ')) for line in result.stdout.splitlines() if line.startswith('CONTROL_REPLY ')]
    if not any(reply.get('ok') and reply.get('result', {}).get('liquify') is not None for reply in replies):
        raise SystemExit(f'{name}: no confirmed Liquify dialog')
    if not (out / (name + '.png')).is_file():
        raise SystemExit(f'{name}: missing screenshot')
