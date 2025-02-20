use anchor_lang::prelude::*;
use crate::state::{patient_data::PatientData, test_result::TestResult};
use crate::error::Errors; // Importing state structures

#[derive(Accounts)]
pub struct StoreTestResults<'info> {
    #[account(
        mut, 
        seeds = [b"patient", patient_data.upid.as_bytes().as_ref()], 
        bump
    )]
    pub patient_data: Account<'info, PatientData>, // Patient's on-chain record
    #[account(mut)]
    pub admin: Signer<'info>, // Only the admin can store test results
}

pub fn store_test_results(
    ctx: Context<StoreTestResults>, 
    test_id: String, 
    test_type: String, 
    haemoglobin: Option<f32>,
    rbc_count: Option<f32>,
    wbc_count: Option<f32>,
    platelet_count: Option<f32>,
    hematocrit: Option<f32>,
    mcv: Option<f32>,
    mch: Option<f32>,
    mchc: Option<f32>,
    rdw: Option<f32>,
    neutrophils: Option<f32>,
    lymphocytes: Option<f32>,
    monocytes: Option<f32>,
    eosinophils: Option<f32>,
    basophils: Option<f32>,
    additional_notes: Option<String>
) -> Result<()> {
    let patient_data = &mut ctx.accounts.patient_data;

    require!(ctx.accounts.admin.key() == patient_data.admin, Errors::UnauthorizedAccess);

    patient_data.tests.push(TestResult {
        test_id,
        test_type,
        timestamp: Clock::get()?.unix_timestamp,
        haemoglobin,
        rbc_count,
        wbc_count,
        platelet_count,
        hematocrit,
        mcv,
        mch,
        mchc,
        rdw,
        neutrophils,
        lymphocytes,
        monocytes,
        eosinophils,
        basophils,
        additional_notes,
    });

    msg!("Blood test result added for UPID: {}", patient_data.upid);
    Ok(())
}
