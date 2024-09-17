use harfbuzz_wasm::{debug, Font, Glyph, GlyphBuffer};
use wasm_bindgen::prelude::*;
use meval;

pub mod llama2;
type Ty = f32;

fn inplace_softmax(x: &mut [Ty]) {
    let max_val = x.iter().fold(Ty::NAN, |acc, &v| v.max(acc));
    let mut denom = 0 as Ty;
    for v in x.iter_mut() {
        *v = (*v - max_val).exp();
        denom += *v;
    }

    x.iter_mut().for_each(|v| *v /= denom);
}

fn cdf_sample(probs: &[Ty]) -> usize {
    // getrandom
    // let mut small_rng = SmallRng::from_entropy();
    let mut buf = [0u8; 32];
    let result = getrandom::getrandom(&mut buf);
    let r = buf[0] as Ty;
    let mut cdf = 0 as Ty;
    for (idx, p) in probs.iter().enumerate() {
        cdf += *p;
        if r < cdf {
            return idx;
        }
    }
    probs.len() - 1
}

fn next_n_words(s: &str, seq_len: usize) -> String {
    let config = llama2::Config::from_file();
    let mut state = llama2::ExecutionState::<Vec<f32>>::init(&config);
    let vocab = llama2::Vocab::from_file(config.vocab_size);
    let mut weights = llama2::LlamaWeights::load_weights(&config);
    // Token 1 is the starting token. TODO: Use the user's input as starting tokens
    // instead. This is totally possible using e.g. the sentencepiece crate to
    // encode tokens, but it does unfortunately seem to be hard to use with Wasm,
    // and a custom encoder owuld be better.
    let mut tokens = vec![1];
    let mut pos = 1;
    let mut token = 1;
    for token_us in tokens.as_slice() {
        token = *token_us;
        llama2::LamaExecuter::step(&mut weights, token, pos, &config, &mut state);
        pos += 1;
    }

    let temperature = 0.5 as Ty;
    let mut probs = vec![0 as Ty; config.vocab_size];
    let initial_tokens = tokens.len();
    tokens = vec![];
    // Here we're just running with temperature 0, so we get deterministic outputs. Some other fun
    // could be to allow for some randomness by using a letter, e.g. "?" to say "I didn't like
    // that last token, please give me another one". Or a way of specifying a seed, which could
    // also just be done in the input text.
    while pos < seq_len + initial_tokens {
        let next =if temperature == 0 as Ty {
            state
                .logits
                .iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.total_cmp(b))
                .map(|(index, _)| index)
                .unwrap()
        } else {
            state
                .logits
                .iter()
                .zip(probs.iter_mut())
                .for_each(|(logit, p)| *p = logit / temperature);
            inplace_softmax(&mut probs);
            cdf_sample(&probs)
        };
        tokens.push(next);
        token = next;
        llama2::LamaExecuter::step(&mut weights, token, pos, &config, &mut state);
        pos += 1;
    }

    let result = tokens
        .into_iter()
        .map(|x| vocab.get_token(x).to_string())
        .collect::<Vec<_>>()
        .join("");
    result.replace("\n", " ")
}

#[wasm_bindgen]
pub fn shape(
    _shape_plan: u32,
    font_ref: u32,
    buf_ref: u32,
    _features: u32,
    _num_features: u32,
) -> i32 {
    let font = Font::from_ref(font_ref);
    let mut buffer = GlyphBuffer::from_ref(buf_ref);
    // Get buffer as string
    let buf_u8: Vec<u8> = buffer.glyphs.iter().map(|g| g.codepoint as u8).collect();
    let str_buf = String::from_utf8_lossy(&buf_u8);

    // Here's a hardcoded assumption that the story we want to build starts with "Once upon a time";
    // cf. the comment above, just encoding actual text we get into tokens instead, we get
    // text generation for any string. This is fine enough for a demo though.
    let res_str = if str_buf.starts_with(
        "Once upon a time.",
    ) {
        let count = str_buf.chars().filter(|c| *c == '.').count() as usize;
        let s = format!("{}", next_n_words(&str_buf, count + 5 - 70));
        debug(&s);
        s
    } else if str_buf.ends_with("=") {
        let curr = str_buf.to_string();
        let mut exp = curr.clone();
        exp.truncate(exp.len() - 1);
        let res = meval::eval_str(exp).unwrap();
        debug(&(curr.clone() + res.to_string().as_str()));
        format!("{}", str_buf)
        .replace(curr.as_str(), res.to_string().as_str())
    } else {
        format!("{}", str_buf)
        .replace("Open", "PoC")
        .replace("y", "ў")
    };

    buffer.glyphs = res_str
        .chars()
        .enumerate()
        .map(|(ix, x)| Glyph {
            codepoint: x as u32,
            flags: 0,
            x_advance: 0,
            y_advance: 0,
            cluster: ix as u32,
            x_offset: 0,
            y_offset: 0,
        })
        .collect();
    
    for item in buffer.glyphs.iter_mut() {
        // Map character to glyph
        item.codepoint = font.get_glyph(item.codepoint, 0);
        // Set advance width
        item.x_advance = font.get_glyph_h_advance(item.codepoint);
    }
    // Buffer is written back to HB on drop
    1
}