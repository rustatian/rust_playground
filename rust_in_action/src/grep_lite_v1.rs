pub fn grep_lite_v1() {
    let search_pattern = "picture";
    let quote = "Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books. What do we seek through millions of pages?";

    for line in quote.lines() {
        if line.contains(search_pattern) {
            println!("{}", line);
        }
    }
}

pub fn grep_lite_v2() {
    let search_term = "picture";
    let quote = "Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books. What do we seek through millions of pages?";
    let mut line_num: i32 = 1;
    for line in quote.lines() {
        if line.contains(search_term) {
            println!("{}: {}", line_num, line);
        }
        line_num += 1;
    }
}

pub fn grep_lite_v3() {
    let search_term = "picture";
    let quote = "Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books. What do we seek through millions of pages?";

    for (idx, line) in quote.lines().enumerate() {
        if line.contains(search_term) {
            let line_num = idx + 1;
            println!("{}: {}", line_num, line);
        }
    }
}

pub fn grep_lite_v4_vector() {
    let context_lines = 2;
    let needle = "oo";
    let haystack = "Every face, every shop,
bedroom window, public-house, and
dark square is a picture
feverishly turned--in search of what?
It is the same with books.
What do we seek
through millions of pages?";

    let mut tags: Vec<usize> = Vec::new();
    let mut ctx: Vec<Vec<(usize, String)>> = Vec::new();

    for (i, line) in haystack.lines().enumerate() {
        if line.contains(needle) {
            tags.push(1);

            let v: Vec<(usize, String)> = Vec::with_capacity(2 * context_lines + 1);
            ctx.push(v);
        }
    }

    if tags.is_empty() {
        return;
    }

    for (i, line) in haystack.lines().enumerate() {
        for (j, tag) in tags.iter().enumerate() {
            let lower_bound = tag.saturating_sub(context_lines);
            let upper_bound = tag + context_lines;

            if (i >= lower_bound) && (i <= upper_bound) {
                let line_as_string = String::from(line);
                let local_ctx = (i, line_as_string);
                ctx[j].push(local_ctx);
            }
        }
    }

    for local_ctx in ctx.iter() {
        for &(i, ref line) in local_ctx.iter() {
            let line_num = i + 1;
            println!("{}: {}", line_num, line);
        }
    }
}
