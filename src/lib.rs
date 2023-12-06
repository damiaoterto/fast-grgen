#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use qrcode_generator::QrCodeEcc;
use napi::bindgen_prelude::*;
use base64::{Engine as _, engine::general_purpose};

#[napi]
pub enum QRCorrectionLevel {
	Low,
	Medium,
	Quartile,
	High,
}

#[napi(object)]
pub struct QrCodeOptions {
  pub data: String,
  pub ecc: Option<QRCorrectionLevel>,
  pub size: Option<u32>
}

#[napi]
pub struct QrGen {}

#[napi]
impl QrGen {
  #[napi]
  pub async fn to_url(options: QrCodeOptions) -> Result<String> {
    let result = png_to_vec(options).unwrap();

    let mime_type = "image/png";
    let base64_encoded = general_purpose::STANDARD_NO_PAD.encode(result);

    Ok(format!("data:{};base64,{}", mime_type, base64_encoded))
  }

  #[napi]
  pub async fn to_buff(options: QrCodeOptions) -> Result<Buffer> {
    let result = png_to_vec(options).unwrap();
    Ok(result.into())
  }

  #[napi]
  pub async fn to_file(options: QrCodeOptions, path: String) -> Result<()> {
    png_to_file(options, path)
  }
}

fn png_to_vec(options: QrCodeOptions) -> Result<Vec<u8>> {
  let size = options.size.unwrap_or(1024) as usize;
  let ecc = options.ecc.unwrap_or(QRCorrectionLevel::Low);
  
  let result = qrcode_generator::to_png_to_vec(
      options.data, 
      From::from(ecc), 
      size
    ).unwrap();

  Ok(result)
}

fn png_to_file(options: QrCodeOptions, path: String) -> Result<()> {
  let size = options.size.unwrap_or(1024) as usize;
  let ecc = options.ecc.unwrap_or(QRCorrectionLevel::Low);

  qrcode_generator::to_png_to_file(
    options.data, 
    From::from(ecc), 
    size, 
    path,
  ).unwrap();

  Ok(())
}

impl From<QRCorrectionLevel> for QrCodeEcc {
    fn from(level: QRCorrectionLevel) -> Self {
        match level {
            QRCorrectionLevel::Low => QrCodeEcc::Low,
            QRCorrectionLevel::Medium => QrCodeEcc::Medium,
            QRCorrectionLevel::Quartile => QrCodeEcc::Quartile,
            QRCorrectionLevel::High => QrCodeEcc::High,
        }
    }
}
