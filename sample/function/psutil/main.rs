/*
    psutil crate를 이용한 pc정보 확인
    conv crate를 이용한 안전한 자료형 변환

    1.참조 사이트
        1)psutil crate : https://crates.io/crates/psutil
        2)conv crate : https://crates.io/crates/conv

    2. Cargo.toml파일 버전 설정 :
        [dependencies]
        psutil = "3.2.2"
        conv ="0.3.3"
        anyhow="1.0"

    3. 동작설명
      1)400ms delay로 cpu 사용률 체크 후 결과를 변환/저장
      2)disk 사용정보/메모리 사용정보 체크 후 결과를 변환/저장
      3)계산한 결과가 정상적이면 vec에 (..)형태로 push후 출력
*/

use anyhow::{anyhow, Result};
use conv::ValueFrom;
use psutil::*;

use std::thread;
use std::time::Duration;

const BYTE_UNIT: u64 = 1000;
const IBYTE_UNIT: u64 = 1024;
const POINT_UNIT: u64 = 10;
const INIT_VAL: f32 = -1.0;

#[derive(Copy, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ByteUnit {
    KB,
    KiB,
    MB,
    MiB,
    GB,
    GiB,
    TB,
    TiB,
    PB,
    PiB,
    EB,
    EiB,
    ZB,
    ZiB,
}

fn convert_byte_unit(byte_value: u64, point_unit: u32, byte_unit: ByteUnit) -> Result<f32> {
    let point_val = POINT_UNIT.pow(point_unit);
    let mut div_val = 1;
    match byte_unit {
        ByteUnit::KB => div_val = BYTE_UNIT.pow(1),
        ByteUnit::KiB => div_val = IBYTE_UNIT.pow(1),
        ByteUnit::MB => div_val = BYTE_UNIT.pow(2),
        ByteUnit::MiB => div_val = IBYTE_UNIT.pow(2),
        ByteUnit::GB => div_val = BYTE_UNIT.pow(3),
        ByteUnit::GiB => div_val = IBYTE_UNIT.pow(3),
        ByteUnit::TB => div_val = BYTE_UNIT.pow(4),
        ByteUnit::TiB => div_val = IBYTE_UNIT.pow(4),
        ByteUnit::PB => div_val = BYTE_UNIT.pow(5),
        ByteUnit::PiB => div_val = IBYTE_UNIT.pow(5),
        ByteUnit::EB => div_val = BYTE_UNIT.pow(6),
        ByteUnit::EiB => div_val = IBYTE_UNIT.pow(6),
        ByteUnit::ZB => div_val = BYTE_UNIT.pow(7),
        ByteUnit::ZiB => div_val = IBYTE_UNIT.pow(7),
    };
    let conv_val = byte_value * point_val / div_val;
    if let Ok(result) = f32::value_from(conv_val) {
        if let Ok(div_point) = f32::value_from(point_val) {
            return Ok(result / div_point);
        }
    }
    Err(anyhow!("fail convert byte unit"))
}

fn main() {
    let mut sysinfo = Vec::new();
    let mut cpu_percent: f32 = INIT_VAL;
    let mut usage_disk: f32 = INIT_VAL;
    let mut total_disk: f32 = INIT_VAL;
    let mut usage_mem: f32 = INIT_VAL;
    let mut total_mem: f32 = INIT_VAL;

    if let Ok(mut cpu_percent_collector) = cpu::CpuPercentCollector::new() {
        thread::sleep(Duration::from_millis(400));
        if let Ok(cpu_val) = cpu_percent_collector.cpu_percent() {
            cpu_percent = (cpu_val * 10.0).round() / 10.0;
        }
    }

    if let Ok(disk_usage) = disk::disk_usage("/") {
        if let Ok(ret) = convert_byte_unit(disk_usage.used(), 2, ByteUnit::TB) {
            usage_disk = ret;
        }
        if let Ok(ret) = convert_byte_unit(disk_usage.total(), 2, ByteUnit::TB) {
            total_disk = ret;
        }
    }

    if let Ok(virtual_memory) = memory::virtual_memory() {
        if let Ok(ret) = convert_byte_unit(virtual_memory.used(), 1, ByteUnit::GB) {
            usage_mem = ret;
        }
        if let Ok(ret) = convert_byte_unit(virtual_memory.total(), 1, ByteUnit::GB) {
            total_mem = ret;
        }
    }

    if cpu_percent < INIT_VAL
        || usage_disk < INIT_VAL
        || total_disk < INIT_VAL
        || usage_mem < INIT_VAL
        || total_mem < INIT_VAL
    {
        println!("psutil fail!!");
    } else {
        sysinfo.push((cpu_percent, usage_disk, total_disk, usage_mem, total_mem));
        println!("{:?}", sysinfo);
    }
}
