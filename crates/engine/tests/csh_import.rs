//! Original geometric CSH fixture, assembled from the public container/path layout.
//! No Adobe or third-party shape assets are embedded.

use photocraft_engine::Session;
use serde_json::json;

fn triangle_csh() -> Vec<u8> {
    let mut out = b"cush".to_vec();
    out.extend_from_slice(&2u32.to_be_bytes());
    out.extend_from_slice(&1u32.to_be_bytes());
    let name: Vec<u16> = "Triangle".encode_utf16().chain(std::iter::once(0)).collect();
    out.extend_from_slice(&(name.len() as u32).to_be_bytes());
    for unit in &name {
        out.extend_from_slice(&unit.to_be_bytes());
    }
    if name.len() % 2 != 0 {
        out.extend_from_slice(&[0, 0]);
    }
    out.extend_from_slice(&1u32.to_be_bytes());
    let mut body = vec![36];
    body.extend_from_slice(b"00000000-0000-4000-8000-000000000001");
    for bound in [0i32, 0, 100, 200] {
        body.extend_from_slice(&bound.to_be_bytes());
    }
    for selector in [6u16, 8] {
        body.extend_from_slice(&selector.to_be_bytes());
        body.extend_from_slice(&[0; 24]);
    }
    body.extend_from_slice(&0u16.to_be_bytes());
    body.extend_from_slice(&3u16.to_be_bytes());
    body.extend_from_slice(&[0; 22]);
    for (x, y) in [(0.25f64, 0.25f64), (0.75, 0.25), (0.5, 0.75)] {
        body.extend_from_slice(&2u16.to_be_bytes());
        for _ in 0..3 {
            body.extend_from_slice(&((y * 16_777_216.0) as i32).to_be_bytes());
            body.extend_from_slice(&((x * 16_777_216.0) as i32).to_be_bytes());
        }
    }
    while body.len() % 4 != 0 {
        body.push(0);
    }
    out.extend_from_slice(&(body.len() as u32).to_be_bytes());
    out.extend_from_slice(&body);
    out
}

#[test]
fn csh_import_makes_an_original_shape_available_without_a_document() {
    let bytes = triangle_csh();
    if let Some(path) = std::env::var_os("PHOTOCRAFT_CSH_FIXTURE") {
        std::fs::write(path, &bytes).unwrap();
    }
    let mut session = Session::new();
    session
        .execute("shape.presets.importCsh", json!({"data": photocraft_paint::tile::b64_encode(&bytes), "group": "Original CSH"}))
        .expect("CSH import must be available without an open document");
    let groups = photocraft_engine::presets::shapes::all_groups(&session);
    let imported = groups.iter().find(|g| g.name == "Original CSH").expect("imported group");
    assert_eq!(imported.items.len(), 1);
    assert_eq!(imported.items[0].name, "Triangle");
    let path = &imported.items[0].path;
    assert_eq!(path.subpaths.len(), 1);
    assert!(path.subpaths[0].closed);
    assert_eq!(path.subpaths[0].knots.len(), 3);
}
