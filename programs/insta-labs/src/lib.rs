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
        protien: Option<u32>,
        glucose: Option<u32>

    ) -> Result<()> {
        store_test_results::store_test_results(
            ctx, test_id, test_type, path_lab_name, timestamp, haemoglobin, rbc_count, wbc_count, platelet_count, 
            mcv, mch, mchc, rdw, neutrophils, lymphocytes, monocytes, eosinophils, basophils, ph_level, protien, glucose
        )
    }

    // ✅ Fetch Patient Test Results Using UPID
    pub fn get_test_results(ctx: Context<GetTestResults>) -> Result<Vec<TestResult>> {
        get_test_results::get_test_results(ctx)
    }
}


