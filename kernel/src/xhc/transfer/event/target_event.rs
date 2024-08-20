use core::panic;

use xhci::ring::trb::transfer::{
    Normal,
    SetupStage,
    DataStage,
    StatusStage
};

use crate::xhc::transfer::trb_raw_data::TrbRawData;

#[derive(Debug)]
pub enum TargetEvent {
    Normal(Normal),
    Setup(SetupStage),
    DataStage(DataStage),
    StatusStage(StatusStage),
}

impl TargetEvent {
    pub fn new(target_pointer_addr: u64) -> Option<Self> {
        let raw_data = TrbRawData::from_addr(target_pointer_addr);
        match raw_data.template().trb_type(){
            1 => Some(TargetEvent::Normal(
                Normal::try_from(raw_data.into_u32_array()).ok()?,
            )),
            2 => Some(TargetEvent::Setup(
                SetupStage::try_from(raw_data.into_u32_array()).ok()?,
            )),
            3 => Some(TargetEvent::DataStage(
                DataStage::try_from(raw_data.into_u32_array()).ok()?,
            )),
            4 => Some(TargetEvent::StatusStage(
                StatusStage::try_from(raw_data.into_u32_array()).ok()?,
            )),
            _ => None,
        }
    }

    pub fn data_stage(self) -> DataStage {
        if let TargetEvent::DataStage(data_stage) = self {
            data_stage
        } else {
            panic!("invalid target event")
        }
    }

    pub fn status_stage(self) -> StatusStage {
        if let TargetEvent::StatusStage(status_stage) = self {
            status_stage
        } else {
            panic!("invalid target event")
        }
    }

    pub fn normal(self) -> Normal {
        if let TargetEvent::Normal(normal) = self {
            normal
        } else {
            panic!("invalid target event")
        }
    }
}