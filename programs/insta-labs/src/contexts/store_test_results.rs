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
    timestamp: i64,
    haemoglobin: Option<u32>,
    rbc_count: Option<u32>,
    wbc_count: Option<u32>,
    platelet_count: Option<u32>,
    mcv: Option<u32>,
    mch: Option<u32>,
    mchc: Option<u32>,

) -> Result<()> {
    let patient_data = &mut ctx.accounts.patient_data;

    require!(ctx.accounts.admin.key() == patient_data.admin, Errors::UnauthorizedAccess);

    let haemoglobin_scaled = haemoglobin.map(|value| TestResult::scale_up (value as f32));
    let rbc_count_scaled = rbc_count.map(|value| TestResult::scale_up (value as f32));
    let mcv_scaled = mcv.map(|value| TestResult::scale_up (value as f32));
    let mch_scaled  =mch.map(|value| TestResult::scale_up (value as f32));
    let mchc_scaled = mchc.map(|value| TestResult::scale_up (value as f32));

    const MAX_TESTS: usize = 9;

    if patient_data.tests.len() >= MAX_TESTS {
        patient_data.tests.remove(0); // ✅ Remove the oldest test to make space
    }

    patient_data.tests.push(TestResult {
        test_id,
        timestamp: Clock::get()?.unix_timestamp,
        haemoglobin: haemoglobin_scaled,
        rbc_count: rbc_count_scaled,
        wbc_count,
        platelet_count,
        mcv: mcv_scaled,
        mch: mch_scaled,
        mchc: mchc_scaled,

    });

    msg!("Blood test result added for UPID: {}", patient_data.upid);
    Ok(())
}
