# BFS (幅優先探索) アルゴリズムの組み立てプロセス

## 1. BFSとは？
**幅優先探索（Breadth-First Search）** は、グラフや木構造を探索するアルゴリズムです。  
スタート地点から「波紋が広がるように」同心円状に探索を進めます。

### 特徴
- **最短経路を保証**: 重みなしグラフでは最短経路を見つけられる
- **キュー（Queue）を使用**: 先入先出し（FIFO）でノードを処理
- **レベル順の探索**: 距離1のノード → 距離2のノード → ... の順に探索

## 2. BFSの基本公式

### 算数レベルの公式
```
距離(次のマス) = 距離(現在のマス) + 1
```

### 探索の順序
```
レベル0: スタート地点
レベル1: スタートから1手で行ける場所
レベル2: スタートから2手で行ける場所
...以下続く
```

## 3. BFSアルゴリズムの組み立てステップ

### ステップ1: データ構造の準備
```rust
// 必要なデータ構造
let mut queue = VecDeque::new();        // キュー（待ち行列）
let mut visited = HashMap::new();       // 訪問済みの記録
let mut distances = HashMap::new();     // 各地点への距離
```

### ステップ2: 初期化
```rust
// スタート地点をキューに入れる
queue.push_back((start_position, 0));   // (位置, 距離)
visited.insert(start_position, true);   // 訪問済みにする
```

### ステップ3: メインループ
```rust
while !queue.is_empty() {
    // 1. キューの先頭を取り出す
    let (current_pos, current_dist) = queue.pop_front().unwrap();
    
    // 2. 4方向（上下左右）を調べる
    for direction in [(0,1), (1,0), (0,-1), (-1,0)] {
        let next_pos = (
            current_pos.0 + direction.0,
            current_pos.1 + direction.1
        );
        
        // 3. 次の位置が有効かチェック
        if is_valid(next_pos) && !visited.contains_key(&next_pos) {
            // 4. 訪問済みにして、キューに追加
            visited.insert(next_pos, true);
            queue.push_back((next_pos, current_dist + 1));
            
            // 5. 距離を記録
            distances.insert(next_pos, current_dist + 1);
        }
    }
}
```

## 4. 実装時の思考プロセス

### 4.1 問題を理解する
1. **スタート地点はどこか？**
2. **ゴール（ターゲット）はどこか？**
3. **通れない場所（壁）はあるか？**
4. **何を求めるのか？**（最短距離？経路？到達可能性？）

### 4.2 BFSが適している理由を確認
- ✅ 最短経路を求める → BFSが最適
- ✅ 全ての重みが同じ（1手=1距離）→ BFSが使える
- ✅ 複数の目標から最も近いものを探す → BFSで全探索

### 4.3 境界条件を考える
```rust
// 境界チェックのパターン
if target_row < 0 || target_col < 0 {
    continue;  // 負の座標は無効
}
if target_row >= grid.len() || target_col >= grid[0].len() {
    continue;  // グリッドの外は無効
}
if grid[target_row][target_col] == '#' {
    continue;  // 壁は通れない
}
```

### 4.4 最適化を考える
```rust
// 早期終了の条件（必要な場合）
if result.len() == target_count {
    break;  // 全ターゲットを見つけたら終了
}
```

## 5. よくあるミスと対策

### ミス1: 訪問済みチェックのタイミング
```rust
// ❌ 間違い: キューから取り出した時にチェック
while !queue.is_empty() {
    let pos = queue.pop_front();
    if visited.contains_key(&pos) { continue; }  // 遅い！
    visited.insert(pos, true);
}

// ✅ 正解: キューに入れる時にチェック
if !visited.contains_key(&next_pos) {
    visited.insert(next_pos, true);  // 先に訪問済みにする
    queue.push_back(next_pos);
}
```

### ミス2: 距離の計算
```rust
// ❌ 間違い: 距離を別途管理
let mut distance = 0;
distance += 1;  // これだと全体の距離になってしまう

// ✅ 正解: ノードごとに距離を持つ
queue.push_back((next_pos, current_dist + 1));
```

## 6. 実装例の解説

今回のコードでは：

1. **キューの要素**: `(位置, 距離)` のタプル
2. **訪問済み管理**: `HashMap<(isize, isize), bool>`
3. **結果の保存**: ターゲット位置のみ距離を記録
4. **最適化**: 全ターゲット発見で早期終了

## 7. 時間計算量と空間計算量

### 時間計算量
- **O(V + E)** 
  - V: ノード数（マスの数）
  - E: エッジ数（接続の数）
- グリッドの場合: **O(H × W)** （高さ×幅）

### 空間計算量
- **O(V)**: visitedとqueueの保存
- グリッドの場合: **O(H × W)**

## 8. BFSの応用パターン

### パターン1: 単一の最短経路
```rust
// ゴールに到達したらすぐreturn
if position == goal {
    return Some(distance);
}
```

### パターン2: 全地点への最短距離（今回のコード）
```rust
// 全ての到達可能な地点の距離を計算
let distances = bfs_distance_map(&grid, start, targets.len());
```

### パターン3: 複数ゴールから最も近いもの
```rust
// 今回の実装: 複数のターゲットから最小距離を選択
find_nearest_target(&distances, &targets)
```

## 9. デバッグのコツ

### 視覚化
```rust
// 各ステップでキューの中身を出力
println!("Queue: {:?}", queue);
println!("Current: {:?}, Distance: {}", pos, dist);
```

### 距離マップの確認
```rust
// 計算後の距離を視覚的に確認
for y in 0..height {
    for x in 0..width {
        match distances.get(&(y, x)) {
            Some(d) => print!("{:2} ", d),
            None => print!("## "),
        }
    }
    println!();
}
```

## 10. まとめ

BFSは「波紋のように広がる」探索方法です。

**覚えておくべきポイント：**
1. キューを使う（FIFO）
2. 訪問済みは「キューに入れる時」にマーク
3. 距離は「現在の距離 + 1」
4. 最短経路が保証される

**実装の流れ：**
1. データ構造を準備（queue, visited, distances）
2. スタート地点を初期化
3. キューが空になるまでループ
4. 各ノードから隣接ノードを探索
5. 未訪問なら訪問済みにしてキューに追加

この基本パターンを理解すれば、様々な最短経路問題に応用できます！