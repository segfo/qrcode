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
#[cfg(test)]
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
    .arg(Arg::with_name("string")
        .long("string")
        .short("s")
        .required(false)
        .help("text string you want to convert to QR code"))
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

fn text2qr(data:&str) -> std::result::Result<String,Box<dyn std::error::Error>> {
	let qr: QrCode = QrCode::encode_text(data,  QrCodeEcc::High)?;
	Ok(to_svg_string(&qr,1))
}

fn binary2qr(data:&[u8]) -> std::result::Result<String,Box<dyn std::error::Error>> {
	let qr: QrCode = QrCode::encode_binary(data,  QrCodeEcc::High)?;
	Ok(to_svg_string(&qr,1))
}

fn data2qr(data:&[u8],kind:DataKind) -> std::result::Result<String,Box<dyn std::error::Error>>{
    if kind == DataKind::Text {
        text2qr(&std::str::from_utf8(&data).unwrap())
    }else{
        binary2qr(&data)
    }
}

fn load_to_file(input_filename:&str)->std::result::Result<Vec<u8>,Box<dyn std::error::Error>>{
    let mut input_file = OpenOptions::new()
        .read(true).write(false)
        .create_new(false)
        .open(input_filename)?;

    let mut data = Vec::new();
    input_file.read_to_end(&mut data).unwrap();
    Ok(data)
}

fn main() {
    let arg = init();
    let input_filename = arg.value_of("data").unwrap();
    let out_filename = if arg.is_present("qrcode"){
        arg.value_of("qrcode").unwrap()
    }else{
        "qrcode.svg"
    };

    let data = if arg.is_present("string"){
        input_filename.as_bytes().iter().cloned().collect()
    }else{
        load_to_file(input_filename).unwrap()
    };
    let mut out_file = match OpenOptions::new()
        .read(true).write(true)
        .create_new(false)
        .open(out_filename){
            Ok(fp)=>fp,
            Err(e)=>{
                eprintln!("output file creation error : {}->{}",e,out_filename);
                return;
            }
        };

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

fn to_svg_string(qr: &QrCode, border: i32) -> String {
	assert!(border >= 0, "Border must be non-negative");
	let mut result = String::new();
	result += "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n";
	result += "<!DOCTYPE svg PUBLIC \"-//W3C//DTD SVG 1.1//EN\" \"http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd\">\n";
	let dimension = qr.size().checked_add(border.checked_mul(2).unwrap()).unwrap();
	result += &format!(
		"<svg xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\" viewBox=\"0 0 {0} {0}\" stroke=\"none\">\n", dimension);
	result += "\t<rect width=\"100%\" height=\"100%\" fill=\"#FFFFFF\"/>\n";
	result += "\t<path d=\"";
	for y in 0 .. qr.size() {
		for x in 0 .. qr.size() {
			if qr.get_module(x, y) {
				if x != 0 || y != 0 {
					result += " ";
				}
				result += &format!("M{},{}h1v1h-1z", x + border, y + border);
			}
		}
	}
	result += "\" fill=\"#000000\"/>\n";
	result += "</svg>\n";
	result
}