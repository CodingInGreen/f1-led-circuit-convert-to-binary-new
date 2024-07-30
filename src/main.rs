use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde::ser::SerializeSeq;
use serde::de::{self, Visitor, SeqAccess};
use std::fs::File;
use std::io::{BufWriter};
use std::fmt;
use bincode;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct DriverData {
    pub driver_number: u8,
    pub led_num: u8,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct UpdateFrame {
    pub frame: [Option<DriverData>; 20],
}

#[derive(Debug, PartialEq)]
pub struct VisualizationData {
    pub update_rate_ms: u32,
    pub frames: Box<[UpdateFrame]>,
}

// Custom serialization for VisualizationData
impl Serialize for VisualizationData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.frames.len()))?;
        for frame in self.frames.iter() {
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
            type Value = Vec<UpdateFrame>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an array of UpdateFrame")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut frames = Vec::with_capacity(8879); // Allocate capacity to avoid reallocation
                while let Some(frame) = seq.next_element()? {
                    frames.push(frame);
                }
                Ok(frames)
            }
        }

        let frames = deserializer.deserialize_seq(FrameVisitor)?;
        Ok(VisualizationData {
            update_rate_ms: 250,
            frames: frames.into_boxed_slice(),
        })
    }
}

fn main() -> std::io::Result<()> {
    // Read the CSV file
    let file_path = "zandvoort_grouped_1hz.csv";
    let mut rdr = csv::Reader::from_path(file_path)?;

    let headers = rdr.headers()?.clone();
    let mut frames = Vec::with_capacity(8879); // Use Vec for dynamic allocation

    for result in rdr.records() {
        let record = result.unwrap();
        let mut frame: [Option<DriverData>; 20] = Default::default();

        for (j, field) in record.iter().skip(1).enumerate() {
            let driver_number: u8 = headers[j + 1].parse().unwrap();
            let led_num: u8 = field.parse().unwrap();
            frame[j] = Some(DriverData { driver_number, led_num });
        }

        frames.push(UpdateFrame { frame });
    }

    let visualization_data = VisualizationData {
        update_rate_ms: 250,
        frames: frames.into_boxed_slice(), // Convert Vec to Box<[T]>
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

    // Deserialize JSON
    let json_file = File::open("output.json")?;
    let json_data: VisualizationData = serde_json::from_reader(json_file)?;

    // Deserialize Binary
    let bin_file = File::open("output.bin")?;
    let bin_data: VisualizationData = bincode::deserialize_from(bin_file)
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;

    // Ensure the deserialized data matches
    assert_eq!(json_data, bin_data, "JSON and Binary data do not match!");

    Ok(())
}
