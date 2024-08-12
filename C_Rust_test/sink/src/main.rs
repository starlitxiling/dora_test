use csv::Writer;
use dora_node_api::arrow::array::{AsArray, PrimitiveArray};
use dora_node_api::arrow::datatypes::UInt8Type;
use dora_node_api::{self, DoraNode, Event};
use eyre::ContextCompat;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use uhlc::system_time_clock;
use uhlc::HLC;

use dora_node_api::arrow::array::{ArrayRef};
use dora_node_api::arrow::buffer::ScalarBuffer;
use std::str;

static LANGUAGE: &str = "Rust";
static PLATFORM: &str = "i7-8750@2.20GHz";
static NAME: &str = "dora-rs daemon Rust";

fn main() -> eyre::Result<()> {
    let (_node, mut events) = DoraNode::init_from_env()?;

    // latency is tested first
    let latency = true;

    let mut current_size = 0;
    let mut n = 0;
    let mut start = Instant::now();
    let mut latencies = Vec::new();
    let hlc = HLC::default();
    let timestamp = hlc.new_timestamp().to_string();
    let date = timestamp
        .split('.')
        .next()
        .expect("Could not extract date from timestamp.");

    // Preallocated vector
    let sizes = [1, 10 * 512, 100 * 512, 1000 * 512, 10000 * 512];
    let mut root_vec = HashMap::new();
    for size in sizes {
        root_vec.insert(size, vec![0u64; size]);
    }

    while let Some(event) = events.recv() {
        match event {
            Event::Input {
                id: _,
                data,
                metadata: _,
            } => {      
                // check if new size bracket
                let array = data
                    .as_any()
                    .downcast_ref::<PrimitiveArray<UInt8Type>>()
                    .expect("Expected a PrimitiveArray of UInt8Type");

                let buffer: &ScalarBuffer<u8> = array.values();

                let bytes: &[u8] = unsafe {
                    std::slice::from_raw_parts(buffer.as_ptr(), buffer.len())
                };
                
                match str::from_utf8(bytes) {
                    Ok(s) => println!("received message: {}", s),
                    Err(e) => println!("failed: {}", e),
                }
            
                // let time_u64 = array.get(0).context("could not slice data")?;
                // let t_send = uhlc::NTP64((*time_u64).into());

                // .to_vec() Data Latency
                // let _owned_data = array.to_vec();

                // Preallocated data
                // let _ = root_vec
                // .get_mut(&data.len())
                // .unwrap()
                // .copy_from_slice(array);

                // let t_received = system_time_clock();

                // latencies.push((t_received - t_send).to_duration());
                // let data_len = data.len() * 8;
                // if data_len != current_size {
                //     if n > 0 {
                //         record_results(start, current_size, n, latencies, latency, date);
                //     }
                //     current_size = data_len;
                //     n = 0;
                //     start = Instant::now();
                //     latencies = Vec::new();
                // }
                // n += 1;
            }
            Event::InputClosed { id } => {
                println!("Input `{id}` was closed");
            }
            other => eprintln!("Received unexpected input: {other:?}"),
        }
    }

    record_results(start, current_size, n, latencies, latency, date);
    println!("finished");
    Ok(())
}

fn record_results(
    _start: Instant,
    current_size: usize,
    n: u32,
    latencies: Vec<Duration>,
    _latency: bool,
    date: &str,
) {
    let avg_latency = latencies.iter().sum::<Duration>() / n;
    let file = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("timer.csv")
        .unwrap();
    let mut wtr = Writer::from_writer(file);
    let name = std::env::var("NAME").unwrap_or_else(|_| NAME.to_string());

    wtr.write_record(&[
        date.to_string(),
        LANGUAGE.to_string(),
        PLATFORM.to_string(),
        name,
        current_size.to_string(),
        avg_latency.as_micros().to_string(),
    ])
    .unwrap();
}
