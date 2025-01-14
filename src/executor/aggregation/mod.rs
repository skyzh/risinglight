use super::*;
use crate::array::ArrayImpl;
use crate::types::DataValue;

mod count;
mod min_max;
mod rowcount;
mod sum;

pub use count::*;
pub use min_max::*;
pub use rowcount::*;
pub use sum::*;

/// `AggregationState` records the state of an aggregation
pub trait AggregationState: 'static + Send + Sync {
    fn update(&mut self, array: &ArrayImpl) -> Result<(), ExecutorError>;

    fn update_single(&mut self, value: &DataValue) -> Result<(), ExecutorError>;

    fn output(&self) -> DataValue;
}
