pub trait Printer {
    fn print(&self, data: &str) -> Result<String, &str>;

    fn verify_data<'a>(&self, data: &'a str) -> Result<&'a str, &str> {
        if data.is_empty() {
            return Err("Data is empty");
        }
        Ok(data)
    }
}

/// WebPrinter struct
///
/// # Example
///
/// ```
/// use clean_code_notes_exercises::exercises::exercise_06::ocp::*;
///
/// let web_printer = WebPrinter;
/// assert_eq!(web_printer.print("Some data"), Ok("Web Printer Printing: Some data".to_string()));
/// assert_eq!(web_printer.print(""), Err("Data is empty"));
/// ```
pub struct WebPrinter;

impl Printer for WebPrinter {
    fn print(&self, data: &str) -> Result<String, &str> {
        let valid_data = self.verify_data(data)?;
        Ok(format!("Web Printer Printing: {}", valid_data))
    }
}

/// PDFPrinter struct
///
/// # Example
///
/// ```
/// use clean_code_notes_exercises::exercises::exercise_06::ocp::*;
///
/// let pdf_printer = PDFPrinter;
/// assert_eq!(pdf_printer.print("Some data"), Ok("PDF Printer Printing: Some data".to_string()));
/// assert_eq!(pdf_printer.print(""), Err("Data is empty"));
/// ```
pub struct PDFPrinter;

impl Printer for PDFPrinter {
    fn print(&self, data: &str) -> Result<String, &str> {
        let valid_data = self.verify_data(data)?;
        Ok(format!("PDF Printer Printing: {}", valid_data))
    }
}

/// PagePrinter struct
///
/// # Example
///
/// ```
/// use clean_code_notes_exercises::exercises::exercise_06::ocp::*;
///
/// let page_printer = PagePrinter;
/// assert_eq!(page_printer.print("Some data"), Ok("Page Printer Printing: Some data".to_string()));
/// assert_eq!(page_printer.print(""), Err("Data is empty"));
/// ```
pub struct PagePrinter;

impl Printer for PagePrinter {
    fn print(&self, data: &str) -> Result<String, &str> {
        let valid_data = self.verify_data(data)?;
        Ok(format!("Page Printer Printing: {}", valid_data))
    }
}
