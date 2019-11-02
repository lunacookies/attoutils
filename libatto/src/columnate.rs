fn get_longest_str_len(x: &[&str]) -> usize {
    match x.iter().map(|s| s.len()).max() {
        Some(len) => len,
        None => 0,
    }
}

fn pad_right(s: &mut String, n: usize) {
    s.push_str(&" ".repeat(n));
}

// This should probably be generic over all integers, but since it’s only used here as a helper
// function and const generics aren’t stable as of this writing I think I’ll just leave it.
fn ceil(n: usize, m: usize) -> usize {
    if n % m == 0 {
        n / m
    } else {
        n / m + 1
    }
}

pub fn columnate(items: &[&str], total_row_len: usize, padding: usize) -> String {
    // Early return for trivial cases.
    match items.len() {
        1 => return items[0].to_string(),
        0 => return String::new(),
        _ => (),
    }

    let longest_item_len = get_longest_str_len(items);

    let max_row_items = (total_row_len + padding) / (longest_item_len + padding);
    // We remove one padding at the end to account for not adding padding at the end of each row.
    let max_row_chars = max_row_items * (longest_item_len + padding) - padding;
    let row_nums = ceil(items.len(), max_row_items);

    let mut rows = vec![String::with_capacity(max_row_chars); row_nums];

    let mut col_idx = 0;
    let mut row_idx = 0;

    items.iter().enumerate().for_each(|(item_idx, item)| {
        rows[row_idx].push_str(item);

        let at_last_col = col_idx == max_row_items - 1;
        let at_last_item = item_idx == items.len() - 1;
        let apply_padding = !at_last_col && !at_last_item;

        if apply_padding {
            let padding_len = longest_item_len - item.len() + padding;
            pad_right(&mut rows[row_idx], padding_len);
        }

        col_idx += 1;

        if col_idx == max_row_items {
            col_idx = 0;
            row_idx += 1;
        }
    });

    rows.join("\n")
}

#[cfg(test)]
mod pad_right_tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut s = "Some stuff".to_string();
        pad_right(&mut s, 5);
        assert_eq!("Some stuff     ".to_string(), s);
    }

    #[test]
    fn zero() {
        let mut s = "Some stuff".to_string();
        pad_right(&mut s, 0);
        assert_eq!("Some stuff".to_string(), s);
    }
}

#[cfg(test)]
mod columnate_tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(String::new(), columnate(&[], 80, 8).unwrap());
    }

    #[test]
    fn one() {
        assert_eq!("foo".to_string(), columnate(&["foo"], 80, 8).unwrap());
    }

    #[test]
    fn small_pad() {
        let data = ["reallylong", "short", "reallylong", "medium", "x"];

        assert_eq!(
            "reallylong short      reallylong medium     x",
            columnate(&data, 80, 1).unwrap()
        );

        assert_eq!(
            "reallylongshort     reallylongmedium    x",
            columnate(&data, 80, 0).unwrap()
        );
    }

    #[test]
    fn small() {
        assert_eq!(
            "word             exquisite        medium".to_string(),
            columnate(&["word", "exquisite", "medium"], 80, 8).unwrap()
        );
    }

    #[test]
    fn medium() {
        assert_eq!(
            "\
the          quick        brown        fox          jumps        over
the          lazy         dog",
            columnate(
                &["the", "quick", "brown", "fox", "jumps", "over", "the", "lazy", "dog"],
                80,
                8,
            )
            .unwrap()
        );
    }
}
