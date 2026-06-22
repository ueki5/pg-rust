struct Excerpt<'a> {
    part: &'a str, // 参照元データより長く生存できない
}

impl<'a> Excerpt<'a> {
    // 省略規則3: &self があるので戻り値は self の 'a を引き継ぐ
    fn part_with_note(&self, note: &str) -> &str {
        println!("[{}]", note);
        self.part
    }
}

// ── 9. ライフタイム基本 ──────────────────────────────────────────────────────
fn lifetime_basic() {
    println!("\n=== 9. ライフタイム基本 ===");

    // 'a は「戻り値は引数と同じかそれより短い寿命を持つ」という制約
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() >= y.len() { x } else { y }
    }

    let s1 = String::from("long string is long");
    {
        let s2 = String::from("xyz");
        let result = longest(s1.as_str(), s2.as_str());
        println!("longest: {}", result); // result は s2 と同じスコープで使う必要がある
    }

    //     // NG 例: result を s2 より長いスコープで使おうとするとコンパイルエラー
    //     let result;
    //     {
    //         let s2 = String::from("xyz");
    //         result = longest(s1.as_str(), s2.as_str()); // error: s2 does not live long enough
    //     }
    //     println!("{}", result);
}

// // ── 9.1 ライフタイム基本(省略するとコンパイルエラー) ──────────────────────────────────────────────────────
// fn lifetime_basic2() {
//     println!("\n=== 9.1 ライフタイム基本 ===");
//
//     // 'a は「戻り値は引数と同じかそれより短い寿命を持つ」という制約
//     fn longest(x: &str, y: &str) -> &str {
//         if x.len() >= y.len() { x } else { y }
//     }
//
//     let s1 = String::from("long string is long");
//     {
//         let s2 = String::from("xyz");
//         let result = longest(s1.as_str(), s2.as_str());
//         println!("longest: {}", result); // result は s2 と同じスコープで使う必要がある
//     }
// }

// ── 10. 構造体のライフタイム ─────────────────────────────────────────────────
fn lifetime_in_struct() {
    println!("\n=== 10. 構造体のライフタイム ===");

    let novel = String::from("Call me Ishmael. Some years ago...");
    {
        let first_sentence = novel.split('.').next().unwrap();
        // Excerpt は novel より長く生存できない
        let excerpt = Excerpt {
            part: first_sentence,
        };
        // impl のメソッドが省略規則3で &self の 'a を戻り値に引き継ぐ
        println!("part: '{}'", excerpt.part_with_note("第一文"));
    }
    // // スコープ外なのでエラー
    // println!("part: '{}'", excerpt.part_with_note("第一文"));
}

// ── 10.5 構造体のライフタイム ─────────────────────────────────────────────────
fn lifetime_in_struct2() {
    println!("\n=== 10.5 構造体のライフタイム ===");

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence: &str = &novel;
    let excerpt;
    {
        // Excerpt は novel より長く生存できない
        excerpt = Excerpt {
            part: first_sentence,
        };
        // impl のメソッドが省略規則3で &self の 'a を戻り値に引き継ぐ
        println!("part: '{}'", excerpt.part_with_note("第一文"));
    }
    // スコープ内なのでエラーにならない
    println!("part: '{}'", excerpt.part_with_note("第一文"));
}

// ── 11. ライフタイム省略規則 ─────────────────────────────────────────────────
fn lifetime_elision() {
    println!("\n=== 11. ライフタイム省略規則 ===");

    // 規則1: 入力参照ごとに独立したライフタイムが割り当てられる
    // 規則2: 入力ライフタイムが1つだけなら、出力ライフタイムもそれと同じ
    // 規則3: &self / &mut self があれば、出力は self のライフタイムを継承

    // 規則2の適用例: fn first_word<'a>(s: &'a str) -> &'a str と同義
    fn first_word(s: &str) -> &str {
        s.split_whitespace().next().unwrap_or(s)
    }
    println!("first word: '{}'", first_word("hello world"));

    // 規則が適用できず省略不可: 入力が複数でどれを返すか曖昧
    // fn ambiguous(x: &str, y: &str) -> &str { x } // コンパイルエラー
    fn explicit<'a>(x: &'a str, _y: &str) -> &'a str {
        x
    } // 明示が必要
    println!("explicit: '{}'", explicit("first", "second"));
}

