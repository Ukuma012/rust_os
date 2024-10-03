use super::ConfigurationSpace;

#[derive(Debug, Clone)]
pub struct GeneralHeader(ConfigurationSpace);

impl GeneralHeader {
    pub fn new(config_space: ConfigurationSpace) -> Self {
        Self(config_space)
    }

    pub fn msi_capability_pointer(&self) -> u8 {
        (self.0.as_config_space().fetch_data_offset_at(0x34) & 0xFF) as u8
    }
}