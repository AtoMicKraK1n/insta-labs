use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct TestResult {
    pub test_id: String,
    pub test_type: String,
    pub path_lab_name: String, 
    pub timestamp: i64,  

    // Blood Test Fields
    pub haemoglobin: Option<u32>,  
    pub rbc_count: Option<u32>,   
    pub wbc_count: Option<u32>,   
    pub platelet_count: Option<u32>, 
    pub mcv: Option<u32>,        
    pub mch: Option<u32>,        
    pub mchc: Option<u32>,   
    pub rdw: Option<u32>,
    pub neutrophils: Option<u32>,
    pub lymphocytes: Option<u32>,
    pub monocytes: Option<u32>,
    pub eosinophils: Option<u32>,
    pub basophils: Option<u32>,  

    // Urine Test Fields 
    pub ph_level: Option<u32>,
    pub protein: Option<u32>,
    pub glucose: Option<u32>,

}

impl TestResult {
    pub fn size() -> usize {
        4 + 32  
        + 4 + 32  
        + 4 + 4 + 4  
        + 4 + 4 + 4
    }

    pub fn scale_up(value: f32) -> u32 {
        (value * 10.0) as u32
    } 

    pub fn scale_down(value: u32) -> f32 {
        value as f32 / 10.0
    }
}