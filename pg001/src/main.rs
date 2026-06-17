struct User {
    name: String,
    email: String,
}

struct Point {
    x: String,
    y: String,
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
}
