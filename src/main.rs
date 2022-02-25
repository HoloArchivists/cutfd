/* 2 files, one is a cut version of the other
 * Cut the files into relatively small samples
 * Compare using cross correlation
 * Use binary search to find the first difference (different meaning it's before)
 * Compare each sample of the original file starting from the first difference
 * to the next (few) sample of the cut version
 */
// use basic_dsp::CrossCorrelationArgumentOps;
// use basic_dsp::CrossCorrelationOps;
// use basic_dsp::SingleBuffer;
// use basic_dsp::ToComplexVector;
// use itertools::Chunks;
use clap::{arg, crate_description, crate_version, Command};
use hound::WavReader;
use itertools::Itertools;
use parse_duration::parse;
use rstats::Vecg;
use std::fs::File;
use std::time;
use std::{f32, io::BufReader};
use super_mass::mass;

fn format_time(time: f32) -> String {
    let milliseconds = (time % 1.0 * 1000.0) as usize;
    let tot_seconds = time as usize;
    let seconds = tot_seconds % 60;
    let minutes = (tot_seconds / 60) % 60;
    let hours = tot_seconds / 60 / 60;
    format!("{}:{}:{}.{}", hours, minutes, seconds, milliseconds)
}

fn find_cut(
    mut original: WavReader<BufReader<File>>,
    mut copy: WavReader<BufReader<File>>,
    window_len: time::Duration,
) {
    let chunk_len = 44100;
    let mut offset = 0;
    let mut cut_duration = 0;
    let mut length: u32;
    let mut chunk_no: u32;
    while offset != copy.len() {
        length = copy.len() - offset;
        chunk_no = (length / 2) + offset;
        // Find start cut
        let mut stage = 1;
        // println!(
        //     "orig length: {} , offset: {} , cut_duration: {}",
        //     copy.len(),
        //     offset,
        //     cut_duration
        // );
        // println!(
        //     "chunk_no wo offset: {} , length wo offset: {}",
        //     chunk_no - offset,
        //     length
        // );
        for _ in 0..((length as f32).log2() as usize) {
            // println!("chunk_no: {}", chunk_no);
            original
                .seek(chunk_no + cut_duration)
                .expect("seeking error");
            copy.seek(chunk_no).expect("seeking error");
            let orig_sampl: Vec<f32> = original
                .samples::<f32>()
                .map(|x| x.expect("Failed to read sample"))
                .chunks(chunk_len as usize)
                .into_iter()
                .next()
                .expect("Failed to fetch sample")
                .collect();

            let cp_sampl: Vec<f32> = copy
                .samples::<f32>()
                .map(|x| x.expect("Failed to read sample"))
                .chunks(chunk_len as usize)
                .into_iter()
                .next()
                .expect("Failed to fetch sample")
                .collect();
            let corr = orig_sampl.mediancorr(&cp_sampl);
            // println!("chunk_no: {}, corr: {}", chunk_no + offset_cut, corr);
            if corr > 0.95 {
                chunk_no = chunk_no + length / (2 as u32).pow(stage)
            } else {
                chunk_no = chunk_no - length / (2 as u32).pow(stage)
            }
            stage += 1;
        }

        let tot_seconds = chunk_no as f32 / 44100.0;
        let time_str = format_time(tot_seconds);
        println!("Cut at {}", time_str);

        // Find end cut
        let window_len = window_len.as_secs() * 44100;
        let mut window_samples: Vec<f64> = Vec::new();
        let mut orig_samples = original
            .samples::<f32>()
            .map(|x| x.expect("Failed to read sample") as f64);
        let cp_sampl: Vec<f64> = copy
            .samples::<f32>()
            .map(|x| x.expect("Failed to read sample") as f64)
            .chunks(chunk_len as usize)
            .into_iter()
            .next()
            .unwrap()
            .collect();
        for _ in 0..window_len {
            window_samples.extend(orig_samples.next())
        }
        let distances = mass(&window_samples, &cp_sampl);
        let (arg_min, _) = distances
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.partial_cmp(b).expect("Encountered Not a Number(NaN)"))
            .unwrap();
        cut_duration += arg_min as u32;
        offset = chunk_no;
        let tot_seconds = (chunk_no + arg_min as u32) as f32 / 44100.0;
        let time_str = format_time(tot_seconds);
        println!("Cut end at {}", time_str);
        // println!(
        //     "orig len: {}, copy + cut: {}",
        //     original.len(),
        //     copy.len() + cut_duration
        // );
        if original.len() <= (copy.len() + cut_duration) {
            break;
        }
        original.seek(offset).expect("Seeking error");
    }
}