// ── 12. 'static ライフタイム ─────────────────────────────────────────────────
fn lifetime_static() {
    println!("\n=== 12. 'static ライフタイム ===");

    // 文字列リテラルはバイナリに埋め込まれるため常に 'static
    let s: &'static str = "I live for the entire program";
    println!("{}", s);

    // T: 'static は「T が参照を含まないか、'static 参照のみ含む型」を要求
    fn store<T: 'static>(val: T) -> Box<T> {
        Box::new(val)
    }
    let boxed = store(String::from("owned")); // String は参照を含まず 'static
    println!("boxed: {}", boxed);
    // let r = String::from("temp");
    // let _ = store(&r); // コンパイルエラー: &String は 'static でない

    // &'static str はあらゆる短命な 'a に暗黙変換できる (共変)
    fn takes_any(s: &str) {
        println!("got: {}", s);
    }
    takes_any("static literal"); // &'static str → &'_ str へ自動変換
}

// ── 13. 複数のライフタイムパラメータ ────────────────────────────────────────
fn lifetime_multiple() {
    println!("\n=== 13. 複数のライフタイムパラメータ ===");

    // 'a と 'b は独立: x から返すので戻り値は 'a に縛られ 'b とは無関係
    fn first_of<'a, 'b>(x: &'a str, _y: &'b str) -> &'a str {
        x
    }

    let s1 = String::from("first");
    let result;
    {
        let s2 = String::from("second"); // 'b: 短命
        result = first_of(s1.as_str(), s2.as_str());
        // s2('b) の寿命がここで終わる
    }
    println!("result: '{}'", result); // s1('a) はまだ生きているのでOK

    // 2つのライフタイムを持つ構造体
    struct TwoRefs<'a, 'b> {
        x: &'a str,
        y: &'b str,
    }
    let a = String::from("alpha");
    let b = String::from("beta");
    let tr = TwoRefs { x: &a, y: &b };
    println!("x='{}', y='{}'", tr.x, tr.y);
}

// ── 14. ライフタイムのサブタイピング ('long: 'short) ─────────────────────────
fn lifetime_subtyping() {
    println!("\n=== 14. ライフタイムのサブタイピング ('long: 'short) ===");

    // 'l: 's は「'l は少なくとも 's の間は生存する」= 長命な参照を短命として扱える
    fn use_long_return_short<'s, 'l: 's>(short: &'s str, long: &'l str) -> &'s str {
        println!("long='{}'", long);
        short
    }

    let long_lived = String::from("I live long");
    {
        let short_lived = String::from("short");
        let result = use_long_return_short(&short_lived, &long_lived);
        println!("result: '{}'", result);
        // result は short_lived の寿命('s)に縛られるのでここまで
    }

    // 共変性: &'static str は &'a str のサブタイプ → 短命参照が必要な場所に渡せる
    let s: &'static str = "static string";
    let shorter: &str = s; // &'static を &'_ に暗黙変換
    println!("downgraded: '{}'", shorter);
}

// ── 15. ジェネリック境界 T: 'a ───────────────────────────────────────────────
fn lifetime_bounds() {
    println!("\n=== 15. ジェネリック境界 T: 'a ===");

    // T: 'a は「T が含む全ての参照が少なくとも 'a の間は生存する」ことを要求する
    fn print_ref<'a, T: std::fmt::Display + 'a>(val: &'a T) {
        println!("val = {}", val);
    }
    print_ref(&42i32);
    print_ref(&String::from("hello"));

    // T: 'static は「T が参照を含まないか、'static 参照のみを含む」型を要求
    fn store_forever<T: 'static + std::fmt::Debug>(val: T) -> Box<T> {
        Box::new(val)
    }
    let b = store_forever(vec![1, 2, 3]);
    println!("stored: {:?}", b);

    // ライフタイム境界付きラッパー構造体: T: 'a は edition 2024 では implied だが明示も有効
    struct Wrapper<'a, T: 'a> {
        inner: &'a T,
    }
    let data = vec![10, 20, 30];
    let w = Wrapper { inner: &data };
    println!("wrapped: {:?}", w.inner);
}

