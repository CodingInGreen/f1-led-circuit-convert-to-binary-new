use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde::ser::SerializeSeq;
use serde::de::{self, Visitor, SeqAccess};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::fmt;
use bincode;

#[derive(Debug, Serialize, Deserialize)]
pub struct DriverData {
    pub driver_number: u8,
    pub led_num: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateFrame {
    pub drivers: [Option<DriverData>; 20],
}

#[derive(Debug)]
pub struct VisualizationData {
    pub update_rate_ms: u32,
    pub frames: [UpdateFrame; 8879],
}

// Custom serialization for VisualizationData
impl Serialize for VisualizationData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(8879))?;
        for frame in &self.frames {
            seq.serialize_element(frame)?;
        }
        seq.end()
    }
}

// Custom deserialization for VisualizationData
impl<'de> Deserialize<'de> for VisualizationData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FrameVisitor;

        impl<'de> Visitor<'de> for FrameVisitor {
            type Value = [UpdateFrame; 8879];

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an array of 8879 UpdateFrame")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut frames: [UpdateFrame; 8879] = unsafe { std::mem::zeroed() };
                for i in 0..8879 {
                    frames[i] = seq.next_element()?
                        .ok_or_else(|| de::Error::invalid_length(i, &self))?;
                }
                Ok(frames)
            }
        }

        let frames = deserializer.deserialize_seq(FrameVisitor)?;
        Ok(VisualizationData {
            update_rate_ms: 250,
            frames,
        })
    }
}

fn main() -> std::io::Result<()> {
    // Read the CSV file
    let file_path = "zandvoort_grouped_1hz.csv";
    let mut rdr = csv::Reader::from_path(file_path)?;

    let headers = rdr.headers()?.clone();
    let mut frames: [UpdateFrame; 8879] = unsafe { std::mem::zeroed() };

    for (i, result) in rdr.records().enumerate() {
        let record = result.unwrap();
        let mut drivers: [Option<DriverData>; 20] = Default::default();

        for (j, field) in record.iter().skip(1).enumerate() {
            let driver_number: u8 = headers[j + 1].parse().unwrap();
            let led_num: u8 = field.parse().unwrap();
            drivers[j] = Some(DriverData { driver_number, led_num });
        }

        frames[i] = UpdateFrame { drivers };
    }

    let visualization_data = VisualizationData {
        update_rate_ms: 250,
        frames,
    };

    // Output JSON format
    let json_file = File::create("output.json")?;
    let writer = BufWriter::new(json_file);
    serde_json::to_writer(writer, &visualization_data)?;

    // Output Binary format
    let bin_file = File::create("output.bin")?;
    let writer = BufWriter::new(bin_file);
    bincode::serialize_into(writer, &visualization_data)
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;

    Ok(())
}
