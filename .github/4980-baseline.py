import pathlib, subprocess
base = "df055f3dcb0bb0312af97548ee6c6f9628f21dd7"
source = subprocess.check_output(["git", "show", base + ":apps/photocraft-web/src/web.rs"], text=True)
start = source.index("let mut files = Vec::with_capacity(picked.len());")
end = source.index("Some(FileDialogAnswer::ContentsMany(files))", start)
loop = source[start:end]
code = r"""
use std::future::Future;
use std::sync::Arc;
use std::task::{Context, Poll, Wake, Waker};
struct Fake;
impl Fake {
    fn file_name(&self) -> String { "tiny".into() }
    async fn read(&self) -> Vec<u8> { vec![0; 2] }
}
async fn original_picker(picked: Vec<Fake>) -> Vec<(String, Vec<u8>)> {
LOOP
files
}
struct Noop;
impl Wake for Noop { fn wake(self: Arc<Self>) {} }
fn collect(picked: Vec<Fake>) -> Vec<(String, Vec<u8>)> {
    let waker = Waker::from(Arc::new(Noop));
    match std::pin::pin!(original_picker(picked)).as_mut().poll(&mut Context::from_waker(&waker)) {
        Poll::Ready(files) => files,
        Poll::Pending => panic!("fake read unexpectedly pending"),
    }
}
#[test]
fn normal_two_small_files() { assert_eq!(collect(vec![Fake, Fake]).len(), 2); }
#[test]
fn original_picker_has_no_aggregate_admission() {
    let files = collect(vec![Fake, Fake, Fake]);
    let retained: usize = files.iter().map(|(_, bytes)| bytes.len()).sum();
    assert!(retained <= 4, "raw input budget exceeded: retained {retained} bytes for a 4-byte test limit");
}
""".replace("LOOP", loop)
pathlib.Path("/tmp/4980-baseline.rs").write_text(code)
subprocess.run(["rustc", "--edition=2024", "--test", "/tmp/4980-baseline.rs", "-o", "/tmp/4980-baseline"], check=True)
result = subprocess.run(["/tmp/4980-baseline", "--nocapture"], capture_output=True, text=True)
print(result.stdout, result.stderr)
assert result.returncode != 0 and "raw input budget exceeded: retained 6 bytes for a 4-byte test limit" in result.stderr
assert "normal_two_small_files ... ok" in result.stdout
print("Expected assertion failure confirmed on the exact baseline picker loop; no browser exhaustion experiment.")
