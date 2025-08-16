// 二分探索 復習問題
mod binary_search_practice;
use binary_search_practice::*;

fn main() {
    println!("=== 二分探索 復習問題 実行結果 ===\n");
    
    // 問題1: 最も近い値を探す
    println!("【問題1: 最も近い値を探す】");
    let arr1 = vec![1, 2, 3, 4, 5, 6, 7, 9, 10, 11];
    println!("配列: {:?}", arr1);
    
    let test_cases = vec![6, 8, 0, 12];
    for target in test_cases {
        if let Some(idx) = find_closest(&arr1, target) {
            println!("target={} → index {} (値 {})", target, idx, arr1[idx]);
        }
    }
    
    // 問題2: 山型配列のピーク
    println!("\n【問題2: 山型配列のピーク】");
    let mountains = vec![
        vec![1, 3, 5, 7, 6, 4, 2],
        vec![1, 2, 3, 4, 5],
        vec![5, 4, 3, 2, 1],
    ];
    
    for mountain in mountains {
        print!("配列 {:?} → ", mountain);
        if let Some(idx) = find_peak(&mountain) {
            println!("index {} (値 {})", idx, mountain[idx]);
        } else {
            println!("None");
        }
    }
    
    // 問題3: 条件を満たす最小値
    println!("\n【問題3: 条件を満たす最小値】");
    
    // x^2 >= 100
    let result = binary_search_condition(1, 100, |x| x * x >= 100);
    println!("x^2 >= 100 を満たす最小の正整数: {:?}", result);
    if let Some(x) = result {
        println!("  検証: {}^2 = {}", x, x * x);
    }
    
    // x^3 >= 1000
    let result = binary_search_condition(1, 100, |x| x * x * x >= 1000);
    println!("x^3 >= 1000 を満たす最小の正整数: {:?}", result);
    if let Some(x) = result {
        println!("  検証: {}^3 = {}", x, x * x * x);
    }
    
    // 2^n >= 1000
    let result = binary_search_condition(1, 20, |n| {
        2_i32.pow(n as u32) >= 1000
    });
    println!("2^n >= 1000 を満たす最小のn: {:?}", result);
    if let Some(n) = result {
        println!("  検証: 2^{} = {}", n, 2_i32.pow(n as u32));
    }
    
    // 問題4: 行列での探索
    println!("\n【問題4: 行列での探索】");
    let matrix = vec![
        vec![1,  4,  7,  11],
        vec![2,  5,  8,  12],
        vec![3,  6,  9,  16],
    ];
    
    println!("行列:");
    for row in &matrix {
        println!("  {:?}", row);
    }
    
    let targets = vec![5, 13, 1, 16];
    for target in targets {
        let result = search_matrix(&matrix, target);
        println!("target={}: {}", target, result);
    }
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
    let mut left = 0;
    let mut right = arr.len();
    if right == 0 { 
        return None 
    };

    while left < right {
        let mid = left + (right - left) / 2;
        if mid == 0 {
            return Some(0)
        }
        
        let left_hand_neighbor = mid - 1;
        if arr[left_hand_neighbor] < arr[mid] {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    };
    return Some(left - 1)
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
    let mut l = left;
    let mut r = right;
    let mut result = None;
    while l <= r {
        let mid = l + (r -l) / 2;
        if condition(mid) {
            r = mid - 1;
            result = Some(mid);
        } else {
            l = mid + 1;
        }
    };

    return result;
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
    let mut i: usize = 0;
    if matrix.len() == 0 {
        return false
    };
    while i < matrix.len() {
        if matrix[i].len() == 0 {
            continue;
        }
        let mut left = 0;
        let mut right = matrix[i].len() - 1;
        while left <= right {
            let mid = left + (right -left) / 2;
            let vals = &matrix[i];
            if vals[mid] == target {
                return true
            } else if vals[mid] > target {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        i = i + 1;
    };
    return false;
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