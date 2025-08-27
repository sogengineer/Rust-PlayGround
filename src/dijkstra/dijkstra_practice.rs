use std::collections::BinaryHeap;
use std::cmp::Reverse;
use std::{u32, usize};

// =============================================================================
// 📚 ダイクストラ法 練習問題集
// =============================================================================

type Graph = Vec<Vec<(usize, u32)>>;

// =============================================================================
// 練習問題1: 基本的なダイクストラ法の実装
// =============================================================================
// 
// 以下の都市間の移動コストが与えられています：
// 東京(0) --30-- 横浜(1)
//   |              |
//  50             20
//   |              |
// 大阪(2) --40-- 京都(3)
//   |              |
//  60             15
//   |              |
// 名古屋(4)------神戸(5)
//        25
//
// TODO: 東京から全都市への最短距離を求める
//     let graph1: Graph = vec![
//     vec![(1, 30), (2, 50)],           // 東京: 横浜(30), 大阪(50)
//     vec![(0, 30), (3, 20)],           // 横浜: 東京(30), 京都(20)
//     vec![(0, 50), (3, 40), (4, 60)],  // 大阪: 東京(50), 京都(40), 名古屋(60)
//     vec![(1, 20), (2, 40), (5, 15)],  // 京都: 横浜(20), 大阪(40), 神戸(15)
//     vec![(2, 60), (5, 25)],           // 名古屋: 大阪(60), 神戸(25)
//     vec![(3, 15), (4, 25)],           // 神戸: 京都(15), 名古屋(25)
// ];
fn practice1_simple_dijkstra(graph: &Graph, start: usize) -> Vec<u32> {
    // TODO: ここに実装を書く
    // ヒント:
    // 1. distances配列を無限大で初期化
    // 2. heap（優先度付きキュー）を作成
    // 3. 始点の距離を0に設定
    // 4. while文でヒープから取り出して処理
    let n = graph.len();
    let mut distances = vec!(u32::MAX; n);
    let mut heap = BinaryHeap::new();
    
    distances[start] = 0;
    heap.push(Reverse((0, start)));
    while let Some(Reverse((cost, node))) = heap.pop() {
        if cost > distances[node] {
            continue;
        }
        for &(next_node, edge_cost) in &graph[node] {
            let new_cost = edge_cost + cost;
            if new_cost < distances[next_node] {
                distances[next_node] = new_cost;
                heap.push(Reverse((new_cost, next_node)));
            }
        }
    };
    return distances

}

// =============================================================================
// 練習問題2: 特定の2点間の最短距離
// =============================================================================
//
// 始点から終点への最短距離だけを求める関数を実装してください。
// 終点に到達したら処理を打ち切ることで効率化できます。

fn practice2_shortest_distance(graph: &Graph, start: usize, end: usize) -> Option<u32> {
    // TODO: ここに実装を書く
    // ヒント:
    // 1. 基本的なダイクストラ法と同じ
    // 2. ただし、endノードを処理したら即座にreturn
    // 3. 到達できない場合はNone
    let n = graph.len();
    let mut distances = vec!(u32::MAX; n);
    let mut heap = BinaryHeap::new();

    distances[start] = 0;
    heap.push(Reverse((0, start)));
    while let Some(Reverse((cost ,node))) = heap.pop() {
        if cost > distances[node] {
            continue;
        }
        if node == end {
            return Some(distances[node]);
        }
        for &(next_node, edge_cost) in &graph[node] {
            let new_cost: u32 = edge_cost + cost;
            if new_cost < distances[next_node] {
                distances[next_node] = new_cost;
                heap.push(Reverse((new_cost, next_node)));
            }
        }
    };
    None
}

// =============================================================================
// 練習問題3: 経路の本数を数える
// =============================================================================
//
// 始点から終点への「最短経路」が何通りあるか数える関数を実装してください。
// 例：A→B→D と A→C→D が同じコストなら、2通り

fn practice3_count_shortest_paths(graph: &Graph, start: usize, end: usize) -> usize {
    let n = graph.len();
    let mut distances = vec![u32::MAX; n];
    let mut path_count = vec![0; n];  // ← Vec::new()から変更
    let mut heap: BinaryHeap<Reverse<(u32, usize)>> = BinaryHeap::new();

    distances[start] = 0;
    path_count[start] = 1;  // ← 追加：始点への経路は1通り
    heap.push(Reverse((0, start)));

    while let Some(Reverse((cost, node))) = heap.pop() {
        if cost > distances[node] {
            continue;
        }

        for &(next_node, edge_cost) in &graph[node] {
            let new_cost = cost + edge_cost;

            if new_cost < distances[next_node] {
                distances[next_node] = new_cost;
                path_count[next_node] = path_count[node];  // ← 追加：経路数をコピー
                heap.push(Reverse((new_cost, next_node)));
            } else if new_cost == distances[next_node] {  // ← 追加：同じコストの場合
                path_count[next_node] += path_count[node];  // 経路数を加算
            }
        }
    }

    return path_count[end]  // ← path_count.len()から変更
}

