 # Insta-Labs: Technical Specification

- [LOI](https://docs.google.com/document/d/e/2PACX-1vQoLpTkVrhBBIQ5wSSYvwFE3i54lxmwqGUY152jTlBhaRbKtTdJCA_nf9ChVTQJara0VN05aO2f3KjF/pub)
- [User Stories](https://docs.google.com/document/d/e/2PACX-1vQOkCD06oXVEj9tZBNuYGgMJR_s1LWunMcv1_LxLl9v1B6fTmJXeJOH0y0l6HadNb9cUovro4hxq_Q9/pub)
- [Arch Diagrams](https://drive.google.com/file/d/1mw5i_7jyAtiSQZSaxGb8mNJajiVW1A8e/view?usp=sharing) 

---

## **Project Structure**
The contract follows a **modular design** with separate files for **state management** and **business logic (instructions/contexts)**.

### **1️. Account Structures**
These define **data storage** on Solana.

- **`patient_data.rs`** – Stores patient details and **lab test history** in a PDA.  
- **`test_result.rs`** – Defines the structure for **individual test records**, including Blood, Urine, and Imaging tests.  

---

### **2️. Instructions**
These handle **transactions and interactions** with the smart contract.

- **`initialize_patient.rs`** – Creates a **new patient account (PDA)** using a **Unique Patient ID (UPID)**.  
- **`store_test_results.rs`** – Allows **authorized labs** to store patient test results **on-chain**.  
- **`get_test_results.rs`** – Fetches a **patient’s medical history** securely.  

---

### **Account Structures**

```rust
pub struct PatientData {
    pub upid: String,
    pub bump: u8,           
    pub admin: Pubkey,            
    pub tests: Vec<TestResult>,   
}

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
```

## Future Roadmap
 Phase 2 Features
   - Adding more type of tests
   - Making ULID (Unique Lab ID functional)
   - Comparing test rates across the cities possible
