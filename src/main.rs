// 二分探索 復習問題
mod binary_search_practice;
use binary_search_practice::*;

fn main() {
    println!("=== 二分探索 復習問題 ===\n");
    
    // 問題1: 最も近い値を探す
    println!("問題1: 最も近い値を探す");
    let arr1 = vec![1,2, 3,4, 5, 6, 7, 9,10, 11];
    println!("{:?}", find_closest(&arr1, 6));
    println!("配列: {:?}", arr1);
    println!("target=6 に最も近い値は？");
    println!("期待される答え: index 2 (値5)\n");
    
    // 問題2: 山型配列のピーク
    println!("問題2: 山型配列のピーク");
    let mountain = vec![1, 3, 5, 7, 6, 4, 2];
    println!("山型配列: {:?}", mountain);
    println!("ピークはどこ？");
    println!("期待される答え: index 3 (値7)\n");
    
    // 問題3: 条件を満たす最小値
    println!("問題3: 条件を満たす最小値");
    println!("x^2 >= 100 を満たす最小の正整数は？");
    println!("期待される答え: 10\n");
    
    // 問題4: 行列での探索
    println!("問題4: ソート済み行列での探索");
    println!("行列:");
    println!("[1,  4,  7,  11]");
    println!("[2,  5,  8,  12]");
    println!("[3,  6,  9,  16]");
    println!("target=5 は存在する？");
    println!("期待される答え: true\n");
    
    // ヒント
    println!("=== ヒント ===");
    println!("1. 最も近い値: lower_boundを使って挿入位置を見つけ、前後を比較");
    println!("2. 山型配列: arr[mid] と arr[mid+1] の大小関係で判断");
    println!("3. 条件を満たす最小値: 「答えを二分探索する」パターン");
    println!("4. 行列探索: 右上または左下から始めて、値に応じて移動");
    
    println!("\n実装は src/binary_search_practice.rs にあります。");
    println!("テストを実行: cargo test");
}

// ==========================================
// 復習問題1: 最も近い値を探す
// ==========================================
fn find_closest(arr: &[i32], target: i32) -> Option<usize> {
    // 空配列チェック（これは良い！）
    if arr.len() == 0 {
        return None;
    }
    
    // 絶対値の差を計算する関数（これも良い！）
    fn result_cal(n: i32, m: i32) -> i32 {
        if n - m < 0 {
            m - n
        } else {
            n - m
        }
    }
    
    // 元の実装（コメントアウト）
    // let mut left = 0;
    // let mut right = arr.len() - 1;
    // let mut mid = 0;
    // let mut value: usize = 0;
    // while left <= right {
    //     mid = left + (right - left) / 2;
    //     if arr[mid] == target {
    //         return Some(mid)
    //     } 
    //     if mid > 0 && mid < arr.len() - 1 {
    //         let a = &mid - 1;
    //         let b = &mid + 1;
    //         let left_side_val = result_cal(target, arr[a]);
    //         let right_side_val = result_cal(target,arr[b]);
    //         if left_side_val == right_side_val {
    //             return Some(mid);
    //         } else if left_side_val < right_side_val {
    //             left = mid + 1;
    //         } else {
    //             right = mid - 1;
    //         }
    //     }
    //     // ↓問題点: ループが1回で終了してしまう
    //     // break;
    // }
    // Some(mid)
    
    // ===== 正しい実装 =====
    // Step 1: targetが挿入されるべき位置を見つける（lower_bound）
    let mut left = 0;
    let mut right = arr.len();
    
    while left < right {
        let mid = left + (right - left) / 2;
        if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    // ここで left = targetを挿入すべき位置
    
    // Step 2: 前後の値を比較して最も近い値を見つける
    let mut closest_idx = 0;
    let mut min_diff = i32::MAX;
    
    // 候補1: left位置の要素（存在する場合）
    if left < arr.len() {
        let diff = result_cal(arr[left], target);
        if diff < min_diff {
            min_diff = diff;
            closest_idx = left;
        }
    }
    
    // 候補2: left-1位置の要素（存在する場合）
    if left > 0 {
        let diff = result_cal(arr[left - 1], target);
        // 同じ差の場合は左側（小さい値）を優先
        if diff <= min_diff {
            closest_idx = left - 1;
        }
    }
    
    Some(closest_idx)
}

// ==========================================
// 復習問題2: 山型配列のピーク
// ==========================================
fn find_peak(arr: &[i32]) -> Option<usize> {
    // TODO: 実装してください
    // ヒント:
    // - arr[mid] > arr[mid+1] なら、ピークは左側（またはmid）
    // - arr[mid] < arr[mid+1] なら、ピークは右側
    // - 境界チェックに注意
    
    todo!()
}

// ==========================================
// 復習問題3: 条件を満たす最小値
// ==========================================
fn binary_search_condition<F>(left: i32, right: i32, condition: F) -> Option<i32>
where
    F: Fn(i32) -> bool,
{
    // TODO: 実装してください
    // ヒント:
    // - 条件を満たす最小値を探す（lower_boundの一般化）
    // - condition(mid)がtrueなら、答えはmid以下
    // - falseなら、答えはmidより大きい
    
    todo!()
}

// ==========================================
// 復習問題4: 行列での探索
// ==========================================
fn search_matrix(matrix: &[Vec<i32>], target: i32) -> bool {
    // TODO: 実装してください
    // ヒント:
    // - 右上または左下から開始
    // - 現在の値 > target なら左へ
    // - 現在の値 < target なら下へ
    // - O(m + n)で解ける
    
    todo!()
}

/*
学習のポイント:

1. 最も近い値を探す
   - 完全一致しない場合の処理
   - 前後の値との距離を比較

2. 山型配列のピーク
   - 単調性が変わる点を探す
   - 傾きの変化を検出

3. 条件を満たす最小値
   - 「答えを二分探索」の汎用パターン
   - 条件関数を引数に取る

4. 2次元での探索
   - 行と列の両方がソートされている性質を活用
   - O(m + n)で解ける効率的なアルゴリズム

これらは実際の開発でもよく使うパターンです！
*/