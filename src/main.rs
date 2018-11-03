extern crate qrcodegen;
extern crate clap;
use clap::*;
use qrcodegen::QrCode;
use qrcodegen::QrCodeEcc;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::Read;
mod data_type;
use data_type::*;
mod test;

fn init<'a>()->ArgMatches<'a>{
    app_from_crate!()
    .about("Generate the QR code from the file.\nThe data type is auto-detected.")
    .arg(Arg::with_name("data")
        .long("data")
        .short("d")
        .required(true)
        .takes_value(true)
        .help("Data you want to convert to QR code (file path)"))
    .arg(Arg::with_name("qrcode")
        .long("out")
        .short("o")
        .required(false)
        .takes_value(true)
        .help("QR code image file name (file path)"))
    .arg(Arg::with_name("data_type_binary")
        .long("binary")
        .short("b")
        .required(false)
        .takes_value(false)
        .help("DataType is binary"))
    .arg(Arg::with_name("data_type_text")
        .long("text")
        .short("t")
        .required(false)
        .takes_value(false)
        .help("DataType is text"))
    .get_matches()

}

fn text2qr(data:&str) -> std::result::Result<String,Box<std::error::Error>> {
	let qr: QrCode = QrCode::encode_text(data,  QrCodeEcc::High)?;
	Ok(qr.to_svg_string(1))
}

fn binary2qr(data:&[u8]) -> std::result::Result<String,Box<std::error::Error>> {
	let qr: QrCode = QrCode::encode_binary(data,  QrCodeEcc::High)?;
	Ok(qr.to_svg_string(1))
}

fn data2qr(data:&[u8],kind:DataKind) -> std::result::Result<String,Box<std::error::Error>>{
    if kind == DataKind::Text {
        text2qr(&std::str::from_utf8(&data).unwrap())
    }else{
        binary2qr(&data)
    }
}

fn main() {
    let arg = init();
    let input_filename = arg.value_of("data").unwrap();
    let out_filename = if arg.is_present("qrcode"){
        arg.value_of("qrcode").unwrap()
    }else{
        "qrcode.svg"
    };

    let mut input_file = match OpenOptions::new()
        .read(true).write(false)
        .create_new(false)
        .open(input_filename){
            Ok(fp)=>fp,
            Err(e)=>{
                eprintln!("input file opening error : {}",e);
                return;
            }
        };
    let mut out_file = match OpenOptions::new()
        .read(true).write(true)
        .create_new(true)
        .open(out_filename){
            Ok(fp)=>fp,
            Err(e)=>{
                eprintln!("output file creation error : {}->{}",e,out_filename);
                return;
            }
        };

    let mut data = Vec::new();
    input_file.read_to_end(&mut data).unwrap();

    let data_type = if arg.is_present("data_type_text"){
        DataKind::Text
    }else if arg.is_present("data_type_binary"){
        DataKind::Binary
    }else{
        datatype_detect(&data)
    };

    let svg = match data2qr(&data,data_type) {
        Ok(svg)=>svg,
        Err(e)=>{
            eprintln!("{}",e);
            return;
        }
    };
    out_file.write_all(svg.as_bytes()).unwrap();
}
