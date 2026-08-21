use crate::error::{AudioError, Result};
use opus_decoder::OpusDecoder;
use std::io::Cursor;
use symphonia::core::codecs::audio::well_known::CODEC_ID_OPUS;
use symphonia::core::codecs::audio::{AudioCodecParameters, AudioDecoderOptions};
use symphonia::core::codecs::CodecParameters;
use symphonia::core::formats::probe::Hint;
use symphonia::core::formats::{FormatOptions, FormatReader, Track, TrackType};
use symphonia::core::io::{MediaSource, MediaSourceStream};
use symphonia::core::meta::MetadataOptions;

fn first_audio_track(format_reader: &dyn FormatReader) -> Result<Track> {
    format_reader.first_track(TrackType::Audio).cloned().ok_or(AudioError::InvalidData)
}

fn audio_codec_params(track: &Track) -> Result<&AudioCodecParameters> {
    match track.codec_params.as_ref() {
        Some(CodecParameters::Audio(codec_params)) => Ok(codec_params),
        _ => Err(AudioError::InvalidData),
    }
}

fn decode_with_opus<'s>(
    mut format_reader: Box<dyn FormatReader + 's>,
    track_id: u32,
    codec_params: &AudioCodecParameters,
) -> Result<(Vec<f32>, usize)> {
    let sample_rate = codec_params.sample_rate.ok_or(AudioError::InvalidData)?;
    let channel_count = codec_params.channels.as_ref().map_or(1, |c| c.count()).max(1);

    let mut opus_decoder =
        OpusDecoder::new(sample_rate, channel_count).map_err(|_| AudioError::DecodingError)?;
    let max_frame_len = opus_decoder.max_frame_size_per_channel() * channel_count;

    let mut frame_buffer = vec![0.0f32; max_frame_len];
    let mut samples: Vec<f32> = Vec::new();

    while let Some(packet) = format_reader.next_packet().map_err(|_| AudioError::DecodingError)? {
        if packet.track_id != track_id {
            continue;
        }

        let decoded_sample_count = opus_decoder
            .decode_float(&packet.data, &mut frame_buffer, false)
            .map_err(|_| AudioError::DecodingError)?;

        samples.extend_from_slice(&frame_buffer[..decoded_sample_count * channel_count]);
    }

    Ok((samples, sample_rate as usize))
}

fn decode_with_codec_registry<'s>(
    mut format_reader: Box<dyn FormatReader + 's>,
    track_id: u32,
    codec_params: &AudioCodecParameters,
) -> Result<(Vec<f32>, usize)> {
    let sample_rate = codec_params.sample_rate.ok_or(AudioError::InvalidData)? as usize;

    let mut decoder = symphonia::default::get_codecs()
        .make_audio_decoder(codec_params, &AudioDecoderOptions::default())
        .map_err(|_| AudioError::UnsupportedCodec)?;

    let mut samples: Vec<f32> = Vec::new();
    let mut frame_buffer: Vec<f32> = Vec::new();

    loop {
        let packet = match format_reader.next_packet().map_err(|_| AudioError::DecodingError)? {
            Some(packet) => packet,
            None => break,
        };

        if packet.track_id != track_id {
            continue;
        }

        // `copy_to_vec_interleaved` resizes/overwrites its destination on every call,
        // so it must land in a scratch buffer and get appended, not accumulated directly.
        match decoder.decode(&packet) {
            Ok(audio_buf) => {
                audio_buf.copy_to_vec_interleaved(&mut frame_buffer);
                samples.extend_from_slice(&frame_buffer);
            }
            Err(_) => continue,
        }
    }

    Ok((samples, sample_rate))
}

pub fn decode_bytes(bytes: &[u8]) -> Result<Vec<f32>> {
    let cursor = Cursor::new(bytes);
    let source: Box<dyn MediaSource> = Box::new(cursor);
    let media_source_stream = MediaSourceStream::new(source, Default::default());

    let format_reader = symphonia::default::get_probe()
        .probe(&Hint::new(), media_source_stream, FormatOptions::default(), MetadataOptions::default())
        .map_err(|_| AudioError::DecodingError)?;

    let track = first_audio_track(format_reader.as_ref())?;
    let track_id = track.id;
    let codec_params = audio_codec_params(&track)?.clone();

    let (samples, sample_rate) = if codec_params.codec == CODEC_ID_OPUS {
        decode_with_opus(format_reader, track_id, &codec_params)?
    } else {
        decode_with_codec_registry(format_reader, track_id, &codec_params)?
    };

    if samples.is_empty() {
        return Err(AudioError::ProcessingError);
    }

    let duration_sec = samples.len() as f32 / sample_rate as f32;
    let points_per_second = 20;
    let desired_points = (duration_sec * points_per_second as f32).round() as usize;
    let chunk_size = if desired_points == 0 { samples.len() } else { samples.len() / desired_points };

    if chunk_size == 0 {
        return Err(AudioError::ProcessingError);
    }

    let mut rms_values = Vec::with_capacity(desired_points);

    for chunk in samples.chunks(chunk_size) {
        if chunk.is_empty() {
            continue;
        }

        let rms = (chunk.iter().map(|x| x * x).sum::<f32>() / chunk.len() as f32).sqrt();
        rms_values.push(rms);
    }

    if rms_values.is_empty() {
        return Err(AudioError::ProcessingError);
    }

    let max_rms = rms_values.iter().cloned().fold(0.0, f32::max);
    if max_rms == 0.0 {
        return Err(AudioError::ProcessingError);
    }

    let normalized_samples: Vec<f32> = rms_values.iter().map(|v| v / max_rms).collect();

    Ok(normalized_samples)
}
