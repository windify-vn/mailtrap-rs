use crate::endpoints::general::billing::{BillingCycle, BillingSending, BillingTesting};
use crate::framework::response::ApiResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetAccountBillingResponse {
    pub billing: BillingCycle,
    pub testing: BillingTesting,
    pub sending: BillingSending,
}

impl ApiResult for GetAccountBillingResponse {}
