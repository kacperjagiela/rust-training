use std::collections::HashMap;

fn main() {
    fn median_and_mode() {
        let list_of_ints = [15, 231, 127, 23, 50, 23];

        let mut int_vector = Vec::from(list_of_ints);

        int_vector.sort_unstable();

        let len = int_vector.len();

        let median;

        if len % 2 == 0 {
            // Take two in the middle and return arthmetic average
            median = (int_vector[len / 2] + int_vector[len / 2 - 1]) / 2;
        } else {
            median = int_vector[len / 2];
        }

        let mut value_map = HashMap::new();

        for i in int_vector.iter() {
            let value = value_map.entry(i).or_insert(0);

            *value += 1;
        }

        let mut mode = 0;
        let mut mode_key = 0;

        for (key, value) in &value_map {
            if mode < *value {
                mode = *value;
                mode_key = **key;
            }
        }

        println!("{:?}", median);
        println!("{:?}: {:?}", mode_key, mode);
    }

    fn pig_latin(to_pit_latin: String) {
        let vowels = ['a', 'e', 'i', 'o', 'u', 'y'];

        let mut pig_version = to_pit_latin;

        if pig_version.starts_with(vowels) {
            pig_version.push_str("-hay");
        } else {
            let first_char = &pig_version[0..1];
            let rest_of_word = format!("-{first_char}ay");

            pig_version = pig_version[1..].to_string();

            pig_version.push_str(&rest_of_word);
        }

        println!("{:?}", pig_version);
    }

    pig_latin(String::from("apple"));

    median_and_mode();
}
