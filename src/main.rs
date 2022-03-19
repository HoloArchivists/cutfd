/* 2 files, one is a cut version of the other
 * Cut the files into relatively small samples
 * Compare using cross correlation
 * Use binary search to find the first difference (different meaning it's before)
 * Compare each sample of the original file starting from the first difference
 * to the next (few) sample of the cut version
 */
use clap::{arg, crate_description, crate_version, Command};
use cutfd_lib::find_cut;
use parse_duration::parse;

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
