#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use qrcode_generator::QrCodeEcc;
use napi::bindgen_prelude::*;
use base64::{Engine as _, engine::general_purpose};
use image_convert::{ ImageResource, to_jpg, JPGConfig };

const DEFAULT_RENDER_SIZE: u32 = 1024;

#[napi]
pub enum QRCorrectionLevel {
	Low,
	Medium,
	Quartile,
	High,
}

#[napi]
pub enum QrImagesType {
  JPG,
  GIF,
  PNG,
}

#[napi(object)]
pub struct QrCodeOptions {
  pub data: String,
  pub ecc: Option<QRCorrectionLevel>,
  pub ext: Option<QrImagesType>,
  pub size: Option<u32>
}

#[napi]
pub struct QrGen {}

#[napi]
impl QrGen {
  #[napi]
  pub async fn to_url(options: QrCodeOptions) -> Result<String> {
    let result = img_to_vec(options).unwrap();
    let ext = options.ext.unwrap_or(QrImagesType::PNG);

    let mime_type = ext.as_mime_type();
    let base64_encoded = general_purpose::STANDARD_NO_PAD.encode(result);

    Ok(format!("data:{};base64,{}", mime_type, base64_encoded))
  }

  #[napi]
  pub async fn to_buff(options: QrCodeOptions) -> Result<Buffer> {
    let result = img_to_vec(options).unwrap();
    Ok(result.into())
  }

  #[napi]
  pub async fn to_file(options: QrCodeOptions, path: String) -> Result<()> {
    png_to_file(options, path)
  }

  #[napi]
  pub async fn to_svg(options: QrCodeOptions) -> Result<String> {
    let result = svg_to_string(options);
    Ok(result)
  }
}

fn img_to_vec(options: QrCodeOptions) -> Result<Vec<u8>> {
  let size = options.size.unwrap_or(DEFAULT_RENDER_SIZE) as usize;
  let ecc = options.ecc.unwrap_or(QRCorrectionLevel::Low);
  let ext = options.ext.unwrap_or(QrImagesType::PNG);
  
  let result = qrcode_generator::to_image(
      options.data, 
      From::from(ecc), 
      size
    ).unwrap();

  let image = convert_image(result, ext.as_mime_type());

  Ok(image)
}

fn png_to_file(options: QrCodeOptions, path: String) -> Result<()> {
  let size = options.size.unwrap_or(DEFAULT_RENDER_SIZE) as usize;
  let ecc = options.ecc.unwrap_or(QRCorrectionLevel::Low);

  qrcode_generator::to_png_to_file(
    options.data, 
    From::from(ecc), 
    size, 
    path,
  ).unwrap();

  Ok(())
}

fn svg_to_string(options: QrCodeOptions) -> String {
  let size = options.size.unwrap_or(DEFAULT_RENDER_SIZE) as usize;
  let ecc = options.ecc.unwrap_or(QRCorrectionLevel::Low);

  qrcode_generator::to_svg_to_string(
    options.data, 
    From::from(ecc), 
    size, 
    None::<&str>
  ).unwrap()
}

fn convert_image(vec: Vec<u8>, mime: &str) -> Vec<u8> {
  let mut output = None;
  let mut input = ImageResource::from_reader(vec);

  match mime {
    "image/jpeg" => to_jpg(output, input, &JPGConfig::new())
  };

  output.unwrap()
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

impl QrImagesType {
  fn as_mime_type(&self) -> &str {
    match self {
      QrImagesType::JPEG => "image/jpeg",
      QrImagesType::JPG => "image/jpg",
      QrImagesType::GIF => "image/gif",
      QrImagesType::PNG => "image/png",
    }
  }
}
