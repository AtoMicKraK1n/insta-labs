use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct TestResult {
    pub test_id: String,       // Unique Test ID     // Type of test (Blood, Urine, Imaging)
    pub timestamp: i64,        // Time of test entry
    pub haemoglobin: Option<u32>,  // Hemoglobin Level (g/dL)
    pub rbc_count: Option<u32>,   // Red Blood Cell Count (million/µL)
    pub wbc_count: Option<u32>,   // White Blood Cell Count (thousands/µL)
    pub platelet_count: Option<u32>, // Platelet Count (per µL)
    pub mcv: Option<u32>,        // Mean Corpuscular Volume (fL)
    pub mch: Option<u32>,        // Mean Corpuscular Hemoglobin (pg)
    pub mchc: Option<u32>,   
    pub rdw: Option<u32>,
    pub neutrophils: Option<u32>,
    pub lymphocytes: Option<u32>,
    pub monocytes: Option<u32>,
    pub eosinophils: Option<u32>,
    pub basophils: Option<u32>,   
}

impl TestResult {
    pub fn size() -> usize {
        4 + 32  // test_id (4 bytes for length + max 32 chars)
        + 4 + 32  // test_type (4 bytes for length + max 32 chars)
        + 4 + 4 + 4  // Optional f32 fields (each takes 4 bytes)
    }

    pub fn scale_up(value: f32) -> u32 {
        (value * 10.0) as u32
    } 

    pub fn scale_down(value: u32) -> f32 {
        value as f32 / 10.0
    }
}