// =============================================================================
// 練習問題4: 制約付き最短経路
// =============================================================================
//
// 「通過できるノード数が最大K個」という制約付きで最短経路を求めてください。
// 例：K=3なら、始点→中継1→中継2→終点（4ノード）はNG

fn practice4_limited_hops(graph: &Graph, start: usize, end: usize, max_hops: usize) -> Option<u32> {
    // グラフのノード数を取得（例：6個の都市）
    let n = graph.len();
    
    // 2次元配列を作成: distances[ノード番号][ホップ数] = そのノードにそのホップ数で到達する最小コスト
    // max_hops=2なら、[0ホップ, 1ホップ, 2ホップ]の3つ分の領域が必要なので max_hops+1
    // 例: distances[3][1] = ノード3に1ホップで到達する最小コスト
    let mut distances = vec![vec![u32::MAX; max_hops + 1]; n];
    
    // 優先度付きキュー: (コスト, ノード番号, ホップ数) のタプルを格納
    // Reverseで包むことで、コストが小さい順に取り出せる
    let mut heap: BinaryHeap<Reverse<(u32, usize, usize)>> = BinaryHeap::new();

    // 始点への0ホップでのコストは0（移動なし、自分自身）
    // 例: start=0なら、distances[0][0] = 0
    distances[start][0] = 0;
    
    // ヒープに始点を追加: (コスト0, 始点ノード, 0ホップ)
    // 例: Reverse((0, 0, 0)) → 東京から0ホップで東京に到達
    heap.push(Reverse((0, start, 0)));
    
    // ヒープが空になるまで処理を繰り返す
    while let Some(Reverse((cost, node, hop_count))) = heap.pop() {
        // ホップ数が制限を超えている場合はスキップ
        // 例: max_hops=2でhop_count=3ならスキップ（3ホップは許可されない）
        if hop_count > max_hops {
            continue;
        }
        
        // 既により良い経路が見つかっている場合はスキップ
        // 例: distances[3][1]=50 で cost=60 ならスキップ（既に良い経路がある）
        if cost > distances[node][hop_count] {
            continue;
        }
        
        // 終点に到達したら、そのコストを返す
        // 例: node=5(神戸)でend=5なら、現在のコストを返す
        if node == end {
            return Some(cost)
        }
        
        // まだホップ数に余裕がある場合のみ、隣接ノードを探索
        // 例: hop_count=1, max_hops=2 なら、あと1ホップできる
        if hop_count < max_hops {
            // 現在のノードから行ける全ての隣接ノードをチェック
            // 例: node=0(東京)なら、横浜と大阪をチェック
            for &(next_node, edge_cost) in &graph[node] {
                // 新しいコスト = 現在までのコスト + エッジのコスト
                // 例: cost=30(東京→横浜) + edge_cost=20(横浜→京都) = 50
                let new_cost = cost + edge_cost;
                
                // 次のホップ数 = 現在のホップ数 + 1
                // 例: hop_count=1 → next_hop=2（2ホップ目）
                let next_hop = hop_count + 1;
                
                // より良い経路が見つかった場合のみ更新
                // 例: distances[3][2]=100 で new_cost=50 なら更新
                if new_cost < distances[next_node][next_hop] {
                    // 最短距離を更新
                    distances[next_node][next_hop] = new_cost;
                    
                    // ヒープに新しい状態を追加
                    // 例: Reverse((50, 3, 2)) → コスト50で京都に2ホップで到達
                    heap.push(Reverse((new_cost, next_node, next_hop)));
                }
            } 
        }
    }
    
    // ヒープが空になっても終点に到達できなかった場合はNone
    None
}

// =============================================================================
// 練習問題5: 負の重みを検出
// =============================================================================
//
// グラフに負の重みが含まれているか判定する関数を実装してください。
// （ダイクストラ法は負の重みでは正しく動作しません）

