extern crate ffmpeg_next as ffmpeg;

use ffmpeg::format::input;
use ffmpeg::software::scaling::{context::Context, flag::Flags};
use ffmpeg::util::format;
use ffmpeg::util::frame::video::Video as VideoFrame;
use std::path::Path;

const TARGTET_TIME: f64 = 10.0;

pub fn generate_thumbnail(input_path: &str, output_path: &str) {
    ffmpeg::init().unwrap();

    // Abrir o arquivo de vídeo
    let mut input_context = input(&Path::new(input_path)).unwrap();
    let input_stream = input_context
        .streams()
        .best(ffmpeg::media::Type::Video)
        .unwrap();
    let video_stream_index = input_stream.index();
    let mut decoder = input_stream.codec().decoder().video().unwrap();

    // Configurar o escalador para converter o quadro para o formato RGB
    let mut scaler = Context::get(
        decoder.format(),
        decoder.width(),
        decoder.height(),
        format::Pixel::RGB24,
        decoder.width(),
        decoder.height(),
        Flags::BILINEAR,
    )
    .unwrap();

    let target_frame = (TARGTET_TIME * decoder.frame_rate().unwrap().numerator() as f64
        / decoder.frame_rate().unwrap().denominator() as f64) as i64;

    let mut frame = VideoFrame::empty();
    for (stream, packet) in input_context.packets() {
        if stream.index() == video_stream_index {
            decoder.send_packet(&packet).unwrap();
            while decoder.receive_frame(&mut frame).is_ok() {
                if frame.pts().unwrap() >= target_frame {
                    let mut rgb_frame = VideoFrame::empty();
                    scaler.run(&frame, &mut rgb_frame).unwrap();
                    save_frame_as_image(&rgb_frame, output_path);
                    log::debug!("Miniatura salva em {}", output_path);
                    return;
                }
            }
        }
    }
}
fn save_frame_as_image(frame: &VideoFrame, path: &str) {
    use image::{ImageBuffer, Rgb};
    let buffer =
        ImageBuffer::<Rgb<u8>, _>::from_raw(frame.width(), frame.height(), frame.data(0).to_vec())
            .unwrap();
    buffer.save(path).unwrap();
}
