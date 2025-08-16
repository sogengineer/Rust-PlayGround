// Rustアルゴリズム学習
// 第1回: 二分探索
#[warn(unused_imports)]
use std::cmp::Ordering;

// 問題1: 基本的な二分探索
// ソート済み配列から特定の値を探す
fn binary_search_basic(arr: &[i32], target: i32) -> Option<usize> {
    // TODO: 二分探索を実装
    // ヒント:
    // - left, rightの2つのポインタを使う
    // - 中央値 mid = (left + right) / 2
    // - arr[mid]とtargetを比較して範囲を狭める
    // - 見つかったらSome(index)、見つからなかったらNone
    let mut left = 0;
    let mut right = arr.len();
    while left < right {
        let mid = left + (right - left) / 2;
        if target == arr[mid] {
            return Some(mid)
        } else if arr[mid] < target {  // arr[mid]がtargetより小さい
            left = mid + 1;             // → 右側を探す（leftを進める）
        } else {                        // arr[mid]がtargetより大きい
            right = mid;                // → 左側を探す（rightを狭める）
        }
    }
    None
}

// 問題2: lower_bound（挿入位置を探す）
// target以上の値が現れる最初の位置を返す
fn lower_bound(arr: &[i32], target: i32) -> usize {
    // TODO: lower_boundを実装
    // ヒント:
    // - 二分探索の変形
    // - 条件を満たす最小のインデックスを探す
    // - 全ての要素がtargetより小さい場合はarr.len()を返す
    
    // 元の実装（コメントアウト）
    // let mut left = 0;
    // let mut right = arr.len();
    // let mut mid = left + (right - left) / 2;
    // let mut is_minus_idx = false;
    // while left < right {
    //     if target == arr[mid] {
    //         mid = mid - 1;
    //         is_minus_idx = true;
    //         continue;
    //     } else if arr[mid] < target {
    //         if is_minus_idx { break; } // arr[mid]がtargetより小さい
    //         left = mid + 1;             // → 右側を探す（leftを進める）
    //     } else {                        // arr[mid]がtargetより大きい
    //         right = mid;                // → 左側を探す（rightを狭める）
    //     }
    //     mid = left + (right - left) / 2;
    // }
    // return mid + 1
    
    // シンプルな実装
    let mut left = 0;
    let mut right = arr.len();
    
    while left < right {
        let mid = left + (right - left) / 2;
        if arr[mid] < target {
            left = mid + 1;  // targetはもっと右にある
        } else {
            right = mid;     // arr[mid] >= target なので、midも候補に含める
        }
    }
    left  // leftが「target以上の最初の位置」
}

// 問題3: upper_bound
// targetより大きい値が現れる最初の位置を返す
fn upper_bound(arr: &[i32], target: i32) -> usize {
    // TODO: upper_boundを実装
    // ヒント:
    // - lower_boundと似ているが条件が異なる
    // - arr[mid] <= target の場合はleftを更新
    let mut left: usize = 0;
    let mut right: usize = arr.len();
    while left < right {
        let mid = left + (right - left) / 2;
        if arr[mid] <= target {
            left = mid + 1;
        } else {
            right = mid;
        };
    }
    left
}

// 応用問題: 平方根の計算
// 整数nの平方根を二分探索で求める（小数点以下切り捨て）
fn sqrt_binary_search(n: u64) -> u64 {
    // TODO: 二分探索で平方根を求める
    // ヒント:
    // - 答えの範囲は0からn
    // - mid * mid <= n を満たす最大のmidを探す
    // - オーバーフロー対策: mid <= n / mid
    if n == 0 {
        return 0;
    }
    
    let mut left = 1;  // 0ではなく1から始める（ゼロ除算回避）
    let mut right = n;
    let mut result = 1;  // 最小でも1
    
    while left <= right {  // <= を使う
        let mid = left + (right - left) / 2;
        
        // midが0でないことが保証されている
        if mid <= n / mid {
            result = mid;  // 現在の答えを保存
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    result
}

// 発展問題: 回転ソート配列での探索
// [4,5,6,7,0,1,2]のような回転ソート配列から値を探す
fn search_rotated_array(arr: &[i32], target: i32) -> Option<usize> {
    // TODO: 回転ソート配列での二分探索
    // ヒント:
    // - 配列の半分は必ずソートされている
    // - どちらの半分がソートされているか判定
    // - targetがその範囲内にあるか確認
    let mut left = 0;
    let mut right = arr.len() - 1;
    while left <= right {
        let mid = left + (right - left) / 2;
        if arr[mid] == target {
            return Some(mid)
        }
        if arr[left] < arr[mid] {
            if arr[left] <= target && target < arr[mid] {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        } else {
            if arr[mid] <= target && target < arr[right] {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
    };
    None
}


fn main() {
    println!("=== Rustアルゴリズム学習: 二分探索 ===\n");
    
    // テストケース1: 基本的な二分探索
    let arr1 = vec![1, 3, 5, 7, 9, 11, 13, 15];
    println!("配列: {:?}", arr1);
    
    // 存在する値を探す
    if let Some(index) = binary_search_basic(&arr1, 7) {
        println!("7 は index {} にあります", index);
    }
    
    // 存在しない値を探す
    if binary_search_basic(&arr1, 6).is_none() {
        println!("6 は配列に存在しません");
    }
    
    // Rustの標準ライブラリの使い方
    println!("\n標準ライブラリの例:");
    match arr1.binary_search(&7) {
        Ok(index) => println!("binary_search: 7 found at index {}", index),
        Err(index) => println!("binary_search: 7 would be inserted at index {}", index),
    }
    
    // テストケース2: lower_bound / upper_bound
    let arr2 = vec![1, 2, 2, 2, 3, 4, 5];
    println!("\n配列: {:?}", arr2);
    println!("lower_bound(2) = {}", lower_bound(&arr2, 2));
    println!("upper_bound(2) = {}", upper_bound(&arr2, 2));
    
    // テストケース3: 平方根
    println!("\n平方根の計算:");
    for n in [0, 1, 4, 8, 16, 25, 100, 1000000] {
        println!("sqrt({}) = {}", n, sqrt_binary_search(n));
    }
    
    // テストケース4: 回転ソート配列
    let rotated = vec![4, 5, 6, 7, 0, 1, 2];
    println!("\n回転ソート配列: {:?}", rotated);
    for target in [0, 1, 4, 6, 8] {
        match search_rotated_array(&rotated, target) {
            Some(idx) => println!("{} found at index {}", target, idx),
            None => println!("{} not found", target),
        }
    }
}

/*
学習ポイント:
1. 二分探索の基本的な考え方
   - O(log n)の時間計算量
   - ソート済み配列が前提

2. 境界条件の処理
   - left <= right vs left < right
   - mid の計算でのオーバーフロー対策

3. Rustの特徴
   - Option<T>を使った結果の表現
   - スライス&[T]の使用
   - 標準ライブラリのbinary_search

4. 応用
   - 答えを二分探索する
   - 条件を満たす最小/最大を探す
*/