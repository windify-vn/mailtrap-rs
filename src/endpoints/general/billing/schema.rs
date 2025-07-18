use alloc::string::String;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BillingMessagesCount {
    pub current: u64,
    pub limit: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BillingPlan {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BillingSendingUsage {
    pub sent_messages_count: BillingMessagesCount,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BillingSending {
    pub plan: BillingPlan,
    pub usage: BillingSendingUsage,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BillingTestingUsage {
    pub sent_messages_count: BillingMessagesCount,
    pub forwarded_messages_count: BillingMessagesCount,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BillingTesting {
    pub plan: BillingPlan,
    pub usage: BillingTestingUsage,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BillingCycle {
    pub cycle_start: chrono::DateTime<chrono::Utc>,
    pub cycle_end: chrono::DateTime<chrono::Utc>,
}
