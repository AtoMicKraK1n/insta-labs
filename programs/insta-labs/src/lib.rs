use anchor_lang::prelude::*;

declare_id!("9tNAK3cjM1HKEd2ijZZpwF4LN84Kbq1eKR3LhvRC8HD5");

pub mod contexts;
pub mod state;
pub mod error;

pub use contexts::*;
pub use state::*;
pub use error::*;

#[program]
pub mod insta_labs {
    use super::*;

    // ✅ Initialize a Patient Record (Admin Only)
    pub fn initialize_patient(ctx: Context<InitializePatient>, upid: String) -> Result<()> {
        initialize_patient::initialize_patient(ctx, upid)
    }

    // ✅ Store Lab Test Results (Admin Only)
    pub fn store_test_results(
        ctx: Context<StoreTestResults>, 
        test_id: String, 
        timestamp: i64,
        haemoglobin: Option<f32>,
        rbc_count: Option<f32>,
        wbc_count: Option<f32>,
        platelet_count: Option<f32>,
        mcv: Option<f32>,
        mch: Option<f32>,
        mchc: Option<f32>,

    ) -> Result<()> {
        store_test_results::store_test_results(
            ctx, test_id, timestamp, haemoglobin, rbc_count, wbc_count, platelet_count, 
            mcv, mch, mchc
        )
    }

    // ✅ Fetch Patient Test Results Using UPID
    pub fn get_test_results(ctx: Context<GetTestResults>) -> Result<Vec<TestResult>> {
        get_test_results::get_test_results(ctx)
    }
}


