/*
In this kata you have to write a Morse code decoder for wired electrical telegraph.
Electric telegraph is operated on a 2-wire line with a key that, when pressed, connects the wires
together, which can be detected on a remote station. The Morse code encodes every character being
transmitted as a sequence of "dots" (short presses on the key) and "dashes" (long presses on the
key).

When transmitting the Morse code, the international standard specifies that:

"Dot" – is 1 time unit long.
"Dash" – is 3 time units long.
Pause between dots and dashes in a character – is 1 time unit long.
Pause between characters inside a word – is 3 time units long.
Pause between words – is 7 time units long.
However, the standard does not specify how long that "time unit" is. And in fact different operators
would transmit at different speed. An amateur person may need a few seconds to transmit a single
character, a skilled professional can transmit 60 words per minute, and robotic transmitters may go
way faster.

For this kata we assume the message receiving is performed automatically by the hardware that checks
the line periodically, and if the line is connected (the key at the remote station is down), 1 is
recorded, and if the line is not connected (remote key is up), 0 is recorded. After the message is
fully received, it gets to you for decoding as a string containing only symbols 0 and 1.

For example, the message HEY JUDE, that is ···· · −·−−   ·−−− ··− −·· · may be received as follows:

1100110011001100000011000000111111001100111111001111110000000000000011001111110011111100111111000000
110011001111110000001111110011001100000011

As you may see, this transmission is perfectly accurate according to the standard, and the hardware
sampled the line exactly two times per "dot".

That said, your task is to implement two functions:

Function decodeBits(bits), that should find out the transmission rate of the message, correctly
decode the message to dots ., dashes - and spaces (one between characters, three between words) and
return those as a string. Note that some extra 0's may naturally occur at the beginning and the end
of a message, make sure to ignore them. Also if you have trouble discerning if the particular
sequence of 1's is a dot or a dash, assume it's a dot.
2. Function decodeMorse(morseCode), that would take the output of the previous function and return
a human-readable string.

NOTE: For coding purposes you have to use ASCII characters . and -, not Unicode characters.

The Morse code table is preloaded for you (see the solution setup, to get its identifier in your
language).


Eg:
  morseCodes(".--") //to access the morse translation of ".--"
All the test strings would be valid to the point that they could be reliably decoded as described
above, so you may skip checking for errors and exceptions, just do your best in figuring out what
the message is!

Good luck!

After you master this kata, you may try to Decode the Morse code, for real.
 */


fn main() {
    println!("{}", decode_morse(&decode_bits("1100110011001100000011000000111111001100111111001111110000000000000011001111110011111100111111000000110011001111110000001111110011001100000011")));

    println!("{}", decode_morse(&decode_bits("1")));
    println!("{}", decode_morse(&decode_bits("101")));
    println!("{}", decode_morse(&decode_bits("10001")));
    println!("{}", decode_morse(&decode_bits("10111")));
    println!("{}", decode_morse(&decode_bits("1110111")));

    println!("{}", decode_morse(&decode_bits("01110")));

    println!("{}", decode_morse(&decode_bits("111")));
    println!("{}", decode_morse(&decode_bits("1111111")));
    println!("{}", decode_morse(&decode_bits("110011")));
    println!("{}", decode_morse(&decode_bits("111000111")));
    println!("{}", decode_morse(&decode_bits("111110000011111")));
    println!("{}", decode_morse(&decode_bits("11111100111111")));
}

use std::cmp::min;
// mod preloaded;
// use preloaded::MORSE_CODE;
// MORSE_CODE is `HashMap<String, String>`. e.g. ".-" -> "A".
use std::collections::HashMap;



pub fn decode_bits(encoded: &str) -> String {
    println!("bits {encoded}");
    let mut res = String::new();
    let mut ones  = 0;
    let mut zeroes = 0;
    let mut prev = '_';
    let encoded = encoded.trim_end_matches('0').trim_start_matches('0');

    let period = min(encoded.split('1').filter(|s| !s.is_empty()).map(|s| s.chars().count()).min().unwrap_or(100),
                     encoded.split('0').filter(|s| !s.is_empty()).map(|s| s.chars().count()).min().unwrap_or(100)) as i32;
    println!("period {period}");

    let push_char = |ones: &mut i32, zeroes: &mut i32, res: &mut String| {
        if *ones != 0 {
            if *ones == period { res.push('.'); }
            else if *ones == 3* period { res.push('-'); }
            else { panic!("Wrong amount of 1"); }
            *ones = 0;
        } else {
            if *zeroes  == period {}
            else if *zeroes == 3 * period { res.push(' '); }
            else if *zeroes == 7 * period { res.push_str("   "); }
            else { panic!("Wrong amount of 0"); }
            *zeroes = 0;
        }
    };
    for c in encoded.chars() {
        if c != prev && prev != '_' {
            push_char(&mut ones, &mut zeroes, &mut res);
        }
        match c {
            '0' => zeroes += 1,
            '1' => ones += 1,
            _ => panic!("Unexpected character."),
        };
        prev = c;
    }
    push_char(&mut ones, &mut zeroes, &mut res);
    res
}


fn decode_morse(encoded: &str) -> String {
    let mut MORSE_CODE: HashMap<String, String> = HashMap::new();
    MORSE_CODE.insert("....".to_string(), "H".to_string());
    MORSE_CODE.insert(".".to_string(), "E".to_string());
    MORSE_CODE.insert("-.--".to_string(), "Y".to_string());
    MORSE_CODE.insert(".---".to_string(), "J".to_string());
    MORSE_CODE.insert("..-".to_string(), "U".to_string());
    MORSE_CODE.insert("-..".to_string(), "D".to_string());

    encoded.trim().split("   ").map(|word| {
        word.split(' ').map(|letter| MORSE_CODE.get(letter).unwrap_or(&"".to_string()).to_owned()).collect::<Vec<String>>()
            .join("")
    }).collect::<Vec<String>>().join(" ")
}

#[test]
fn examples() {
    assert_eq!(decode_morse(&decode_bits("1100110011001100000011000000111111001100111111001111110000000000000011001111110011111100111111000000110011001111110000001111110011001100000011")), "HEY JUDE".to_string());
}