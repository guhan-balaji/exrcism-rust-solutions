use std::{collections::HashMap, sync::mpsc, thread};

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let (tx, rx) = mpsc::channel::<HashMap<char, usize>>();

    let chunk_size = match input.len() {
        0 => return HashMap::new(),
        len if len % worker_count == 0 => len / worker_count,
        len => (len / worker_count) + 1,
    };

    input.chunks(chunk_size).for_each(|chunk| {
        let s = chunk
            .iter()
            .flat_map(|s| s.chars())
            .filter(|c| !c.is_whitespace() && c.is_alphabetic())
            .map(|c| c.to_ascii_lowercase())
            .collect::<String>();

        let tx = tx.clone();
        thread::spawn(move || {
            let mut lc: HashMap<char, usize> = HashMap::new();
            s.chars().for_each(|c| {
                lc.entry(c).and_modify(|e| *e += 1).or_insert(1);
            });

            tx.send(lc).unwrap();
        });
    });

    // mpsc receiver waits for all transmitters to drop out of scope before
    // the receiver stops iterating.
    // The spawned threads drop the cloned tx's as it goes out of scope.
    // In this code, if tx from the main thread is not dropped, the for loop
    // below waits forever,thereby, blocking the main thread from terminating.
    drop(tx);

    let mut letter_count: HashMap<char, usize> = HashMap::new();
    for lc in rx {
        lc.into_iter().for_each(|(letter, count)| {
            letter_count
                .entry(letter)
                .and_modify(|e| *e += count)
                .or_insert(count);
        })
    }
    letter_count
}
