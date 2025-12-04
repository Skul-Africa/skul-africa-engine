use anyhow::Result;
use std::io::Read;
use zip::ZipArchive;
use quick_xml::Reader;
use quick_xml::events::Event;

pub type Row = Vec<String>;

pub fn parse_docx_table(path: &str) -> Result<Vec<Row>> {
    let file = std::fs::File::open(path)?;
    let mut zip = ZipArchive::new(file)?;

    // Extract the main Word document XML
    let mut document_xml = zip.by_name("word/document.xml")?;
    let mut xml = String::new();
    document_xml.read_to_string(&mut xml)?;

    let mut reader = Reader::from_str(&xml);
    reader.trim_text(true);

    let mut buf = Vec::new();
    let mut rows = Vec::new();
    let mut current_row = Vec::new();

    let mut in_cell = false;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                if e.name().as_ref() == b"w:tc" {
                    in_cell = true;
                }
            }

            Ok(Event::Text(e)) => {
                if in_cell {
                    let text = e.unescape()?.to_string();
                    current_row.push(text);
                }
            }

            Ok(Event::End(e)) => {
                match e.name().as_ref() {
                    b"w:tc" => {
                        in_cell = false;
                    }
                    b"w:tr" => {
                        if !current_row.is_empty() {
                            rows.push(current_row.clone());
                            current_row.clear();
                        }
                    }
                    _ => {}
                }
            }

            Ok(Event::Eof) => break,
            Err(e) => return Err(e.into()),
            _ => {}
        }
    }

    Ok(rows)
}
