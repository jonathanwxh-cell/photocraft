import json
import pathlib
import subprocess

# Verification-only setup: configure the existing snapshot example's starting tool options.
source = pathlib.Path('crates/ui-egui/examples/snapshot.rs')
text = source.read_text()
needle = '            app.background_jobs = background_jobs;'
assert needle in text
text = text.replace(needle, '''            app.ui.tool_options.marquee_style = "fixedRatio".into();
            app.ui.tool_options.marquee_width = 16.0;
            app.ui.tool_options.marquee_height = 9.0;
''' + needle)
source.write_text(text)
subprocess.run(['cargo', 'build', '-p', 'photocraft-ui-egui', '--example', 'snapshot'], check=True)
out = pathlib.Path('marquee-evidence')
out.mkdir(exist_ok=True)
base = [
 ['engine.execute', {'command':'file.new','params':{'width':400,'height':300}}],
 ['engine.execute', {'command':'edit.fill','params':{'color':'#778899'}}],
 ['ui.set', {'tool':'rectMarquee'}],
]
for name, end in [('left-edge', [100,110]), ('multiple-edges', [500,250])]:
 start = [-100,20] if name == 'left-edge' else [-40,-30]
 events = [{'kind':'down','x':start[0],'y':start[1]}, {'kind':'move','x':end[0],'y':end[1]}]
 for stage in ['preview','committed']:
  sequence = list(events)
  if stage == 'committed': sequence.append({'kind':'up','x':end[0],'y':end[1]})
  script = base + [['ui.pointer', {'events':sequence}], ['ui.inspect', {}]]
  result = subprocess.run(['target/debug/examples/snapshot','--size','1200x800','--scale','1','--safe-gpu','--out',str(out/(name+'-'+stage+'.png')),'--script',json.dumps(script)],capture_output=True,text=True)
  log = result.stdout+result.stderr
  (out/(name+'-'+stage+'.log')).write_text(log)
  result.check_returncode()
  if '"ok":false' in log: raise RuntimeError('Snapshot control failed: '+name+'-'+stage)
