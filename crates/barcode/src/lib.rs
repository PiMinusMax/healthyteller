//! Barcode recognition in Rust, so scanning also works in the web app on iOS,
//! where browsers offer no barcode API. Only product codes are read.

use std::collections::HashSet;

use rxing::{BarcodeFormat, DecodeHints};

/// Product barcodes on food packaging.
const FORMATS: [BarcodeFormat; 4] = [
    BarcodeFormat::EAN_13,
    BarcodeFormat::EAN_8,
    BarcodeFormat::UPC_A,
    BarcodeFormat::UPC_E,
];

/// Reads a product barcode from a grey image (one byte per pixel, row by row).
pub fn decode_luma(luma: Vec<u8>, width: u32, height: u32) -> Option<String> {
    if width == 0 || height == 0 || luma.len() != (width as usize) * (height as usize) {
        return None;
    }
    let mut hints = DecodeHints {
        PossibleFormats: Some(HashSet::from(FORMATS)),
        ..DecodeHints::default()
    };
    rxing::helpers::detect_in_luma_with_hints(luma, width, height, None, &mut hints)
        .ok()
        .map(|r| r.getText().to_owned())
}

/// Converts RGBA pixels, as a canvas delivers them, to grey values.
pub fn rgba_to_luma(rgba: &[u8]) -> Vec<u8> {
    rgba.as_chunks::<4>()
        .0
        .iter()
        .map(|p| {
            // ITU-R BT.601 weights, integer arithmetic.
            ((u32::from(p[0]) * 299 + u32::from(p[1]) * 587 + u32::from(p[2]) * 114) / 1000) as u8
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rxing::{MultiFormatWriter, Writer};

    /// Renders a barcode into a grey image with a white border, like a camera frame.
    fn render(code: &str, format: BarcodeFormat) -> (Vec<u8>, u32, u32) {
        let matrix = MultiFormatWriter
            .encode(code, &format, 300, 100)
            .expect("encode");
        let (w, h) = (matrix.getWidth() + 40, matrix.getHeight() + 40);
        let mut luma = vec![255u8; (w * h) as usize];
        for y in 0..matrix.getHeight() {
            for x in 0..matrix.getWidth() {
                if matrix.get(x, y) {
                    luma[((y + 20) * w + x + 20) as usize] = 0;
                }
            }
        }
        (luma, w, h)
    }

    #[test]
    fn reads_ean13_and_ean8() {
        let (luma, w, h) = render("4000000000013", BarcodeFormat::EAN_13);
        assert_eq!(decode_luma(luma, w, h).as_deref(), Some("4000000000013"));
        let (luma, w, h) = render("96385074", BarcodeFormat::EAN_8);
        assert_eq!(decode_luma(luma, w, h).as_deref(), Some("96385074"));
    }

    #[test]
    fn empty_or_wrong_sized_images_give_nothing() {
        assert_eq!(decode_luma(vec![255; 100], 10, 10), None);
        assert_eq!(decode_luma(vec![255; 99], 10, 10), None);
        assert_eq!(decode_luma(Vec::new(), 0, 0), None);
    }

    #[test]
    fn rgba_to_grey() {
        assert_eq!(
            rgba_to_luma(&[255, 255, 255, 255, 0, 0, 0, 255]),
            vec![255, 0]
        );
    }
}
