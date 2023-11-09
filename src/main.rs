use std::collections::HashMap;
use tracing::{ info, error };
use tracing_subscriber::{ filter, prelude::* };

#[allow(dead_code, unused)]
#[derive(Debug)]
struct FieldType {
    name: String,
    size: usize,
    signed: bool,
    divisor: u8,
}

#[derive(Debug)]
#[allow(dead_code, unused)]
struct DecodeData {
    name: String,
    channel: usize,
    value: f64,
}

fn main() {
    tracing_subscriber
        ::registry()
        .with(tracing_subscriber::fmt::layer().pretty().with_filter(filter::LevelFilter::INFO))
        .init();

    let field_type_map: HashMap<usize, FieldType> = HashMap::from([
        (
            0,
            FieldType {
                name: String::from("options"),
                size: 1,
                divisor: 1,
                signed: false,
            },
        ),
        (
            1,
            FieldType {
                name: String::from("backlogCount"),
                size: 2,
                divisor: 1,
                signed: false,
            },
        ),
        (
            2,
            FieldType {
                name: String::from("battery"),
                size: 1,
                divisor: 1,
                signed: false,
            },
        ),
        (
            3,
            FieldType {
                name: String::from("timestamp"),
                size: 4,
                divisor: 1,
                signed: false,
            },
        ),
        (
            4,
            FieldType {
                name: String::from("rssi"),
                size: 1,
                divisor: 1,
                signed: true,
            },
        ),
        (
            5,
            FieldType {
                name: String::from("internal_temperature"),
                size: 2,
                divisor: 100,
                signed: true,
            },
        ),
        (
            6,
            FieldType {
                name: String::from("external_temperature"),
                size: 2,
                divisor: 100,
                signed: true,
            },
        ),
        (
            7,
            FieldType {
                name: String::from("sht85_humidity"),
                size: 1,
                divisor: 2,
                signed: false,
            },
        ),
        (
            8,
            FieldType {
                name: String::from("voltage"),
                size: 2,
                divisor: 100,
                signed: true,
            },
        ),
        (
            9,
            FieldType {
                name: String::from("current"),
                size: 2,
                divisor: 100,
                signed: true,
            },
        ),
        (
            10,
            FieldType {
                name: String::from("door"),
                size: 1,
                divisor: 1,
                signed: false,
            },
        ),
        (
            11,
            FieldType {
                name: String::from("error_mask"),
                size: 2,
                divisor: 1,
                signed: false,
            },
        ),
        (
            12,
            FieldType {
                name: String::from("snr"),
                size: 1,
                divisor: 1,
                signed: true,
            },
        ),
        (
            13,
            FieldType {
                name: String::from("sht85_temperature"),
                size: 2,
                divisor: 100,
                signed: true,
            },
        ),
        (
            20,
            FieldType {
                name: String::from("configuration"),
                size: 2,
                divisor: 1,
                signed: false,
            },
        ),
        (
            21,
            FieldType {
                name: String::from("meas_period"),
                size: 2,
                divisor: 1,
                signed: false,
            },
        ),
        (
            22,
            FieldType {
                name: String::from("epoch"),
                size: 4,
                divisor: 1,
                signed: false,
            },
        ),
        (
            23,
            FieldType {
                name: String::from("get_backlog"),
                size: 2,
                divisor: 1,
                signed: false,
            },
        ),
    ]);

    let mut decoded_results: Vec<DecodeData> = Vec::new();

    let bytes_val: Vec<usize> = vec![
        0,
        10,
        1,
        1,
        11,
        1,
        183,
        2,
        10,
        100,
        3,
        12,
        101,
        75,
        88,
        53,
        4,
        10,
        209,
        5,
        14,
        7,
        213,
        7,
        104,
        99,
        13,
        14,
        7,
        144,
        11,
        11,
        0,
        0,
        12,
        10,
        7
    ];

    let mut i: usize = 0;

    while i < bytes_val.len() {
        let f_type_index = bytes_val.get(i);
        i += 1;
        let channel_no = bytes_val.get(i + 1).unwrap();
        i += 1;
        let ff = field_type_map.get(f_type_index.unwrap());
        if ff.is_some() {
            let found_field = ff.unwrap();
            let end_index = i + found_field.size;
            let slices = &bytes_val[i..end_index];
            let val = array_to_decimal(slices.to_vec(), found_field.signed, found_field.divisor);
            decoded_results.push(DecodeData {
                name: found_field.name.clone(),
                channel: *channel_no,
                value: val,
            });
            i += found_field.size;
        } else {
            error!("Field type not found for");
        }
    }
    // let t = format!("decoded resuts {:#?}", decoded_results);
    info!(" decoded results generated");
}

fn array_to_decimal(stream: Vec<usize>, is_signed: bool, divisor: u8) -> f64 {
    let mut value: i64 = 0;
    for item in stream.iter() {
        let item_val = *item as i64;
        value = (value << 8) | item_val;
    }
    if is_signed {
        let edge = 1 << (stream.len() * 8);
        let max = (edge - 1) >> 1;
        if value > max {
            value = value - edge;
        }
    }
    (value as f64) / f64::from(divisor)
}
