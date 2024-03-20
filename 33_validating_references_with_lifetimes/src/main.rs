fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);

    println!("The longest string is {}", result);

    // with scope

    let string1 = String::from("this string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(&string1.as_str(), &string2.as_str());

        println!("The longest string is {}", result);
    }

    // with structs

    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    impl<'a> ImportantExcerpt<'a> {
        fn level(&self) -> i32 {
            3
        }
    }

    let novel = String::from("Stormlight archive. The way of the kings");

    let first_sentence = novel.split('.').next().expect("Could not find '.'");

    let i = ImportantExcerpt {
        part: first_sentence,
    };

    let s: &'static str = "I have a static lifetime.";
}
