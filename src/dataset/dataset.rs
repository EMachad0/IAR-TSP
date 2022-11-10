use anyhow::{Context, Result};
use bevy::reflect::TypeUuid;

use crate::dataset::dataset_error::DatasetError;

#[derive(Debug, TypeUuid)]
#[uuid = "b07671ba-d0ce-4ac3-b028-7d05fd890e7a"]
pub struct Dataset {
    pub data: Vec<[f32; 2]>,
}

impl Dataset {
    pub fn len(&self) -> usize {
        self.data.len()
    }
}

pub enum DatasetReadingState {
    HEADER,
    DATA,
    FOOTER,
}

impl Dataset {
    pub fn from_buffer(buffer: &[u8]) -> Result<Dataset> {
        let text = std::str::from_utf8(buffer)?;
        let mut data = Vec::new();
        let mut status = DatasetReadingState::HEADER;
        for (idx, line) in text.lines().enumerate() {
            let idx = idx + 1;

            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            match status {
                DatasetReadingState::HEADER => {
                    if line == "NODE_COORD_SECTION" {
                        status = DatasetReadingState::DATA;
                    }
                }
                DatasetReadingState::DATA => {
                    if line == "EOF" {
                        status = DatasetReadingState::FOOTER;
                        continue;
                    }

                    let mut itr = line.split_whitespace();

                    let mut read_float = || -> Result<f32> {
                        itr.next()
                            .ok_or(DatasetError::MissingArgumentError {
                                idx,
                                text: line.to_string(),
                            })?
                            .replace(',', ".")
                            .parse::<f32>()
                            .with_context(|| {
                                format!("Unable to parse float on line {}: {:?}", idx, line)
                            })
                    };

                    let _ = read_float()?;
                    let x = read_float()?;
                    let y = read_float()?;

                    if itr.next().is_some() {
                        Err(DatasetError::TooManyArgumentsError {
                            idx,
                            text: line.to_string(),
                        })?
                    }

                    data.push([x, y]);
                }
                DatasetReadingState::FOOTER => {}
            }
        }
        match status {
            DatasetReadingState::HEADER => Err(DatasetError::MissingDataEntryTag.into()),
            DatasetReadingState::DATA => Err(DatasetError::MissingDataEndingTag.into()),
            DatasetReadingState::FOOTER => Ok(Self { data }),
        }
    }
}
