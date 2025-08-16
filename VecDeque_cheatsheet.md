# VecDeque（両端キュー）の使い方まとめ

## 基本概念
`VecDeque`は両端（前と後ろ）から要素を効率的に追加・削除できるデータ構造です。

## インポート
```rust
use std::collections::VecDeque;
```

## 初期化
```rust
// 空のキューを作成
let mut queue: VecDeque<i32> = VecDeque::new();

// 初期容量を指定
let mut queue: VecDeque<i32> = VecDeque::with_capacity(10);

// 配列から作成
let mut queue: VecDeque<_> = VecDeque::from([1, 2, 3]);

// vecから変換
let mut queue: VecDeque<_> = vec![1, 2, 3].into();
```

## 要素の追加
```rust
let mut queue = VecDeque::new();

// 後ろに追加（エンキュー）
queue.push_back(1);     // [1]
queue.push_back(2);     // [1, 2]

// 前に追加
queue.push_front(0);    // [0, 1, 2]
```

## 要素の取り出し
```rust
// 前から取り出す（デキュー）
let first = queue.pop_front();     // Some(値) or None
let first = queue.pop_front().unwrap();  // 値を直接取得（Noneだとパニック）

// 後ろから取り出す
let last = queue.pop_back();       // Some(値) or None
```

## 要素の参照（削除しない）
```rust
// 前の要素を見る
let front = queue.front();         // Some(&値) or None
let front = queue.get(0);          // Some(&値) or None

// 後ろの要素を見る
let back = queue.back();           // Some(&値) or None

// インデックスでアクセス
let element = queue[0];            // 直接アクセス（境界チェックなし）
let element = queue.get(0);        // Option<&T>（安全）
```

## サイズ確認
```rust
// 要素数
let len = queue.len();

// 空かどうか
if queue.is_empty() {
    println!("キューは空");
}

// 容量
let capacity = queue.capacity();
```

## イテレーション
```rust
// 通常のイテレーション
for item in &queue {
    println!("{}", item);
}

// ミュータブルなイテレーション
for item in &mut queue {
    *item += 1;
}

// 所有権を取得するイテレーション
for item in queue.into_iter() {
    println!("{}", item);
}
```

## その他の便利なメソッド
```rust
// クリア
queue.clear();

// 要素の存在確認
if queue.contains(&5) {
    println!("5が含まれている");
}

// 回転
queue.rotate_left(1);   // 左に1つ回転
queue.rotate_right(1);  // 右に1つ回転

// 分割
let (left, right) = queue.split_at(2);

// 範囲を指定して削除
queue.drain(1..3);      // インデックス1から2を削除

// 並び替え（要素がOrdトレイトを実装している場合）
queue.make_contiguous().sort();
```

## BFS/DFSでの典型的な使い方

### BFS（幅優先探索）
```rust
let mut queue = VecDeque::new();
let mut visited = HashMap::new();

// 初期状態を追加
queue.push_back((start_pos, 0));
visited.insert(start_pos, true);

while !queue.is_empty() {
    // 前から取り出す（FIFO: 先入れ先出し）
    let (pos, dist) = queue.pop_front().unwrap();
    
    // 処理...
    
    // 次の状態を後ろに追加
    queue.push_back((next_pos, dist + 1));
}
```

### DFS（深さ優先探索）※VecDequeをスタックとして使用
```rust
let mut stack = VecDeque::new();

// 初期状態を追加
stack.push_back(start_pos);

while !stack.is_empty() {
    // 後ろから取り出す（LIFO: 後入れ先出し）
    let pos = stack.pop_back().unwrap();
    
    // 処理...
    
    // 次の状態を後ろに追加
    stack.push_back(next_pos);
}
```

## パフォーマンス
- `push_front`, `push_back`, `pop_front`, `pop_back`: **O(1)**
- インデックスアクセス: **O(1)**
- 中間への挿入・削除: **O(n)**

## Vecとの使い分け
- **Vec**: 後ろからの操作が主な場合
- **VecDeque**: 両端からの操作が必要な場合（キュー、デック）