/* 2 files, one is a cut version of the other
 * Cut the files into relatively small samples
 * Compare using cross correlation
 * Use binary search to find the first difference (different meaning it's before)
 * Compare each sample of the original file starting from the first difference
 * to the next (few) sample of the cut version
 */
use clap::{arg, crate_description, crate_version, Command};
use cutfd_lib::{find_beginning, find_cut};
use parse_duration::parse;

fn format_time(time: f32) -> String {
    let milliseconds = (time % 1.0 * 1000.0) as usize;
    let tot_seconds = time as usize;
    let seconds = tot_seconds % 60;
    let minutes = (tot_seconds / 60) % 60;
    let hours = tot_seconds / 60 / 60;
    format!("{}:{}:{}.{}", hours, minutes, seconds, milliseconds)
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
        .arg(arg!(-'1' --"one-cut" ... "Only search for the first cut"))
        .get_matches();
    // Read arguments

    let original = matches.value_of("original").expect("required");
    let copy = matches.value_of("copy").expect("required");
    let only_one = matches.is_present("one-cut");
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
    if only_one {
        let mut original = orig_samples;
        let mut copy = copy_samples;
        let beginning = find_beginning(&mut original, &mut copy, 0, 0);
        let tot_seconds = beginning as f32 / 44100.0;
        let time_str = format_time(tot_seconds);
        println!("Cut at {}", time_str);

        let length = &original.duration();
        let copy_len = &copy.duration();
        let difference = length - copy_len;
        let end = beginning + difference;
        let tot_seconds = end as f32 / 44100.0;
        let time_str = format_time(tot_seconds);
        println!("Cut at {}", time_str);
    } else {
        let vec_result = find_cut(orig_samples, copy_samples, window_len);
        for [beginning, end] in vec_result {
            let tot_seconds = beginning as f32 / 44100.0;
            let time_str = format_time(tot_seconds);
            println!("Cut at {}", time_str);
            let tot_seconds = end as f32 / 44100.0;
            let time_str = format_time(tot_seconds);
            println!("Cut end at {}", time_str);
        }
    }
}
