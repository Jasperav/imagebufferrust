use image::ImageFormat;

fn main() {
    {
        let pixels = image::open("test_image_png.png").unwrap().raw_pixels();
        let converted = image::load_from_memory(&pixels);
        let converted_with_format = image::load_from_memory_with_format(&pixels, ImageFormat::Png).unwrap();
    }

    {
        let pixels = image::open("test_image_jpg.jpg").unwrap().raw_pixels();
        let converted = image::load_from_memory(&pixels);
        let converted_with_format = image::load_from_memory_with_format(&pixels, ImageFormat::Jpeg).unwrap();
    }
}
