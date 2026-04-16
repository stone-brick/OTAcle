//! Observe 模块的图像处理
//!
//! 提供捕获帧的缩放和裁剪操作。

use base64::Engine;
use image::{ImageBuffer, Rgba};
use crate::observe::types::{CropBlock, CropRegion};

/// 用于缩放和裁剪操作的图像处理器
pub struct ImageProcessor;

impl ImageProcessor {
    /// 将图像数据缩放到目标尺寸
    ///
    /// # 参数
    /// * `data` - 原始 RGBA 像素数据
    /// * `src_width` - 源图像宽度
    /// * `src_height` - 源图像高度
    /// * `target_width` - 目标宽度
    /// * `target_height` - 目标高度
    ///
    /// # 返回
    /// 缩放后的 RGBA 像素数据
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

    /// 将图像裁剪为多个区域并编码为 base64
    ///
    /// # 参数
    /// * `data` - RGBA 像素数据（缩放后）
    /// * `width` - 图像宽度
    /// * `height` - 图像高度
    /// * `regions` - 裁剪区域列表
    ///
    /// # 返回
    /// 包含 base64 编码图像的裁剪块列表
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
                // 跳过无效区域
                if region.x >= width || region.y >= height {
                    return CropBlock {
                        x: region.x,
                        y: region.y,
                        w: region.w,
                        h: region.h,
                        image: String::new(),
                    };
                }

                // 限制裁剪尺寸以适应图像边界
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
