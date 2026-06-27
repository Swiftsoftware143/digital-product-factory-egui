//! Export module for products

pub struct Exporter;

impl Exporter {
    pub fn new() -> Self {
        Self
    }
    
    pub fn export_zip(&self, product_id: usize, output_path: &str) -> Result<(), String> {
        // TODO: Implement ZIP export
        Ok(())
    }
    
    pub fn export_folder(&self, product_id: usize, output_path: &str) -> Result<(), String> {
        // TODO: Implement folder export
        Ok(())
    }
}
