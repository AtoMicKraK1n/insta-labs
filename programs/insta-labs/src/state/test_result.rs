use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct TestResult {
    pub test_id: String,       // Unique Test ID     // Type of test (Blood, Urine, Imaging)
    pub timestamp: i64,        // Time of test entry
    pub haemoglobin: Option<f32>,  // Hemoglobin Level (g/dL)
    pub rbc_count: Option<f32>,   // Red Blood Cell Count (million/µL)
    pub wbc_count: Option<f32>,   // White Blood Cell Count (thousands/µL)
    pub platelet_count: Option<f32>, // Platelet Count (per µL)
    pub mcv: Option<f32>,        // Mean Corpuscular Volume (fL)
    pub mch: Option<f32>,        // Mean Corpuscular Hemoglobin (pg)
    pub mchc: Option<f32>,       // Mean Corpuscular Hemoglobin Concentration (g/dL)
}
