struct User {
    name: String,
    email: String,
}

struct Point {
    x: String,
    y: String,
}

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

fn main() {
    move_basic();
    move_into_function();
    return_ownership();
    move_in_collection();
    clone_to_keep_ownership();
    move_in_closure();
    move_in_struct();
    partial_move();
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

// ── 1. 基本的な移動 ─────────────────────────────────────────────────────────
fn move_basic() {
    println!("=== 1. 基本的な移動 ===");

    let s1 = String::from("hello");
    let s2 = s1; // s1 の所有権が s2 へ移動
    // println!("{}", s1); // コンパイルエラー: s1 はもう使えない
    println!("s2 = {}", s2);

    // Copy トレイトを実装した型 (i32 など) は移動ではなくコピー
    let n1: i32 = 42;
    let n2 = n1; // コピー
    println!("n1 = {}, n2 = {}", n1, n2); // 両方使える
}

// ── 2. 関数への移動 ─────────────────────────────────────────────────────────
fn move_into_function() {
    println!("\n=== 2. 関数への移動 ===");

    let s = String::from("world");
    takes_ownership(s); // s の所有権が関数内へ移動
    // println!("{}", s); // コンパイルエラー: s はもう使えない

    let n = 5;
    makes_copy(n); // i32 は Copy なので n はそのまま使える
    println!("n はまだ使える: {}", n);
}

fn takes_ownership(s: String) {
    println!("関数が受け取った: {}", s);
} // s はここでドロップされる

fn makes_copy(n: i32) {
    println!("コピーを受け取った: {}", n);
}

// ── 3. 所有権を返す ─────────────────────────────────────────────────────────
fn return_ownership() {
    println!("\n=== 3. 所有権を返す ===");

    let s1 = gives_ownership(); // 関数から所有権を受け取る
    println!("s1 = {}", s1);

    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2); // s2 を渡し、s3 として受け取り直す
    // println!("{}", s2); // コンパイルエラー: s2 は移動済み
    println!("s3 = {}", s3);
}

fn gives_ownership() -> String {
    String::from("from function") // 呼び出し元へ所有権が移動
}

fn takes_and_gives_back(s: String) -> String {
    s // そのまま返すことで所有権を呼び出し元へ戻す
}

// ── 4. コレクション内の移動 ──────────────────────────────────────────────────
fn move_in_collection() {
    println!("\n=== 4. コレクション内の移動 ===");

    let v1 = vec![String::from("a"), String::from("b"), String::from("c")];
    // into_iter() で vec の所有権ごと消費しながら要素を取り出す
    for s in v1.into_iter() {
        println!("取り出した: {}", s);
    }
    // println!("{:?}", v1); // コンパイルエラー: v は消費済み
    println!("v1 はもう使えない");

    let v2 = vec![String::from("a"), String::from("b"), String::from("c")];
    // iter() で vecを借用
    for s in v2.iter() {
        println!("借用した: {}", s);
    }
    println!("v2 はまだある: {:?}", v2);

    let mut v3 = vec![String::from("a"), String::from("b"), String::from("c")];
    // iter_mut() で vecを可変借用
    for s in v3.iter_mut() {
        println!("借用した: {}", s);
    }
    println!("v3 はまだある: {:?}", v3);

    // 参照で借用すれば v は残る
    let v9 = vec![String::from("x"), String::from("y")];
    for s in &v9 {
        println!("借用: {}", s);
    }
    println!("v9 はまだある: {:?}", v9);
}

// ── 5. clone で所有権を保持 ───────────────────────────────────────────────
fn clone_to_keep_ownership() {
    println!("\n=== 5. clone で所有権を保持 ===");

    let s1 = String::from("deep copy");
    let s2 = s1.clone(); // ヒープデータごとコピー
    println!("s1 = {}, s2 = {}", s1, s2); // 両方使える (コストはかかる)
}

// ── 6. クロージャへの移動 ────────────────────────────────────────────────
fn move_in_closure() {
    println!("\n=== 6. クロージャへの移動 ===");

    let s = String::from("closure");

    // move キーワードで s の所有権をクロージャ内へ強制移動
    let print_s = move || println!("クロージャ内: {}", s);
    // println!("{}", s); // コンパイルエラー: s は移動済み
    println!("s はもう使えない");

    print_s();
    print_s(); // クロージャ自体が所有しているので何度でも呼べる

    // お試しFn
    fn call_fn<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
        f(x)
    }
    fn func(a: i32) -> i32 {
        a * 2
    }
    let mut call_count = 0;
    let clos = move |a: i32| -> i32 {
        call_count += 1;
        println!("{}回目の呼び出し{}", call_count, a);
        a * 2
    };
    call_fn(func, 2);

    // お試しFnMut
    fn call_fn_mut<F: FnMut(i32) -> i32>(mut f: F, x: i32) -> i32 {
        f(x)
    }
    call_fn_mut(func, 2);
    call_fn_mut(clos, 2);

    // お試しFnOnce
    fn call_fn_once<F: FnOnce(i32) -> i32>(f: F, x: i32) -> i32 {
        f(x)
    }
    let mut call_count2 = 0;
    let clos2 = move |a: i32| -> i32 {
        call_count2 += 1;
        println!("{}回目の呼び出し{}", call_count2, a);
        a * 2
    };
    call_fn_once(func, 2);
    call_fn_once(clos2, 2);
}

