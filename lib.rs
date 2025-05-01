#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, Env, Symbol, String, symbol_short};

#[contracttype]
#[derive(Clone)]
pub struct EmployeeWellness {
    pub name: String,
    pub steps_walked: u64,
    pub meditation_minutes: u64,
    pub is_active: bool,
}

const EMP_WELLNESS: Symbol = symbol_short!("EMP_WELL");

#[contract]
pub struct CorporateWellness;

#[contractimpl]
impl CorporateWellness {
    pub fn register_employee(env: Env, emp_id: u64, name: String) {
        let wellness = EmployeeWellness {
            name,
            steps_walked: 0,
            meditation_minutes: 0,
            is_active: true,
        };
        env.storage().instance().set(&emp_id, &wellness);
    }

    pub fn log_activity(env: Env, emp_id: u64, steps: u64, meditation: u64) {
        let mut record: EmployeeWellness = env.storage().instance().get(&emp_id).expect("Employee not registered");

        if !record.is_active {
            panic!("Employee not active");
        }

        record.steps_walked += steps;
        record.meditation_minutes += meditation;

        env.storage().instance().set(&emp_id, &record);
    }

    pub fn view_progress(env: Env, emp_id: u64) -> EmployeeWellness {
        env.storage().instance().get(&emp_id).expect("Employee not found")
    }
}