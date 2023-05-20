use regex::Regex;

fn main() {
    task_04();
}

fn task_01() {
    println!("Dates:");
    let date_regex = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
    let text = "On 2023-04-28, the company announced its financial results for the first quarter ending on 2023-03-31. The revenue for the quarter was $10 million, which was higher than the previous year's revenue of $8 million on the same period in 2022-03-31. The company's CEO, John Smith, stated that the increase in revenue was due to the successful launch of their new product on 2023-02-15. The company plans to continue its growth strategy and expects to achieve revenue of $50 million by the end of the year, 2023-12-31.";

    for captures in date_regex.captures_iter(text) {
        println!("{}-{}-{}",
            captures.get(1).unwrap().as_str(),
            captures.get(2).unwrap().as_str(),
            captures.get(3).unwrap().as_str()
        );
    }
    println!(" ");

    println!("Email:");
    let email_regex = Regex::new(r"([A-Za-z0-9])@([A-Za-z0-9]+\.)+[\w]{2,4}").unwrap();
    let email = "example@test.co.uk";
    println!("{}: {}", email, email_regex.is_match(email));
    println!(" ");

    println!("Phone:");
    let phone_regex = Regex::new(r"\(([0-9]{3})\) ([0-9]{3})-([0-9]{4})").unwrap();
    let phone = "(123) 432-9811";
    println!("{}: {}", phone, phone_regex.is_match(phone));
}

fn task_02() {
    println!("Lowercase:");
    let lowercase_regex = Regex::new(r"^[a-z\s]+$").unwrap();
    let lowercase = "hello it is me i am a lowercase kinda guy look at me";
    println!("{}: {}", lowercase, lowercase_regex.is_match(lowercase));
    println!(" ");

    println!("at least one digit:");
    let digit_regex = Regex::new(r"[0-9]+").unwrap();
    let digit_text = "this text has 1 digit in it. Just kidding there's 2";
    println!("{}: {}", digit_text, digit_regex.is_match(digit_text));

    println!("two digits and a space:");
    let two_digits_regex = Regex::new(r"[0-9]{2,}[\s]").unwrap();
    let two_digits_text = "There's 001 digits in this text 002";
    println!("{}: {}", two_digits_text, two_digits_regex.is_match(two_digits_text));
    println!(" ");

    println!("Cat:");
    let cat_regex = Regex::new(r"(\b(cats?)\b)+").unwrap();
    let cat_text = "internet memes often feature a depiction of a cat";
    println!("{}: {}", cat_text, cat_regex.is_match(cat_text));
}

fn task_03() {
    println!("consecutive vowels:");
    let vowels_regex = Regex::new(r"[AEIOUYaeiouy]{3}").unwrap();
    let vowels_text = "sometimes i just wanna go aaa";
    println!("{}: {}", vowels_text, vowels_regex.is_match(vowels_text));
    println!("");

    println!("");
}

fn task_04() {
    println!("starts with http:// or https://");
    let http_regex = Regex::new(r"^(http\:\/\/|https\:\/\/)").unwrap();
    let http_text = "https://www.dumbshit.ninja";
    println!("{}: {}", http_text, http_regex.is_match(http_text));
    println!("");

    println!("end with file extension");
    let file_regex = Regex::new(r"\.([^\.]{3})$").unwrap();
    let file_text = "tax_evasion_document.pdf";
    println!("{}: {}", file_text, file_regex.is_match(file_text));
}
