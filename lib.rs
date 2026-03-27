#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PaymentRecord {
    pub record_id: u32,
    pub employer: Address,
    pub worker: Address,
    pub usdc_amount: i128,
    pub work_minted: i128,
    pub job_type: String,
    pub timestamp: u64,
    pub clawback_exp: u64,
    pub clawed_back: bool,
}

#[contract]
pub struct PaySlipContract;

const RECORD_COUNT: &str = "REC_CNT";

#[contractimpl]
impl PaySlipContract {
    pub fn pay(
        env: Env,
        employer: Address,
        worker: Address,
        usdc_amount: i128,
        job_type: String,
    ) -> u32 {
        employer.require_auth();

        let mut count: u32 = env.storage().instance().get(&RECORD_COUNT).unwrap_or(0);
        count += 1;

        let timestamp = env.ledger().timestamp();
        let clawback_exp = timestamp + 86400; // 24 hours

        let record = PaymentRecord {
            record_id: count,
            employer: employer.clone(),
            worker: worker.clone(),
            usdc_amount,
            work_minted: usdc_amount,
            job_type,
            timestamp,
            clawback_exp,
            clawed_back: false,
        };

        env.storage().persistent().set(&count, &record);
        env.storage().instance().set(&RECORD_COUNT, &count);

        count
    }

    pub fn get_record(env: Env, record_id: u32) -> PaymentRecord {
        env.storage().persistent().get(&record_id).expect("Record not found")
    }

    pub fn clawback(env: Env, employer: Address, record_id: u32) {
        employer.require_auth();
        
        let mut record: PaymentRecord = env.storage().persistent().get(&record_id).expect("Record not found");
        assert!(record.employer == employer, "Only the employer can clawback");
        assert!(!record.clawed_back, "Already clawed back");
        assert!(env.ledger().timestamp() <= record.clawback_exp, "Clawback period expired");

        record.clawed_back = true;
        
        env.storage().persistent().set(&record_id, &record);
    }
}
