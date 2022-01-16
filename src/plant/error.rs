use thiserror::Error;
#[derive(Error, Debug)]
pub enum PlantError {
    #[error("entity already exists")]
    EntityAlreadyExists
}
