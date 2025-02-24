import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { InstaLabs } from "../target/types/insta_labs";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";

describe("insta-labs", () => {
  // Configure the client to use the local cluster.

  const provider = anchor.AnchorProvider.local(); // Uses local validator or Devnet
  anchor.setProvider(provider);

  const program = anchor.workspace.InstaLabs as Program<InstaLabs>;

  const admin = provider.wallet;
  const testUPID = "TEST-12345";

  it("Should initialize a new patient record", async () => {
    // 3️⃣ Derive PDA for the patient account
    const [patientPDA] = await PublicKey.findProgramAddress(
      [
        Buffer.from("patient"), 
        Buffer.from(testUPID, "utf8") // ✅ Ensure UTF-8 encoding
      ],
      program.programId
    );;

    console.log(`📌 Patient PDA: ${patientPDA.toBase58()}`);
    // Add your test here.
    const tx = await program.methods.initializePatient(testUPID).accounts({
      patient_data: patientPDA,
      admin: admin.publicKey,
      systemProgram: SystemProgram.programId,
    }).rpc();
    console.log("✅ Transaction Signature", tx);

  });
  it("✅ Should store test results for the patient", async () => {

    const [patientPDA] = await PublicKey.findProgramAddress(
      [
        Buffer.from("patient"), 
        Buffer.from(testUPID, "utf8") // ✅ Ensure UTF-8 encoding
      ],
      program.programId
    );;
    // Define test data (multiple blood parameters)
    const testID = "BLOOD-001";
    const timestamp = new anchor.BN(Date.now());
    const hemoglobin = 13.5;
    const rbcCount = 4.8;
    const wbcCount = 6700;
    const plateletCount = 250000;
    const mcv = 90.0;
    const mch = 30.5;
    const mchc = 34.0;

    // 7️⃣ Send transaction to store test results
    const tx = await program.methods
      .storeTestResults(
        testID,
        timestamp,
        hemoglobin,
        rbcCount,
        wbcCount,
        plateletCount,
        mcv,
        mch,
        mchc
      )
      .accounts({
        patient_data: patientPDA,
        admin: admin.publicKey, 
      })
      .rpc();

    console.log("✅ Test Results Stored - Transaction Signature:", tx);
  });
});
