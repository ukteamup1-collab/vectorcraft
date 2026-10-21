use super::load;
use serde_json::json;
use tracedraw_core::document::{Shape, ShapeKind};
use tracedraw_core::geometry::{Rect, Size};
use tracedraw_core::{Document, ShapeId};

#[test]
fn cdr_x7_opens_as_editable_art_at_the_physical_label_size() {
    let mut source = Document::new("Label", Size::new(282.0, 105.0));
    source.pages[0].layers[0]
        .shapes
        .push(Shape::new(ShapeId(100), ShapeKind::Rect { rect: Rect::new(10.0, 20.0, 45.0, 60.0), radius: 0.0, corners: None }));
    let bytes = tracedraw_cdr::write::document_to_cdr_version(&source, 17);
    let loaded = load::load("Label.cdr", &bytes).unwrap();
    assert_eq!(loaded.format.id, "cdr");
    assert!(!loaded.format.write);
    assert!(loaded.doc.node_count() >= 2);
    let page = &loaded.doc.artboards[0];
    assert!((page.rect.width() - 282.0 * 72.0 / 25.4).abs() < 0.01);
    assert!((page.rect.height() - 105.0 * 72.0 / 25.4).abs() < 0.01);
    assert!(loaded.warnings.iter().any(|s| s.contains("Experimental CDR")));
}

#[test]
fn cdr_bad_input_does_not_replace_the_existing_document() {
    let mut session = crate::Session::new();
    session.execute("file.new", &json!({"width": 200, "height": 100})).unwrap();
    let before = session.doc().unwrap().doc.clone();
    let result = session.execute("document.open", &json!({"name": "broken.cdr", "dataBase64": "YnJva2Vu"}));
    assert!(result.is_err());
    assert_eq!(session.doc().unwrap().doc.artboards, before.artboards);
    assert_eq!(session.doc().unwrap().doc.node_count(), before.node_count());
}
