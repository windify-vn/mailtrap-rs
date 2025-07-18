use crate::endpoints::general::billing::{BillingCycle, BillingSending, BillingTesting};
use crate::framework::response::JsonResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetAccountBillingResponse {
    pub billing: BillingCycle,
    pub testing: BillingTesting,
    pub sending: BillingSending,
}

impl JsonResult for GetAccountBillingResponse {}
