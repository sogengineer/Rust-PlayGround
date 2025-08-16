// 二分探索 復習問題集

use std::cmp::Ordering;

// ==========================================
// 復習問題1: 最も近い値を探す
// ==========================================
// ソート済み配列から、targetに最も近い値のインデックスを返す
// 例: [1, 3, 5, 7, 9], target=6 → index 2 (値5が最も近い)
pub fn find_closest(arr: &[i32], target: i32) -> Option<usize> {
    // TODO: 実装してください
    // ヒント:
    // - 配列が空の場合はNone
    // - 二分探索で挿入位置を見つける
    // - 前後の値を比較して最も近い方を選ぶ
    // - 絶対値の差で判定: |arr[i] - target|
    if arr.len() == 0 {
        return None;
    }
    let mut left = 0;
    let mut right = arr.len() - 1;
    let mut mid = 0;
    let mut _value: usize = 0;
    fn compare(n: i32, m: i32) -> i32{
        if n - m < 0 {
            m - n
        } else {
            n - m
        }
    }
    while left <= right {
        mid = left + (right - left) / 2;
        if arr[mid] == target {
            return Some(mid)
        } 
        if mid > 0 && mid < arr.len() - 1 {
            let left_side_val = arr[mid - 1];
            let right_side_val = arr[mid + 1];
            println!("left_side_val: {}", left_side_val);
            println!("right_side_val: {}", right_side_val);
        }
        break;
    }
    Some(mid) // 仮の返り値
}

// ==========================================
// 復習問題2: 山型配列のピーク
// ==========================================
// 山型配列（最初は増加、ある点から減少）のピーク要素のインデックスを返す
// 例: [1, 3, 5, 7, 6, 4, 2] → index 3 (値7がピーク)
pub fn find_peak(arr: &[i32]) -> Option<usize> {
    // TODO: 実装してください
    // ヒント:
    // - arr[mid] > arr[mid+1] なら、ピークは左側（またはmid）
    // - arr[mid] < arr[mid+1] なら、ピークは右側
    // - 境界チェックに注意
    
    todo!()
}

// ==========================================
// 復習問題3: 2つのソート配列の中央値
// ==========================================
// 2つのソート済み配列から全体の中央値を求める
// 例: [1, 3], [2, 4] → 中央値は2.5
pub fn find_median_sorted_arrays(nums1: &[i32], nums2: &[i32]) -> f64 {
    // TODO: 実装してください（難易度高）
    // ヒント:
    // - 短い配列に対して二分探索
    // - パーティションを作って左右の要素数を調整
    // - 中央値の定義: 要素数が奇数なら中央、偶数なら中央2つの平均
    
    todo!()
}

// ==========================================
// 復習問題4: 条件を満たす最小値
// ==========================================
// 「f(x) = true となる最小のx」を見つける
// 例: x^2 >= 100 を満たす最小の正整数 → 10
pub fn binary_search_condition<F>(left: i32, right: i32, condition: F) -> Option<i32>
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
// 復習問題5: 行列での探索
// ==========================================
// 各行・各列がソートされた2次元配列から値を探す
// 例:
// [1,  4,  7,  11]
// [2,  5,  8,  12]
// [3,  6,  9,  16]
// target=5 → true
pub fn search_matrix(matrix: &[Vec<i32>], target: i32) -> bool {
    // TODO: 実装してください
    // ヒント:
    // - 右上または左下から開始
    // - 現在の値 > target なら左へ
    // - 現在の値 < target なら下へ
    // - O(m + n)で解ける
    
    todo!()
}

// ==========================================
// テスト用のmain関数
// ==========================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_closest() {
        let arr = vec![1, 3, 5, 7, 9];
        assert_eq!(find_closest(&arr, 6), Some(2)); // 5が最も近い
        assert_eq!(find_closest(&arr, 8), Some(3)); // 7と9の中で7が近い
        assert_eq!(find_closest(&arr, 0), Some(0)); // 1が最も近い
        assert_eq!(find_closest(&[], 5), None);     // 空配列
    }

    #[test]
    fn test_find_peak() {
        assert_eq!(find_peak(&[1, 3, 5, 7, 6, 4, 2]), Some(3));
        assert_eq!(find_peak(&[1, 2, 3, 4, 5]), Some(4)); // 単調増加
        assert_eq!(find_peak(&[5, 4, 3, 2, 1]), Some(0)); // 単調減少
        assert_eq!(find_peak(&[1, 3, 2]), Some(1));
    }

    #[test]
    fn test_binary_search_condition() {
        // x^2 >= 100 を満たす最小の正整数
        let result = binary_search_condition(1, 100, |x| x * x >= 100);
        assert_eq!(result, Some(10));

        // x^3 >= 1000 を満たす最小の正整数
        let result = binary_search_condition(1, 100, |x| x * x * x >= 1000);
        assert_eq!(result, Some(10));
    }

    #[test]
    fn test_search_matrix() {
        let matrix = vec![
            vec![1,  4,  7,  11],
            vec![2,  5,  8,  12],
            vec![3,  6,  9,  16],
        ];
        assert_eq!(search_matrix(&matrix, 5), true);
        assert_eq!(search_matrix(&matrix, 13), false);
    }
}

// ==========================================
// 実装例（答えを見る前に自分で解いてみよう！）
// ==========================================

/*
// 問題1の実装例
fn find_closest(arr: &[i32], target: i32) -> Option<usize> {
    if arr.is_empty() {
        return None;
    }
    
    let mut left = 0;
    let mut right = arr.len() - 1;
    
    // targetより大きい最初の位置を探す
    while left < right {
        let mid = left + (right - left) / 2;
        if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    
    // leftの前後を比較
    if left > 0 {
        let diff_left = (arr[left - 1] - target).abs();
        let diff_right = if left < arr.len() {
            (arr[left] - target).abs()
        } else {
            i32::MAX
        };
        
        if diff_left <= diff_right {
            Some(left - 1)
        } else {
            Some(left)
        }
    } else {
        Some(0)
    }
}

// 問題2の実装例
fn find_peak(arr: &[i32]) -> Option<usize> {
    if arr.is_empty() {
        return None;
    }
    
    let mut left = 0;
    let mut right = arr.len() - 1;
    
    while left < right {
        let mid = left + (right - left) / 2;
        
        if mid + 1 < arr.len() && arr[mid] < arr[mid + 1] {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    
    Some(left)
}

// 問題4の実装例
fn binary_search_condition<F>(left: i32, right: i32, condition: F) -> Option<i32>
where
    F: Fn(i32) -> bool,
{
    let mut l = left;
    let mut r = right;
    let mut result = None;
    
    while l <= r {
        let mid = l + (r - l) / 2;
        if condition(mid) {
            result = Some(mid);
            r = mid - 1;
        } else {
            l = mid + 1;
        }
    }
    
    result
}

// 問題5の実装例
fn search_matrix(matrix: &[Vec<i32>], target: i32) -> bool {
    if matrix.is_empty() || matrix[0].is_empty() {
        return false;
    }
    
    let mut row = 0;
    let mut col = matrix[0].len() - 1;
    
    loop {
        let current = matrix[row][col];
        
        if current == target {
            return true;
        } else if current > target {
            if col == 0 {
                return false;
            }
            col -= 1;
        } else {
            row += 1;
            if row >= matrix.len() {
                return false;
            }
        }
    }
}
*/