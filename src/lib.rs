use std::io::Cursor;
use wasm_bindgen::prelude::*;
use image::{DynamicImage, GenericImageView, ImageBuffer, ImageFormat, Rgba};
use base64::{engine::general_purpose, Engine as _};
use image::imageops::FilterType;

#[wasm_bindgen]
pub fn zoom_img(str_url: String,point_x: u32,point_y: u32) -> String {
    let comma_pos = str_url.find(',').unwrap();
    let str_base64 :String  =Some(&str_url[comma_pos + 1..]).unwrap().to_string();
    let image_data = general_purpose::STANDARD.decode(str_base64).unwrap();
    let img =  image::load_from_memory_with_format(&image_data, ImageFormat::Png).unwrap();
    let scaled_img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_fn(
        img.width(),img.height(),
        |i, j| {
            let original_x = point_x * 4 + i / 16;// zoom 16倍
            let original_y = point_y * 4 + j / 16;
            img.get_pixel(original_x, original_y).clone()
        },
    );
    let mut buffer = Cursor::new(Vec::new());
    scaled_img.write_to(&mut buffer, ImageFormat::Png).unwrap();
    let base64_string = general_purpose::STANDARD.encode(buffer.get_ref());
    // データURL形式で返す
    format!("data:image/png;base64,{}", base64_string)
}

#[wasm_bindgen]
pub fn resize_img(image_data: &[u8]) -> String {
    let img = resize_image(image::load_from_memory(image_data).unwrap());
    let mut buffer = Cursor::new(Vec::new());
    img.write_to(&mut buffer, ImageFormat::Png).unwrap();
    let base64_string = general_purpose::STANDARD.encode(buffer.get_ref());
    // データURL形式で返す
    format!("data:image/png;base64,{}", base64_string)
}

// 画像のアスペクト比・操作しやすいサイズに設定・調整
fn resize_image(img: DynamicImage) -> DynamicImage {
    let aspect_ratio = img.width() as f32 / img.height() as f32;
    let (new_width, new_height) = if aspect_ratio > 1.0 {
        // 横長の画像の場合
        (1280, (1280.0 / aspect_ratio) as u32)
    } else {
        // 縦長の画像の場合
        ((1280.0 * aspect_ratio) as u32, 1280)
    };
    // リサイズ
    let resized_img = img.resize_exact(new_width, new_height, FilterType::Lanczos3);
    // 1280x1280のキャンバスを作成
    let mut canvas = DynamicImage::new_rgba8(1280, 1280);
    // 白を背景色とする
    let bkg_image= ImageBuffer::from_pixel(1280, 1280, Rgba([255, 255, 255, 255]));
    image::imageops::overlay(&mut canvas, &bkg_image, 0, 0);
    // リサイズ画像をキャンバスの中央に貼付
    let x = (1280 - resized_img.width()) / 2;
    let y = (1280 - resized_img.height()) / 2;
    image::imageops::overlay(&mut canvas, &resized_img, x.into(), y.into());
    canvas
}