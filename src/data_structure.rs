
#[derive(Debug)]
pub struct DriverData {
    pub driver_number: u32,
    pub led_num: u32,
}

#[derive(Debug)]
pub struct UpdateFrame {
    pub drivers: [Option<DriverData>; 20],
}

#[derive(Debug)]
pub struct VisualizationData {
    pub update_rate_ms: u32,
    pub frames: [UpdateFrame; 8879],
}



pub const VISUALIZATION_DATA: VisualizationData = VisualizationData {
    update_rate_ms: 250,
    frames: [
UpdateFrame {
    drivers: [
        DriverData {
            driver_number: 23,
            led_num: 65,
        },
        DriverData {
            driver_number: 14,
            led_num: 48,
        },
        DriverData {
            driver_number: 77,
            led_num: 95,
        },
        DriverData {
            driver_number: 10,
            led_num: 51,
        },
        DriverData {
            driver_number: 24,
            led_num: 25,
        },
        DriverData {
            driver_number: 44,
            led_num: 58,
        },
        DriverData {
            driver_number: 27,
            led_num: 51,
        },
        DriverData {
            driver_number: 40,
            led_num: 47,
        },
        DriverData {
            driver_number: 16,
            led_num: 9,
        },
        DriverData {
            driver_number: 20,
            led_num: 84,
        },
        DriverData {
            driver_number: 4,
            led_num: 52,
        },
        DriverData {
            driver_number: 31,
            led_num: 33,
        },
        DriverData {
            driver_number: 11,
            led_num: 54,
        },
        DriverData {
            driver_number: 81,
            led_num: 5,
        },
        DriverData {
            driver_number: 63,
            led_num: 70,
        },
        DriverData {
            driver_number: 55,
            led_num: 20,
        },
        DriverData {
            driver_number: 2,
            led_num: 51,
        },
        DriverData {
            driver_number: 18,
            led_num: 47,
        },
        DriverData {
            driver_number: 22,
            led_num: 46,
        },
        DriverData {
            driver_number: 1,
            led_num: 51,
        },
    ],
},
    ]};