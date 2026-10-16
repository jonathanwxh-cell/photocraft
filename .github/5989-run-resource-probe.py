"""Cloud-only driver; run sequentially on baseline and candidate, never locally."""
import json
import os
from pathlib import Path
import subprocess
import sys

if os.environ.get('GITHUB_ACTIONS') != 'true' or sys.platform != 'linux':
    raise SystemExit('This probe requires the authorized Linux cloud runner')
label, probe_path, output_dir = sys.argv[1:]
out = Path(output_dir)
out.mkdir(parents=True, exist_ok=True)
source = Path('crates/ui-egui/src/liquify_ui.rs')
original = source.read_text()
probe = Path(probe_path).read_text()
assert original.rstrip().endswith('}')
source.write_text(original.rstrip()[:-1] + '\n' + probe + '\n}\n')
try:
    # Compilation is deliberately outside the timed execution and RSS measurement.
    build = subprocess.run(['cargo', 'test', '--release', '-p', 'photocraft-ui-egui',
                            '--lib', '--locked', '--no-run', '--message-format=json'],
                           stdout=subprocess.PIPE, text=True, check=True)
    executable = None
    for line in build.stdout.splitlines():
        item = json.loads(line)
        if item.get('reason') == 'compiler-artifact' and item.get('executable') and item.get('profile', {}).get('test'):
            executable = item['executable']
    if not executable:
        raise SystemExit('No test executable produced')
    for mode in ['unselected', 'selected', 'zoomed']:
        for sample in range(1, 4):
            stem = out / f'{label}-{mode}-{sample}'
            env = dict(os.environ, LIQUIFY_PROBE_MODE=mode)
            result = subprocess.run(['/usr/bin/time', '-v', '-o', str(stem)+'.time.txt',
                                     executable, '--ignored', '--nocapture',
                                     'liquify_selection_resource_probe_5989'],
                                    env=env, text=True, capture_output=True)
            (Path(str(stem)+'.log')).write_text(result.stdout + result.stderr)
            print(result.stdout, flush=True)
            if result.returncode or '1 passed; 0 failed' not in result.stdout or 'RESOURCE mode=' not in result.stdout:
                raise SystemExit(f'{label}/{mode}/{sample}: resource probe failed or did not execute')
finally:
    source.write_text(original)
