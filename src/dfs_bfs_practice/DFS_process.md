# DFS (深さ優先探索) アルゴリズムの組み立てプロセス

## 1. DFSとは？
**深さ優先探索（Depth-First Search）** は、グラフや木構造を探索するアルゴリズムです。  
一つの道を「行き止まりまで突き進んで」から、戻って別の道を探索します。

### 特徴
- **迷路の右手法のような探索**: 一つの道を最後まで進む
- **スタック（Stack）を使用**: 後入先出し（LIFO）でノードを処理
- **再帰でも実装可能**: 関数の呼び出しスタックを利用
- **メモリ効率が良い**: 深さ分のメモリしか使わない

## 2. DFSの基本公式

### 算数レベルの公式
```
深さ(次のノード) = 深さ(現在のノード) + 1
```

### 探索の順序（迷路の例）
```
1. スタート地点から一方向に進めるだけ進む
2. 行き止まりに到達
3. 一つ前に戻る（バックトラック）
4. 別の道があれば、そちらに進む
5. 全ての道を探索するまで繰り返す
```

## 3. DFSアルゴリズムの組み立てステップ

### 実装方法1: スタックを使った反復的実装
```rust
// 必要なデータ構造
let mut stack = Vec::new();             // スタック（後入先出し）
let mut visited = HashSet::new();       // 訪問済みの記録
```

### 実装方法2: 再帰を使った実装
```rust
fn dfs(position: (i32, i32), visited: &mut HashSet<(i32, i32)>) {
    // 訪問済みにマーク
    visited.insert(position);
    
    // 隣接ノードを探索
    for next_pos in get_neighbors(position) {
        if !visited.contains(&next_pos) {
            dfs(next_pos, visited);  // 再帰呼び出し
        }
    }
}
```

## 4. スタック版DFSの詳細実装

### ステップ1: 初期化
```rust
let mut stack = Vec::new();
let mut visited = HashSet::new();

// スタート地点をスタックに追加
stack.push(start_position);
```

### ステップ2: メインループ
```rust
while !stack.is_empty() {
    // 1. スタックの頂上を取り出す（pop）
    let current_pos = stack.pop().unwrap();
    
    // 2. 訪問済みチェック
    if visited.contains(&current_pos) {
        continue;
    }
    
    // 3. 訪問済みにマーク
    visited.insert(current_pos);
    
    // 4. 処理（例：ゴール判定、カウントなど）
    if current_pos == goal {
        return true;  // ゴール発見
    }
    
    // 5. 隣接ノードをスタックに追加
    for direction in [(0,1), (1,0), (0,-1), (-1,0)] {
        let next_pos = (
            current_pos.0 + direction.0,
            current_pos.1 + direction.1
        );
        
        if is_valid(next_pos) && !visited.contains(&next_pos) {
            stack.push(next_pos);
        }
    }
}
```

## 5. 再帰版DFSの詳細実装

### 基本構造
```rust
fn dfs_recursive(
    grid: &Vec<Vec<char>>,
    position: (i32, i32),
    visited: &mut HashSet<(i32, i32)>,
    path: &mut Vec<(i32, i32)>
) -> bool {
    // 1. 境界チェック
    if !is_valid_position(grid, position) {
        return false;
    }
    
    // 2. 訪問済みチェック
    if visited.contains(&position) {
        return false;
    }
    
    // 3. 訪問済みにマーク
    visited.insert(position);
    path.push(position);
    
    // 4. ゴール判定
    if grid[position.0 as usize][position.1 as usize] == 'G' {
        return true;  // ゴール発見
    }
    
    // 5. 4方向を再帰的に探索
    for direction in [(0,1), (1,0), (0,-1), (-1,0)] {
        let next_pos = (
            position.0 + direction.0,
            position.1 + direction.1
        );
        
        if dfs_recursive(grid, next_pos, visited, path) {
            return true;  // ゴールへの経路発見
        }
    }
    
    // 6. バックトラック（必要な場合）
    path.pop();
    false
}
```

## 6. DFSの思考プロセス

### 6.1 問題分析
1. **何を探索するか？**（全探索？特定のゴール？）
2. **経路を記録する必要があるか？**
3. **バックトラックが必要か？**
4. **訪問順序は重要か？**

### 6.2 DFSが適している場合
- ✅ 全ての可能な経路を試したい
- ✅ 解が深い位置にある可能性が高い
- ✅ メモリ制約が厳しい（BFSより省メモリ）
- ✅ トポロジカルソートが必要
- ✅ 連結成分の検出

### 6.3 DFSが不適な場合
- ❌ 最短経路を求めたい → BFSを使う
- ❌ 浅い位置に解がある → BFSの方が速い

## 7. DFSとBFSの比較