fn practice5_has_negative_edge(_edges: &[(usize, usize, i32)]) -> bool {
    // TODO: ここに実装を書く
    // edges: [(from, to, weight), ...]
    
    false
}

// =============================================================================
// テスト用のmain関数
// =============================================================================

fn main() {
    println!("=== 📚 ダイクストラ法 練習問題 ===\n");
    
    // 練習問題1用のグラフ
    let graph1: Graph = vec![
        vec![(1, 30), (2, 50)],           // 東京: 横浜(30), 大阪(50)
        vec![(0, 30), (3, 20)],           // 横浜: 東京(30), 京都(20)
        vec![(0, 50), (3, 40), (4, 60)],  // 大阪: 東京(50), 京都(40), 名古屋(60)
        vec![(1, 20), (2, 40), (5, 15)],  // 京都: 横浜(20), 大阪(40), 神戸(15)
        vec![(2, 60), (5, 25)],           // 名古屋: 大阪(60), 神戸(25)
        vec![(3, 15), (4, 25)],           // 神戸: 京都(15), 名古屋(25)
    ];
    
    println!("都市ネットワーク:");
    println!("東京(0) --30-- 横浜(1)");
    println!("  |              |");
    println!(" 50             20");
    println!("  |              |");
    println!("大阪(2) --40-- 京都(3)");
    println!("  |              |");
    println!(" 60             15");
    println!("  |              |");
    println!("名古屋(4)------神戸(5)");
    println!("       25");
    println!();
    
    // 練習問題1のテスト
    println!("=== 練習問題1: 基本的なダイクストラ法 ===");
    let distances = practice1_simple_dijkstra(&graph1, 0);
    if distances.is_empty() {
        println!("未実装です");
    } else {
        let cities = ["東京", "横浜", "京都", "大阪", "名古屋", "神戸"];
        for (i, &dist) in distances.iter().enumerate() {
            if i < cities.len() {
                println!("東京 → {}: {}", cities[i], 
                    if dist == u32::MAX { "到達不可".to_string() } else { format!("{}km", dist) });
            }
        }
    }
    println!("期待値: 東京→東京:0, 横浜:30, 京都:50, 大阪:50, 名古屋:90, 神戸:65");
    println!();
    
    // 練習問題2のテスト
    println!("=== 練習問題2: 特定の2点間の最短距離 ===");
    match practice2_shortest_distance(&graph1, 0, 5) {
        Some(dist) => println!("東京 → 神戸: {}km", dist),
        None => println!("未実装または到達不可"),
    }
    println!("期待値: 65km (東京→横浜→京都→神戸)");
    println!();
    
    // 練習問題3のテスト
    println!("=== 練習問題3: 最短経路の本数 ===");
    let simple_graph: Graph = vec![
        vec![(1, 10), (2, 10)],  // A: B(10), C(10)
        vec![(3, 10)],           // B: D(10)
        vec![(3, 10)],           // C: D(10)
        vec![],                  // D: なし
    ];
    let count = practice3_count_shortest_paths(&simple_graph, 0, 3);
    println!("A → Dの最短経路の本数: {}", count);
    println!("期待値: 2通り (A→B→DとA→C→D)");
    println!();
    
    // 練習問題4のテスト
    println!("=== 練習問題4: 制約付き最短経路 ===");
    match practice4_limited_hops(&graph1, 0, 5, 2) {
        Some(dist) => println!("東京 → 神戸 (最大2ホップ): {}km", dist),
        None => println!("未実装または条件を満たす経路なし"),
    }
    println!("期待値: 条件を満たす経路なし（最短でも3ホップ必要）");
    println!();
    
    // 練習問題5のテスト
    println!("=== 練習問題5: 負の重み検出 ===");
    let edges_positive = vec![(0, 1, 10), (1, 2, 20), (2, 3, 30)];
    let edges_negative = vec![(0, 1, 10), (1, 2, -5), (2, 3, 30)];
    
    println!("正の重みのみ: {}", 
        if practice5_has_negative_edge(&edges_positive) { "負あり" } else { "負なし" });
    println!("負の重みあり: {}", 
        if practice5_has_negative_edge(&edges_negative) { "負あり" } else { "負なし" });
    println!();
    
    println!("ヒント:");
    println!("- 練習問題1: 基本のダイクストラ法をそのまま実装");
    println!("- 練習問題2: endに到達したら早期return");
    println!("- 練習問題3: 同じコストの経路を数える");
    println!("- 練習問題4: ホップ数も状態として管理");
    println!("- 練習問題5: 単純にweight < 0をチェック");
}