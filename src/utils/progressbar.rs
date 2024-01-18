pub fn visualise_progress(done: u32, total: u32) {
    let scale_factor = total / 10;
    let progress_value = (done / scale_factor).min(10);

    // Symbol to represent what has been done
    let done_symbol = "=";

    // Symbol to represent what still has to be done
    let remaining_symbol = ".";

    // Visualize progress
    print!(
        "\rProgress ({}/{}): [{}{}]",
        done,
        total,
        done_symbol.repeat(progress_value as usize),
        remaining_symbol.repeat(10 - progress_value as usize)
    );
}
