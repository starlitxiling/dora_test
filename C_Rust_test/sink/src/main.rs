use dora_node_api::arrow::array::PrimitiveArray;
use dora_node_api::arrow::datatypes::UInt8Type;
use dora_node_api::{self, DoraNode, Event};

use dora_node_api::arrow::buffer::ScalarBuffer;
use std::str;

fn main() -> eyre::Result<()> {
    let (_node, mut events) = DoraNode::init_from_env()?;

    // 定义计数变量
    let mut count = 0;

    while let Some(event) = events.recv() {
        match event {
            Event::Input {
                id: _,
                data,
                metadata: _,
            } => {
                // 每次接收到 Event::Input 事件时递增计数变量
                count += 1;

                let array = data
                    .as_any()
                    .downcast_ref::<PrimitiveArray<UInt8Type>>()
                    .expect("Expected a PrimitiveArray of UInt8Type");

                let buffer: &ScalarBuffer<u8> = array.values();

                let bytes: &[u8] = unsafe {
                    std::slice::from_raw_parts(buffer.as_ptr(), buffer.len())
                };
                
                match str::from_utf8(bytes) {
                    // Ok(s) => println!("received message {}: {}", count, s),
                    Ok(s) => println!("{}", count),
                    Err(e) => println!("failed: {}", e),
                }
            }
            Event::InputClosed { id } => {
                println!("Input `{id}` was closed");
            }
            other => eprintln!("Received unexpected input: {other:?}"),
        }
    }

    println!("finished with {} messages received", count);
    Ok(())
}
