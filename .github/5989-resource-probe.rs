// Verification-only probe; inject inside liquify_ui::tests in a cloud checkout.
// Run the compiled release test binary under /usr/bin/time -v, excluding compilation.
// Compare the same probe on baseline6368e6a8 and candidate; three samples per mode.
#[test]
#[ignore = "cloud-only 24MP resource measurement"]
fn liquify_selection_resource_probe_5989() {
    use std::time::Instant;
    let mode = std::env::var("LIQUIFY_PROBE_MODE").unwrap();
    let mut app = PhotocraftApp::new(photocraft_engine::Session::new(), Default::default());
    app.session.execute("file.new", json!({"width":6000,"height":4000,"depth":8})).unwrap();
    app.session.execute("layer.new.layer", json!({})).unwrap();
    app.session.edit("synthetic resource fixture", |doc, active| {
        doc.layer_mut(active.unwrap()).unwrap().surface_mut().unwrap()
            .fill_rect(Rect::new(0,0,6000,4000), &[0.4,0.6,0.8,0.5]);
        Ok(())
    }).unwrap();
    if mode != "unselected" {
        app.session.execute("select.rect", json!({"x":0,"y":0,"width":3000,"height":4000})).unwrap();
    }
    app.sync_views();
    let ctx = egui::Context::default();
    let started = Instant::now();
    open(&mut app, &ctx).unwrap();
    let open_ms = started.elapsed().as_secs_f64() * 1000.0;
    let d = app.distort.liquify.as_mut().unwrap();
    let area = ERect::from_min_size(pos2(0.0,0.0),vec2(1600.0,1200.0));
    let started = Instant::now();
    if mode == "zoomed" {
        d.zoom = 2.0;
        d.center = [3000.0,2000.0];
        d.update_view_preview(&ctx,area);
        assert!(d.view_tex.is_some());
    }
    let crop_ms = started.elapsed().as_secs_f64() * 1000.0;
    let started = Instant::now();
    for step in 0..20 {
        let x = 2900.0 + f64::from(step) * 5.0;
        let mut stroke = LiquifyStroke::new(LiquifyTool::ForwardWarp,100.0);
        stroke.points = vec![vec![x,2000.0,1.0],vec![x+5.0,2000.0,1.0]];
        let dirty = d.field.stroke_segment(&stroke,[x,2000.0,1.0],[x+5.0,2000.0,1.0]);
        d.mark(dirty,false);
        d.render_dirty();
        if mode == "zoomed" { d.update_view_preview(&ctx,area); }
    }
    println!("RESOURCE mode={mode} pixels=24000000 open_ms={open_ms:.3} crop_ms={crop_ms:.3} twenty_dabs_ms={:.3}",started.elapsed().as_secs_f64()*1000.0);
}