| 特徴 | DFS | BFS |
|------|-----|-----|
| データ構造 | スタック（LIFO） | キュー（FIFO） |
| 探索順序 | 深さ優先 | 幅優先 |
| 最短経路 | 保証されない | 保証される |
| メモリ使用量 | O(深さ) | O(幅) |
| 実装方法 | 再帰/スタック | キュー |
| 適用例 | 迷路探索、バックトラック | 最短経路、レベル順探索 |

## 8. DFSの応用パターン

### パターン1: 全探索（すべての到達可能なノード）
```rust
fn dfs_all_reachable(start: (i32, i32)) -> HashSet<(i32, i32)> {
    let mut visited = HashSet::new();
    let mut stack = vec![start];
    
    while let Some(pos) = stack.pop() {
        if visited.insert(pos) {  // 新規追加ならtrue
            // 隣接ノードを追加
            for next in get_neighbors(pos) {
                if !visited.contains(&next) {
                    stack.push(next);
                }
            }
        }
    }
    visited
}
```

### パターン2: 経路探索（パスを記録）
```rust
fn dfs_find_path(start: (i32, i32), goal: (i32, i32)) -> Option<Vec<(i32, i32)>> {
    let mut visited = HashSet::new();
    let mut path = Vec::new();
    
    if dfs_with_path(start, goal, &mut visited, &mut path) {
        Some(path)
    } else {
        None
    }
}
```

### パターン3: 連結成分の検出
```rust
fn count_connected_components(grid: &Vec<Vec<char>>) -> usize {
    let mut visited = HashSet::new();
    let mut count = 0;
    
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] != '#' && !visited.contains(&(i, j)) {
                // 新しい連結成分を発見
                dfs_mark_component(&grid, (i, j), &mut visited);
                count += 1;
            }
        }
    }
    count
}
```

## 9. よくあるミスと対策

### ミス1: 無限ループ
```rust
// ❌ 間違い: 訪問済みチェックを忘れる
fn dfs(pos: (i32, i32)) {
    for next in get_neighbors(pos) {
        dfs(next);  // 無限ループの危険！
    }
}

// ✅ 正解: 訪問済みをチェック
fn dfs(pos: (i32, i32), visited: &mut HashSet<(i32, i32)>) {
    if !visited.insert(pos) {
        return;  // 既に訪問済みなら終了
    }
    for next in get_neighbors(pos) {
        dfs(next, visited);
    }
}
```

### ミス2: スタックオーバーフロー
```rust
// ❌ 危険: 深い再帰でスタックオーバーフロー
fn deep_recursion(depth: usize) {
    if depth > 10000 {  // 深すぎる！
        return;
    }
    deep_recursion(depth + 1);
}

// ✅ 対策: スタックを使った反復実装
fn iterative_dfs(start: (i32, i32)) {
    let mut stack = vec![start];
    // 反復的に処理
}
```

## 10. デバッグのコツ

### 探索順序の可視化
```rust
fn dfs_with_order(start: (i32, i32)) {
    let mut visited = HashSet::new();
    let mut order = 0;
    
    fn dfs_inner(pos: (i32, i32), visited: &mut HashSet<(i32, i32)>, order: &mut i32) {
        if !visited.insert(pos) {
            return;
        }
        
        *order += 1;
        println!("訪問順序 {}: {:?}", order, pos);
        
        for next in get_neighbors(pos) {
            dfs_inner(next, visited, order);
        }
    }
    
    dfs_inner(start, &mut visited, &mut order);
}
```

### スタックの中身を確認
```rust
// デバッグ時にスタックの状態を出力
println!("Stack: {:?}", stack);
println!("Visited: {:?}", visited);
```

## 11. 時間計算量と空間計算量

### 時間計算量
- **O(V + E)**
  - V: ノード数
  - E: エッジ数
- グリッドの場合: **O(H × W)**

### 空間計算量
- **スタック版**: O(V) - 最悪の場合全ノード
- **再帰版**: O(D) - Dは最大深さ
- 一般的にDFSはBFSよりメモリ効率が良い

## 12. まとめ

DFSは「一つの道を突き進む」探索方法です。

**覚えておくべきポイント：**
1. スタックを使う（LIFO）または再帰
2. 訪問済みの管理が重要（無限ループ防止）
3. バックトラックで別の道を探索
4. 最短経路は保証されない

**実装の選択：**
- **再帰版**: コードがシンプル、理解しやすい
- **スタック版**: スタックオーバーフローを避けられる

**使い分け：**
- 全探索、連結成分 → DFS
- 最短経路 → BFS
- メモリ制約が厳しい → DFS
- 浅い解を期待 → BFS

この基本パターンを理解すれば、迷路探索、パズル解法、グラフの連結性チェックなど、様々な問題に応用できます！