// ── 16. 高ランクトレイト境界 (HRTB) for<'a> ─────────────────────────────────
fn lifetime_hrtb() {
    println!("\n=== 16. 高ランクトレイト境界 (HRTB) for<'a> ===");

    // for<'a> は「任意のライフタイム 'a に対して成立する」という意味
    // F: Fn(&str) -> usize は省略で for<'a> Fn(&'a str) -> usize と同義だが
    // 明示することでクロージャが任意の参照を受け取れることを表明できる
    fn apply<F>(f: F, s: &str) -> usize
    where
        F: for<'a> Fn(&'a str) -> usize,
    {
        f(s)
    }
    println!("len: {}", apply(|s| s.len(), "hello"));
    println!(
        "words: {}",
        apply(|s| s.split_whitespace().count(), "hello world rust")
    );

    // 関数ポインタは自動的に HRTB を持つ
    // fn(&str) -> usize は for<'a> fn(&'a str) -> usize の糖衣構文
    let fp: fn(&str) -> usize = str::len;
    println!("via fn pointer: {}", fp("HRTB"));

    // トレイトオブジェクトと HRTB: 動的ディスパッチでも任意の参照を受け付ける
    let closures: Vec<Box<dyn for<'a> Fn(&'a str) -> usize>> = vec![
        Box::new(|s: &str| s.len()),
        Box::new(|s: &str| s.chars().count()),
    ];
    let word = "Rust";
    for f in &closures {
        println!("result: {}", f(word));
    }
}
fn main() {
    lifetime_basic();
    lifetime_in_struct();
    lifetime_in_struct2();
    lifetime_elision();
    lifetime_static();
    lifetime_multiple();
    lifetime_subtyping();
    lifetime_bounds();
    lifetime_hrtb();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lifetime_longest_returns_longer_string() {
        fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
            if x.len() >= y.len() { x } else { y }
        }
        assert_eq!(longest("long string", "short"), "long string");
        assert_eq!(longest("a", "bb"), "bb");
        assert_eq!(longest("eq", "eq"), "eq");
    }

    #[test]
    fn lifetime_excerpt_holds_reference_to_source() {
        let novel = String::from("Hello. World.");
        let first = novel.split('.').next().unwrap();
        let excerpt = Excerpt { part: first };
        assert_eq!(excerpt.part, "Hello");
    }

    #[test]
    fn lifetime_elision_first_word_extracts_correctly() {
        fn first_word(s: &str) -> &str {
            s.split_whitespace().next().unwrap_or(s)
        }
        assert_eq!(first_word("hello world"), "hello");
        assert_eq!(first_word("single"), "single");
        assert_eq!(first_word(""), "");
    }

    #[test]
    fn lifetime_static_str_coerces_to_shorter_lifetime() {
        let s: &'static str = "static";
        let shorter: &str = s; // &'static → &'_ 暗黙変換
        assert_eq!(shorter, "static");
    }

    #[test]
    fn lifetime_multiple_independent_params_allow_short_second_arg() {
        fn first_of<'a, 'b>(x: &'a str, _y: &'b str) -> &'a str {
            x
        }
        let s1 = String::from("first");
        let result;
        {
            let s2 = String::from("second");
            result = first_of(s1.as_str(), s2.as_str());
        }
        assert_eq!(result, "first"); // s2 はスコープ外だが result('a=s1) は有効
    }

    #[test]
    fn lifetime_hrtb_apply_works_with_closures_and_fn_pointers() {
        fn apply<F>(f: F, s: &str) -> usize
        where
            F: for<'a> Fn(&'a str) -> usize,
        {
            f(s)
        }
        assert_eq!(apply(|s| s.len(), "hello"), 5);
        assert_eq!(apply(str::len, "rust"), 4);
    }
}
