//! `w:pgMar` pins the two bands: `w:header` is the distance from the top edge
//! of the page to the top of the header story, `w:footer` the distance from the
//! bottom edge to the bottom of the footer story.

use docx_edit::{EngineSession, seed_from_docx};
use serde_json::{Value, json};

const FONT: &[u8] = include_bytes!("../../ooxml-text/tests/fonts/LiberationSans-Regular.ttf");

/// 24pt exact, so the line height is 32px whatever the font measures.
const EXACT_LINE: &str = r#"<w:spacing w:before="0" w:after="0" w:line="480" w:lineRule="exact"/>"#;

fn para(text: &str) -> String {
    format!(
        r#"<w:p><w:pPr>{EXACT_LINE}</w:pPr><w:r><w:rPr><w:rFonts w:ascii="Arial" w:hAnsi="Arial"/><w:sz w:val="24"/></w:rPr><w:t>{text}</w:t></w:r></w:p>"#
    )
}

/// Letter page (816x1056px) with the requested header/footer distances.
fn document(header: &str, footer: &str, header_twips: u32, footer_twips: u32) -> Vec<u8> {
    let body = para("BODY");
    let sect = format!(
        r#"<w:sectPr><w:headerReference w:type="default" r:id="rIdHeader"/><w:footerReference w:type="default" r:id="rIdFooter"/><w:pgSz w:w="12240" w:h="15840"/><w:pgMar w:top="1440" w:right="1440" w:bottom="1440" w:left="1440" w:header="{header_twips}" w:footer="{footer_twips}"/></w:sectPr>"#
    );
    let parts: Vec<(String, Vec<u8>)> = vec![
        (
            "[Content_Types].xml".to_owned(),
            r#"<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"><Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/><Default Extension="xml" ContentType="application/xml"/><Override PartName="/word/document.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"/><Override PartName="/word/styles.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.styles+xml"/><Override PartName="/word/header1.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.header+xml"/><Override PartName="/word/footer1.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.footer+xml"/></Types>"#.to_owned(),
        ),
        (
            "_rels/.rels".to_owned(),
            r#"<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="doc" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="word/document.xml"/></Relationships>"#.to_owned(),
        ),
        (
            "word/_rels/document.xml.rels".to_owned(),
            r#"<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="styles" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles" Target="styles.xml"/><Relationship Id="rIdHeader" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/header" Target="header1.xml"/><Relationship Id="rIdFooter" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/footer" Target="footer1.xml"/></Relationships>"#.to_owned(),
        ),
        (
            "word/document.xml".to_owned(),
            format!(
                r#"<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"><w:body>{body}{sect}</w:body></w:document>"#
            ),
        ),
        (
            "word/styles.xml".to_owned(),
            r#"<w:styles xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:docDefaults><w:rPrDefault><w:rPr><w:rFonts w:ascii="Arial" w:hAnsi="Arial"/><w:sz w:val="24"/></w:rPr></w:rPrDefault></w:docDefaults><w:style w:type="paragraph" w:default="1" w:styleId="Normal"><w:name w:val="Normal"/></w:style></w:styles>"#.to_owned(),
        ),
        (
            "word/header1.xml".to_owned(),
            format!(
                r#"<w:hdr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">{header}</w:hdr>"#
            ),
        ),
        (
            "word/footer1.xml".to_owned(),
            format!(
                r#"<w:ftr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">{footer}</w:ftr>"#
            ),
        ),
    ]
    .into_iter()
    .map(|(name, value)| (name, value.into_bytes()))
    .collect();
    ooxml_opc::rezip_parts(&parts).unwrap()
}

fn first_page(bytes: &[u8]) -> Value {
    let font = docx_layout::register_measure_font(FONT).unwrap();
    let package = docx_parse::parse_docx_s9_wire(bytes, Default::default())
        .unwrap()
        .document
        .package;
    let engine = EngineSession::new(76501);
    seed_from_docx(engine.doc(), bytes).unwrap();
    let request = json!({
        "bodyStory": "body", "renderEnv": {},
        "regions": {"sections": [{"properties": package.document.final_section_properties}], "settings": package.settings},
        "measurement": {"fontChains": {"arial|0|0": [font]}, "defaults": {"fontFamily": "Arial", "fontSize": 12}}
    })
    .to_string();
    let layout = engine.layout_document_with_regions_json(&request).unwrap();
    let display: Value =
        serde_json::from_str(&engine.build_display_list_json(&layout).unwrap()).unwrap();
    display["pages"][0].clone()
}

fn number(value: &Value) -> f64 {
    value.as_f64().unwrap()
}

#[test]
fn header_band_starts_at_the_header_distance() {
    // w:header="720" -> 48px from the top edge, whatever the header holds.
    for header in [para("H"), format!("{}{}", para("H"), para("H2"))] {
        let lines = header.matches("<w:p>").count() as f64;
        let page = first_page(&document(&header, &para("F"), 720, 1440));

        assert_eq!(number(&page["header"]["y"]), 48.0);
        assert_eq!(number(&page["header"]["height"]), 32.0 * lines);
    }
}

#[test]
fn footer_band_ends_at_the_footer_distance() {
    // w:footer="1440" -> the story bottom sits 96px above the page bottom, so
    // its top rides up as the story grows.
    for footer in [para("F"), format!("{}{}", para("F"), para("F2"))] {
        let lines = footer.matches("<w:p>").count() as f64;
        let page = first_page(&document(&para("H"), &footer, 720, 1440));

        assert_eq!(number(&page["footer"]["height"]), 32.0 * lines);
        assert_eq!(
            number(&page["footer"]["y"]) + number(&page["footer"]["height"]),
            1056.0 - 96.0
        );
        assert_eq!(number(&page["footer"]["y"]), 1056.0 - 96.0 - 32.0 * lines);
    }
}

#[test]
fn footer_distance_moves_the_band_and_the_header_stays_put() {
    let near = first_page(&document(&para("H"), &para("F"), 720, 397));
    let far = first_page(&document(&para("H"), &para("F"), 720, 1440));

    // 397tw = 26.4667px, 1440tw = 96px.
    // Display-list coordinates round to 3 decimals.
    assert!((number(&near["footer"]["y"]) - (1056.0 - 397.0 / 15.0 - 32.0)).abs() < 1e-3);
    assert_eq!(number(&far["footer"]["y"]), 1056.0 - 96.0 - 32.0);
    assert_eq!(number(&near["header"]["y"]), number(&far["header"]["y"]));
}
