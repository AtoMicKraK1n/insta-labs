use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct TestResult {
    pub test_id: String,       // Unique Test ID
    pub test_type: String,     // Type of test (Blood, Urine, Imaging)
    pub timestamp: i64,        // Time of test entry
    pub haemoglobin: Option<f32>,  // Hemoglobin Level (g/dL)
    pub rbc_count: Option<f32>,   // Red Blood Cell Count (million/µL)
    pub wbc_count: Option<f32>,   // White Blood Cell Count (thousands/µL)
    pub platelet_count: Option<f32>, // Platelet Count (per µL)
    pub hematocrit: Option<f32>,  // Hematocrit Level (%)
    pub mcv: Option<f32>,        // Mean Corpuscular Volume (fL)
    pub mch: Option<f32>,        // Mean Corpuscular Hemoglobin (pg)
    pub mchc: Option<f32>,       // Mean Corpuscular Hemoglobin Concentration (g/dL)
    pub rdw: Option<f32>,        // Red Cell Distribution Width (%)
    pub neutrophils: Option<f32>, // Neutrophil Count (%)
    pub lymphocytes: Option<f32>, // Lymphocyte Count (%)
    pub monocytes: Option<f32>,   // Monocyte Count (%)
    pub eosinophils: Option<f32>, // Eosinophil Count (%)
    pub basophils: Option<f32>,   // Basophil Count (%)
    pub additional_notes: Option<String>, // Optional Notes
}
