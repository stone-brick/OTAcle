//! Image processing for Observe module
//!
//! Provides scaling and cropping operations on captured frames.

use base64::Engine;
use image::{ImageBuffer, Rgba};
use crate::observe::types::{CropBlock, CropRegion};

/// Image processor for scaling and cropping operations
pub struct ImageProcessor;

impl ImageProcessor {
    /// Scale image data to target dimensions
    ///
    /// # Arguments
    /// * `data` - Raw RGBA pixel data
    /// * `src_width` - Source image width
    /// * `src_height` - Source image height
    /// * `target_width` - Target width
    /// * `target_height` - Target height
    ///
    /// # Returns
    /// Scaled RGBA pixel data
    pub fn scale(
        &self,
        data: &[u8],
        src_width: u32,
        src_height: u32,
        target_width: u32,
        target_height: u32,
    ) -> Vec<u8> {
        let img: ImageBuffer<Rgba<u8>, _> = match ImageBuffer::from_raw(src_width, src_height, data.to_vec()) {
            Some(img) => img,
            None => {
                eprintln!("Failed to create image buffer from raw data: expected {} bytes ({}x{}x4), got {} bytes",
                    src_width as usize * src_height as usize * 4, src_width, src_height, data.len());
                return Vec::new();
            }
        };

        let scaled = image::imageops::resize(
            &img,
            target_width,
            target_height,
            image::imageops::FilterType::Triangle,
        );

        scaled.into_raw()
    }

    /// Crop image into multiple regions and encode as base64
    ///
    /// # Arguments
    /// * `data` - RGBA pixel data (after scaling)
    /// * `width` - Image width
    /// * `height` - Image height
    /// * `regions` - List of crop regions
    ///
    /// # Returns
    /// List of crop blocks with base64 encoded images
    pub fn crop_regions(
        &self,
        data: &[u8],
        width: u32,
        height: u32,
        regions: &[CropRegion],
    ) -> Vec<CropBlock> {
        let img: ImageBuffer<Rgba<u8>, _> = match ImageBuffer::from_raw(width, height, data.to_vec()) {
            Some(img) => img,
            None => {
                eprintln!("Failed to create image buffer for cropping");
                return Vec::new();
            }
        };

        regions
            .iter()
            .map(|region| {
                // Skip invalid regions
                if region.x >= width || region.y >= height {
                    return CropBlock {
                        x: region.x,
                        y: region.y,
                        w: region.w,
                        h: region.h,
                        image: String::new(),
                    };
                }

                // Clamp crop dimensions to fit within image bounds
                let crop_x = region.x;
                let crop_y = region.y;
                let crop_w = region.w.min(width.saturating_sub(region.x));
                let crop_h = region.h.min(height.saturating_sub(region.y));

                if crop_w == 0 || crop_h == 0 {
                    return CropBlock {
                        x: region.x,
                        y: region.y,
                        w: region.w,
                        h: region.h,
                        image: String::new(),
                    };
                }

                let cropped = image::imageops::crop_imm(
                    &img,
                    crop_x,
                    crop_y,
                    crop_w,
                    crop_h,
                ).to_image();

                let encoded = base64::engine::general_purpose::STANDARD.encode(cropped.as_raw());

                CropBlock {
                    x: region.x,
                    y: region.y,
                    w: crop_w,
                    h: crop_h,
                    image: encoded,
                }
            })
            .collect()
    }
}
