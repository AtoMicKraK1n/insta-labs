use anchor_lang::prelude::*;
use crate::state::{patient_data::PatientData, test_result::TestResult}; // Import patient data structures

#[derive(Accounts)]
pub struct GetTestResults<'info> {
    #[account(
        seeds = [b"patient", patient_data.upid.as_bytes().as_ref()],
        bump
    )]
    pub patient_data: Account<'info, PatientData>, // On-chain storage for patient test records
}

pub fn get_test_results(ctx: Context<GetTestResults>) -> Result<Vec<TestResult>> {
    let patient_data = &ctx.accounts.patient_data;

    let mut formatted_tests = vec![];
    for test in &patient_data.tests {
        formatted_tests.push(TestResult {
            test_id: test.test_id.clone(),
            test_type: test.test_type.clone(),
            timestamp: test.timestamp.clone(),
            path_lab_name: test.path_lab_name.clone(),
            haemoglobin: test.haemoglobin.map(TestResult::scale_down).map(|value| value as u32),
            rbc_count: test.rbc_count.map(TestResult::scale_down).map(|value| value as u32),
            wbc_count: test.wbc_count.clone(),
            platelet_count: test.platelet_count.clone(),
            mcv: test.mcv.map(TestResult::scale_down).map(|value| value as u32),
            mch: test.mch.map(TestResult::scale_down).map(|value| value as u32),
            mchc: test.mchc.map(TestResult::scale_down).map(|value| value as u32),
            rdw: test.rdw.map(TestResult::scale_down).map(|value| value as u32),
            neutrophils: test.neutrophils.map(TestResult::scale_down).map(|value| value as u32),
            lymphocytes: test.lymphocytes.map(TestResult::scale_down).map(|value| value as u32),
            monocytes: test.monocytes.map(TestResult::scale_down).map(|value| value as u32),
            eosinophils: test.eosinophils.map(TestResult::scale_down).map(|value| value as u32),
            basophils: test.basophils.map(TestResult::scale_down).map(|value| value as u32),

            ph_level: test.ph_level.map(TestResult::scale_down).map(|value| value as u32),
            protein: test.protein.map(TestResult::scale_down).map(|value| value as u32),
            glucose: test.glucose.map(TestResult::scale_down).map(|value| value as u32),
        });
    }

    msg!("Fetching test results for UPID: {}", formatted_tests.len());

    // Return all test results stored in the patient's account
    Ok(formatted_tests)
}