fn main() {
    let matches = Command::new("cutfd")
        .bin_name("cutfd")
        .about(crate_description!())
        .version(crate_version!())
        .arg_required_else_help(true)
        .arg(arg!(-a --original <FILE> "Reference file"))
        .arg(arg!(-b --copy <FILE> "Cut file"))
        .arg(arg!(-w --window [TIME] "Time in natural language (use quotes) [default: 10 minutes]"))
        .get_matches();
    // Read arguments

    let original = matches.value_of("original").expect("required");
    let copy = matches.value_of("copy").expect("required");
    let window_len = parse(matches.value_of("window").unwrap_or("10 minutes"))
        .expect("Failed to parse the window time");

    // Read files
    let orig_reader = hound::WavReader::open(original).expect("Failed to open input waveform");
    assert_eq!(orig_reader.spec().channels, 1);
    assert_eq!(orig_reader.spec().sample_rate, 44100);
    assert_eq!(orig_reader.spec().bits_per_sample, 32);

    let copy_reader = hound::WavReader::open(copy).expect("Failed to open input waveform");
    assert_eq!(copy_reader.spec().channels, 1);
    assert_eq!(copy_reader.spec().sample_rate, 44100);
    assert_eq!(copy_reader.spec().bits_per_sample, 32);

    let orig_samples = orig_reader;
    let copy_samples = copy_reader;
    assert!(
        orig_samples.len() > copy_samples.len(),
        "Original is smaller or equal in size to the copy, consider checking the inputs"
    );
    find_cut(orig_samples, copy_samples, window_len)
    // let spec = hound::WavSpec {
    //     channels: 1,
    //     sample_rate: 44100,
    //     bits_per_sample: 16,
    //     sample_format: hound::SampleFormat::Int,
    // };

    // println!("chunk_no\tcorrelation\tconclusion");
    // let mut chunk_no = 0;
    // let mut cp_chunk_no = 0;
    // let mut nb_contig = 0;
    // let mut is_same = true;
    // let mut cp_chunk_vec: Vec<f32> = Vec::new();
    // for orig_chunk in orig_sampl_grp.into_iter() {
    //     chunk_no += 1;
    //     let orig_chunk_vec: Vec<f32> = orig_chunk.into_iter().collect();
    //     if nb_contig < 5 {
    //         cp_chunk_no += 1;
    //         cp_chunk_vec = cp_sampl_iter.next().unwrap().collect();
    //     }
    //     let corr = orig_chunk_vec.mediancorr(&cp_chunk_vec);
    //     println!("Chunk number: {} \t{} \t{}\r", chunk_no, cp_chunk_no, corr);
    //     if corr >= 0.95 {
    //         if !is_same {
    //             // if nb_contig >= 5 {
    //             //     eprintln!("cut at chunk number: {}", chunk_no - &nb_contig);
    //             //     // let mut writer =
    //             //     //     hound::WavWriter::create(&format!("cut_{}.wav", &chunk_no), spec)
    //             //     //         .expect("Failed to open output waveform");
    //             //     // let mut sample_no = 0;
    //             //     // for sample in &contig_buffer {
    //             //     //     let amplitude = i16::MAX as f32;
    //             //     //     let sample = (sample * amplitude) as i16;
    //             //     //     writer
    //             //     //         .write_sample(sample)
    //             //     //         .expect(&format!("Failed to write sample {}", &sample_no));
    //             //     //     sample_no += 1;
    //             //     // }
    //             //     // writer.finalize().expect("Failed to close output waveform");
    //             // }
    //             if nb_contig >= 5 {
    //                 eprintln!("Cut end at chunk number: {}", chunk_no);
    //             }
    //             nb_contig = 0;
    //             is_same = true;
    //         }
    //     } else {
    //         nb_contig += 1;
    //         if nb_contig == 5 {
    //             eprintln!("Cut at chunk number: {}", chunk_no - nb_contig);
    //         }
    //         is_same = false;
    //     };
    // println!("{}\t{}\t{}", &chunk_no, &corr, &is_same);

    // let max = chunk_vec.iter().cloned().fold(0. / 0., f32::max);
    // let normalized_vec = chunk_vec.iter().map(|x| x / max).into_iter();
    // // let mut norm_dsp_vec = normalized_vec.to_complex_time_vec();

    // let max = cp_chunk.iter().cloned().fold(0. / 0., f32::max);
    // let norm_cp_vec: Vec<f32> = cp_chunk.iter().map(|x| x / max).collect();
    // let norm_cp_dsp_vec = norm_cp_vec.to_complex_time_vec();

    // let mut buffer = SingleBuffer::new();
    // let argument = norm_cp_dsp_vec.prepare_argument_padded(&mut buffer);
    // norm_dsp_vec
    //     .correlate(&mut buffer, &argument)
    //     .expect("Error in correlation");

    // println!(
    //     "max: {}",
    //     norm_dsp_vec.data.iter().cloned().fold(0. / 0., f32::max)
    // );

    // let len = vector.points();
    // println!("Finished processing {} samples", len);
    // }

    // let mut orig_sampl_iter = orig_sampl_grp.into_iter();
    // let size_window = 3 * 60 * 44100 / chunk_len; // 10 minutes
    // let mut window: Vec<f32> = Vec::new();
    // for _ in 0..size_window {
    //     let sample = orig_sampl_iter.next().expect("This shouldn't fail");
    //     window.extend(sample);
    // }

    // let window_dsp = window.to_complex_time_vec();
    // let mut cp_chunk_dsp = cp_chunk_vec.to_complex_time_vec();

    // let mut buffer = SingleBuffer::new();
    // let argument = window_dsp.prepare_argument(&mut buffer);
    // cp_chunk_dsp
    //     .correlate(&mut buffer, &argument)
    //     .expect("Error in correlation");

    // let window_corr_iter = cp_chunk_dsp.data.iter().cloned();

    // let (arg_max, max) = window_corr_iter
    //     .enumerate()
    //     .max_by(|(_, a), (_, b)| a.partial_cmp(b).expect("Encountered Not a Number(NaN)"))
    //     .unwrap();
    // println!("index: {}, max: {}", arg_max, max);
}
