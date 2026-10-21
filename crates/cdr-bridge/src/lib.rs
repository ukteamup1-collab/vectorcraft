//! Experimental CDR import through an attributed, pinned open-source parser.
//! No application executable, proprietary resource or decompiled code is used.
#![forbid(unsafe_code)]

/// One page as editable SVG, with import limitations delivered to the caller.
pub struct Page {
    pub svg: String,
    pub source_pages: usize,
    pub width_mm: f64,
    pub height_mm: f64,
    pub warnings: Vec<String>,
}

/// Input limit in addition to the parser's own decompression limits.
pub const MAX_BYTES: usize = 128 * 1024 * 1024;

/// Read a requested zero-based page. Never silently substitute another page.
pub fn read_page(bytes: &[u8], title: &str, page: usize) -> Result<Page, String> {
    if bytes.len() > MAX_BYTES {
        return Err("CDR input exceeds the 128 MiB import limit".into());
    }
    let (doc, report) = tracedraw_cdr::open_bytes(bytes, title).map_err(|e| format!("CDR could not be read: {e}"))?;
    let source_page = doc.pages.get(page).ok_or_else(|| format!("CDR page {} does not exist ({} pages)", page.saturating_add(1), doc.pages.len()))?;
    let mut warnings = report.warnings;
    warnings.push("Experimental CDR import converts one page through SVG. Effects, colour profiles, text layout and clipping may change; inspect the artwork before printing. Save an editable working copy in the native project format. CDR saving is not available in this build.".into());
    if doc.pages.len() > 1 {
        warnings.push(format!("Opened page {} of {}. Other pages are not included in this document.", page.saturating_add(1), doc.pages.len()));
    }
    if report.skipped_objects > 0 {
        warnings.push(format!("The CDR reader skipped {} unsupported objects.", report.skipped_objects));
    }
    Ok(Page {
        svg: tracedraw_io::svg::page_to_svg(&doc, page),
        source_pages: doc.pages.len(),
        width_mm: source_page.size.width,
        height_mm: source_page.size.height,
        warnings,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracedraw_core::document::{Shape, ShapeKind};
    use tracedraw_core::geometry::{Rect, Size};
    use tracedraw_core::{Document, ShapeId};

    fn sample(version: u16) -> Vec<u8> {
        let mut d = Document::new("Synthetic label", Size::new(282.0, 105.0));
        d.pages[0].layers[0]
            .shapes
            .push(Shape::new(ShapeId(100), ShapeKind::Rect { rect: Rect::new(10.0, 20.0, 45.0, 60.0), radius: 0.0, corners: None }));
        tracedraw_cdr::write::document_to_cdr_version(&d, version)
    }

    #[test]
    fn x7_container_preserves_page_size_and_vector_geometry() {
        let p = read_page(&sample(17), "Label", 0).unwrap();
        assert_eq!(p.source_pages, 1);
        assert!((p.width_mm - 282.0).abs() < 0.001, "{}", p.width_mm);
        assert!((p.height_mm - 105.0).abs() < 0.001, "{}", p.height_mm);
        assert!(p.svg.contains("<path") || p.svg.contains("<rect"));
        assert!(p.warnings.iter().any(|w| w.contains("Experimental")));
    }

    #[test]
    fn older_riff_container_is_also_readable() {
        assert!(read_page(&sample(13), "Label", 0).is_ok());
    }

    #[test]
    fn malformed_input_and_missing_pages_are_errors() {
        for bytes in [&b""[..], &b"not a drawing"[..], &b"RIFF"[..]] {
            assert!(read_page(bytes, "Broken", 0).is_err());
        }
        assert!(read_page(&sample(17), "Label", 1).is_err());
    }
}
