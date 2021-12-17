// John has invited some friends. His list is:
//
// s = "Fred:Corwill;Wilfred:Corwill;Barney:Tornbull;Betty:Tornbull;Bjon:Tornbull;Raphael:Corwill;Alfred:Corwill";
// Could you make a program that
//
// makes this string uppercase
// gives it sorted in alphabetical order by last name.
// When the last names are the same, sort them by first name. Last name and first name of a guest come in the result between parentheses separated by a comma.
//
// So the result of function meeting(s) will be:
//
// "(CORWILL, ALFRED)(CORWILL, FRED)(CORWILL, RAPHAEL)(CORWILL, WILFRED)(TORNBULL, BARNEY)(TORNBULL, BETTY)(TORNBULL, BJON)"
// It can happen that in two distinct families with the same family name two people have the same first name too.
//
// Notes
// You can see another examples in the "Sample tests".

use std::cmp::Ordering;

#[derive(Eq)]
struct Name {
    first: String,
    last: String
}

impl PartialOrd for Name {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }

    fn gt(&self, other: &Self) -> bool {
        if self.last == other.last {
            return self.first > other.first;
        }
        self.last > other.last
    }
}

impl Ord for Name {
    fn cmp(&self, other: &Self) -> Ordering {
        if self == other {
            return Ordering::Equal;
        }
        if self.gt(other) {
            return Ordering::Greater;
        }
        Ordering::Less
    }
}

impl PartialEq for Name {
    fn eq(&self, other: &Self) -> bool {
        self.first == other.first && self.last == other.last
    }
}

fn meeting(s: &str) -> String {
    let mut names: Vec<Name> = s.split(';')
        .collect::<Vec<&str>>()
        .into_iter().map(|v| {
            let words: Vec<&str> = v.split(':').collect();
            Name {
                last: words[1].to_string().to_uppercase(),
                first: words[0].to_string().to_uppercase()
            }
        }).collect();
    names.sort();
    let names: Vec<String> = names.into_iter().map(|n| {
        format!("({}, {})", n.last, n.first)
    }).collect();
    names.join("")
}


fn main() {
    let n1 = Name { first: "ARNO".to_string(), last: "BANN".to_string() };
    let n2 = Name { first: "ARNO".to_string(), last: "ANN".to_string() };
    println!("{} {}", n1 > n2, n2 > n1);

    println!("{}", meeting("Alexis:Wahl;John:Bell;Victoria:Schwarz;Abba:Dorny;Grace:Meta;BAnn:Arno;Ann:Arno;Madison:STAN;Alex:Cornwell;Lewis:Kern;Megan:Stan;Alex:Korn"));
}

#[cfg(test)]

mod tests {
    use super::*;

    fn dotest(s: &str, exp: &str) -> () {
        let ans = meeting(s);
        assert_eq!(ans, exp);
    }

    #[test]
    fn basic_tests() {
        dotest("Alexis:Wahl;John:Bell;Victoria:Schwarz;Abba:Dorny;Grace:Meta;Ann:Arno;Madison:STAN;Alex:Cornwell;Lewis:Kern;Megan:Stan;Alex:Korn",
               "(ARNO, ANN)(BELL, JOHN)(CORNWELL, ALEX)(DORNY, ABBA)(KERN, LEWIS)(KORN, ALEX)(META, GRACE)(SCHWARZ, VICTORIA)(STAN, MADISON)(STAN, MEGAN)(WAHL, ALEXIS)");
        dotest("John:Gates;Michael:Wahl;Megan:Bell;Paul:Dorries;James:Dorny;Lewis:Steve;Alex:Meta;Elizabeth:Russel;Anna:Korn;Ann:Kern;Amber:Cornwell",
               "(BELL, MEGAN)(CORNWELL, AMBER)(DORNY, JAMES)(DORRIES, PAUL)(GATES, JOHN)(KERN, ANN)(KORN, ANNA)(META, ALEX)(RUSSEL, ELIZABETH)(STEVE, LEWIS)(WAHL, MICHAEL)");
        dotest("Alex:Arno;Alissa:Cornwell;Sarah:Bell;Andrew:Dorries;Ann:Kern;Haley:Arno;Paul:Dorny;Madison:Kern",
               "(ARNO, ALEX)(ARNO, HALEY)(BELL, SARAH)(CORNWELL, ALISSA)(DORNY, PAUL)(DORRIES, ANDREW)(KERN, ANN)(KERN, MADISON)");

    }
}
