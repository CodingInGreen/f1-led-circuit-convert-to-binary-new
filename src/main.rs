use csv::ReaderBuilder;
use postcard;
use serde::{Deserialize, Serialize};
use serde_json;
use std::error::Error;
use std::fs::File;
use std::io::Write;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
struct DriverData {
    driver_number: u8,
    led_num: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateFrame {
    pub drivers: [Option<DriverData>; 20],
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VisualizationData {
    pub update_rate_ms: u32,
    pub frames: Vec<UpdateFrame>,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Open the CSV file
    let file_path = "zandvoort_grouped_1hz.csv";
    let mut rdr = ReaderBuilder::new().has_headers(true).from_path(file_path)?;

    // Initialize the frames
    let mut frames: Vec<UpdateFrame> = Vec::new();

    // Iterate through CSV records
    for result in rdr.records() {
        let record = result?;
        let mut drivers: [Option<DriverData>; 20] = Default::default();

        for (i, led_num) in record.iter().skip(1).enumerate() {
            if i < 20 {
                if let Ok(led_num) = led_num.parse::<u8>() {
                    drivers[i] = Some(DriverData {
                        driver_number: (i + 1) as u8,
                        led_num,
                    });
                }
            }
        }

        frames.push(UpdateFrame { drivers });
    }

    // Check if the number of frames matches the expected number
    if frames.len() != 8879 {
        return Err(Box::from(format!(
            "The number of frames ({}) does not match the expected 8879.",
            frames.len()
        )));
    }

    // Create VisualizationData
    let data = VisualizationData {
        update_rate_ms: 1000,
        frames,
    };

    // Output JSON file
    let json_file_path = "output.json";
    let json_file = File::create(json_file_path)?;
    serde_json::to_writer(json_file, &data)?;

    // Output Binary file
    let bin_file_path = "output.bin";
    let mut bin_file = File::create(bin_file_path)?;
    let serialized_data = postcard::to_allocvec(&data)?;
    bin_file.write_all(&serialized_data)?;

    Ok(())
}
