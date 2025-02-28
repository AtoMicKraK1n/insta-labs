use std::collections::btree_map::Values;

use anchor_lang::prelude::*;
use crate::state::{patient_data::PatientData, test_result::TestResult};
use crate::error::Errors; 

#[derive(Accounts)]
pub struct StoreTestResults<'info> {
    #[account(
        mut, 
        seeds = [b"patient", patient_data.upid.as_bytes().as_ref()],   
        bump = patient_data.bump,
    )]
    pub patient_data: Account<'info, PatientData>, 
    #[account(mut)]
    pub admin: Signer<'info>, // Only the admin can store test results
    pub system_program: Program<'info, System>,
}


pub fn store_test_results(
    ctx: Context<StoreTestResults>,
    test_id: String,  
    test_type: String,
    path_lab_name: String,
    timestamp: i64,
    haemoglobin: Option<u32>,
    rbc_count: Option<u32>,
    wbc_count: Option<u32>,
    platelet_count: Option<u32>,
    mcv: Option<u32>,
    mch: Option<u32>,
    mchc: Option<u32>,
    rdw: Option<u32>,
    neutrophils: Option<u32>,
    lymphocytes: Option<u32>,
    monocytes: Option<u32>,
    eosinophils: Option<u32>,
    basophils: Option<u32>,  

    ph_level: Option<u32>,
    protein: Option<u32>,
    glucose: Option<u32>,

) -> Result<()> {
    let patient_data = &mut ctx.accounts.patient_data;

    require!(ctx.accounts.admin.key() == patient_data.admin, Errors::UnauthorizedAccess);

    let haemoglobin_scaled = haemoglobin.map(|value| TestResult::scale_up (value as f32));
    let rbc_count_scaled = rbc_count.map(|value| TestResult::scale_up (value as f32));
    let mcv_scaled = mcv.map(|value| TestResult::scale_up (value as f32));
    let mch_scaled  =mch.map(|value| TestResult::scale_up (value as f32));
    let mchc_scaled = mchc.map(|value| TestResult::scale_up (value as f32));
    let rdw_scaled = rdw.map(|value| TestResult::scale_up (value as f32));
    let neutrophils_scaled = neutrophils.map(|value| TestResult::scale_up (value as f32));
    let lymphocytes_scaled = lymphocytes.map(|value| TestResult::scale_up (value as f32));
    let monocytes_scaled = monocytes.map(|value| TestResult::scale_up (value as f32));
    let eosinophils_scaled = eosinophils.map(|value| TestResult::scale_up (value as f32));
    let basophils_scaled = basophils.map(|value| TestResult::scale_up (value as f32));

    let ph_level_scaled = ph_level.map(|value| TestResult::scale_up(value as f32));
    let protein_scaled = protein.map(|value| TestResult::scale_up(value as f32));
    let glucose_scaled = glucose.map(|value| TestResult::scale_up(value as f32));

    const MAX_TESTS: usize = 9;

    if patient_data.tests.len() >= MAX_TESTS {
        patient_data.tests.remove(0); // ✅ Remove the oldest test to make space
    }

    patient_data.tests.push(TestResult {
        test_id,
        test_type,
        path_lab_name,
        timestamp: Clock::get()?.unix_timestamp,
        haemoglobin: haemoglobin_scaled,
        rbc_count: rbc_count_scaled,
        wbc_count,
        platelet_count,
        mcv: mcv_scaled,
        mch: mch_scaled,
        mchc: mchc_scaled,
        rdw: rdw_scaled,
        neutrophils: neutrophils_scaled,
        lymphocytes: lymphocytes_scaled,
        monocytes: monocytes_scaled,
        eosinophils: eosinophils_scaled,
        basophils: basophils_scaled,

        ph_level: ph_level_scaled,
        protein: protein_scaled,
        glucose: glucose_scaled,

    });

    msg!("Blood test result added for UPID: {}", patient_data.upid);
    Ok(())
}
