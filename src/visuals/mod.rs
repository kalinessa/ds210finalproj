use std::collections::HashMap;

// histogram visualization (written with help of chat gpt)
pub fn draw_histogram(data: &Vec<(usize, usize)>) {
    // finds max value to normalize 
    let max_value = data.iter().map(|(_, count)| count).max().unwrap_or(&0);

    // iterates over the data to draw histogram
    for (degree, count) in data {
        let bar_length = (*count as f32 / *max_value as f32 * 50.0) as usize;
        println!("{:<10} |{}{}", degree, "#".repeat(bar_length), " ".repeat(50 - bar_length));
    }
}