// ── 7. 構造体への移動 ────────────────────────────────────────────────────
fn move_in_struct() {
    println!("\n=== 7. 構造体への移動 ===");

    let name = String::from("Alice");
    let user = User {
        name,
        email: String::from("alice@example.com"),
    }; // name の所有権が User へ移動
    // println!("{}", name); // コンパイルエラー: name は移動済み
    println!("user.name = {}", user.name);
    println!("user.email = {}", user.email);

    // 構造体の更新構文: フィールドを一部だけ変えて新しい値を作る
    let user2 = User {
        name: String::from("Bob"),
        ..user // email の所有権が user から user2 へ移動
    };
    println!("user.name = {}", user.name);
    // println!("{}", user.email); // コンパイルエラー: user.email は移動済み
    println!("user2.name = {}, user2.email = {}", user2.name, user2.email);
}

// ── 8. 部分移動 ──────────────────────────────────────────────────────────
fn partial_move() {
    println!("\n=== 8. 部分移動 ===");

    let point = Point {
        x: String::from("East"),
        y: String::from("North"),
    };

    let x = point.x; // x フィールドだけ所有権を取り出す
    println!("x = {}", x);
    println!("point.y はまだ使える: {}", point.y);
    // println!("{:?}", point.x); // コンパイルエラー: point は部分的に移動済みで全体参照不可
    // println!("{:?}", point); // コンパイルエラー: point は部分的に移動済みで全体参照不可

    #[derive(Debug)]
    struct XY {
        x: Vec<i32>,
        y: Vec<i32>,
    }
    let mut xy = XY {
        x: vec![1, 2, 3],
        y: Vec::new(),
    };

    for elm in xy.x.iter() {
        println!("{:?}", xy);
        println!("{:?}", xy.x);
        xy.y.push(*elm * *elm);
    }

    // let XY { x, y } = &mut xy;
    // for elm in x {
    //     y.push(*elm * *elm);
    // }
    println!("{:?}", xy);
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
    // スコープ外なのでエラー
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copy_type_is_not_consumed() {
        let n1: i32 = 42;
        let n2 = n1;
        assert_eq!(n1, n2); // both still valid after "move"
    }

    #[test]
    fn gives_ownership_returns_expected_string() {
        assert_eq!(gives_ownership(), "from function");
    }

    #[test]
    fn takes_and_gives_back_returns_same_value() {
        let s = String::from("hello");
        assert_eq!(takes_and_gives_back(s), "hello");
    }

    #[test]
    fn clone_produces_independent_copy() {
        let s1 = String::from("deep copy");
        let mut s2 = s1.clone();
        s2.push_str(" modified");
        assert_eq!(s1, "deep copy");
        assert_ne!(s1, s2);
    }

    #[test]
    fn into_iter_consumes_vec_elements_in_order() {
        let v = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let collected: Vec<String> = v.into_iter().collect();
        assert_eq!(collected, ["a", "b", "c"]);
    }

    #[test]
    fn iter_borrow_leaves_vec_intact() {
        let v = vec![String::from("x"), String::from("y")];
        let _borrowed: Vec<&String> = v.iter().collect();
        assert_eq!(v.len(), 2); // v still owned here
    }

    #[test]
    fn move_closure_captures_value_and_is_callable_multiple_times() {
        let s = String::from("closure");
        let get_s = move || s.clone();
        assert_eq!(get_s(), "closure");
        assert_eq!(get_s(), "closure");
    }

    #[test]
    fn struct_update_syntax_moves_string_field() {
        let user1 = User {
            name: String::from("Alice"),
            email: String::from("alice@example.com"),
        };
        let user2 = User {
            name: String::from("Bob"),
            ..user1
        };
        assert_eq!(user2.name, "Bob");
        assert_eq!(user2.email, "alice@example.com"); // email moved from user1
    }

    #[test]
    fn partial_move_leaves_remaining_field_accessible() {
        let point = Point {
            x: String::from("East"),
            y: String::from("North"),
        };
        let x = point.x;
        assert_eq!(x, "East");
        assert_eq!(point.y, "North"); // y is still valid
    }

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
