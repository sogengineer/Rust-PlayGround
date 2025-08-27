use std::collections::BinaryHeap;
use std::cmp::Reverse;

// =============================================================================
// 📦 配送ネットワークの最適化: ダイクストラ法の実装
// =============================================================================
//
// 配送センター間の輸送コストを最小化する問題です。
// 重み付きグラフの最短経路問題をダイクストラ法で解きます。
//
// グラフ構造:
//     A(0)
//    /   \
//   5     2
//  /       \
// B(1)--3--C(2)
//  \      / \
//   7    1   4
//    \  /     \
//    D(3)-----E(4)
//         6

type Graph = Vec<Vec<(usize, u32)>>;

// ダイクストラ法の実装（基本版）
fn dijkstra(graph: &Graph, start: usize) -> Vec<u32> {
    // グラフのノード数を取得（例：5個のノードA,B,C,D,E）
    let n = graph.len();
    
    // 各ノードへの最短距離を記録する配列を作成
    // 初期値はu32::MAX（無限大の代わり）で埋める
    // distances[0]=∞, distances[1]=∞, ..., distances[4]=∞
    let mut distances = vec![u32::MAX; n];
    
    // 優先度付きキュー（最小ヒープとして使用）
    // Reverseで包むことで、コストが小さい順に取り出せる
    let mut heap = BinaryHeap::new();
    
    // 始点（start）への距離は0に設定
    // 例：start=0(A)なら、distances[0] = 0
    distances[start] = 0;
    
    // ヒープに始点を追加
    // Reverse((コスト0, ノード番号start))の形で追加
    // 例：Reverse((0, 0)) → コスト0でノードAを追加
    heap.push(Reverse((0, start)));
    
    // ヒープが空になるまで処理を繰り返す
    while let Some(Reverse((cost, node))) = heap.pop() {
        // 【重要な最適化】
        // もし取り出したコストが、既に記録されている最短距離より大きければスキップ
        // （同じノードが異なるコストで複数回ヒープに入る可能性があるため）
        // 例：distances[1]=5なのに、cost=10のB(1)が出てきたらスキップ
        if cost > distances[node] {
            continue;
        }
        
        // 現在のノードから行ける全ての隣接ノードをチェック
        // graph[node]には [(隣接ノード, エッジのコスト), ...] が入っている
        // 例：node=0(A)なら、graph[0] = [(1, 5), (2, 2)] → Bへ5、Cへ2
        for &(next_node, edge_cost) in &graph[node] {
            // 新しいコスト = 現在のノードまでのコスト + エッジのコスト
            // 例：A(cost=0) → B(edge_cost=5) なら new_cost = 0 + 5 = 5
            let new_cost = cost + edge_cost;

            // もし新しいコストが、記録されている距離より小さければ更新
            // 例：distances[1]=∞ で new_cost=5 なら更新する
            if new_cost < distances[next_node] {
                // 最短距離を更新
                distances[next_node] = new_cost;
                
                // ヒープに新しいコストでこのノードを追加
                // 例：Reverse((5, 1)) → コスト5でノードBを追加
                heap.push(Reverse((new_cost, next_node)))
            }
        }
    }
    
    // 全ノードへの最短距離の配列を返す
    // 例：[0, 5, 2, 3, 6] → A:0, B:5, C:2, D:3, E:6
    return distances;
}

// 経路復元機能付きダイクストラ法
fn dijkstra_with_path(graph: &Graph, start: usize) -> (Vec<u32>, Vec<Option<usize>>) {
    // グラフのノード数を取得
    let n = graph.len();
    
    // 各ノードへの最短距離を記録する配列（初期値は無限大）
    let mut distances = vec![u32::MAX; n];
    
    // 【重要】各ノードに「どのノードから来たか」を記録する配列
    // previous[i] = Some(j) → ノードiにはノードjから来た
    // previous[i] = None → ノードiは未到達または始点
    // 例：previous = [None, Some(0), Some(0), Some(2), Some(2)]
    //     → A:始点, B:Aから, C:Aから, D:Cから, E:Cから
    let mut previous: Vec<Option<usize>> = vec![None; n];
    
    // 優先度付きキュー（最小ヒープ）
    let mut heap = BinaryHeap::new();
    
    // 始点の距離を0に設定
    distances[start] = 0;
    
    // ヒープに始点を追加
    // Reverse((0, 0)) → コスト0でノードAを追加
    heap.push(Reverse((0,start)));
    
    // ヒープが空になるまで処理
    while let Some(Reverse((cost, node))) = heap.pop() {
        // 既に処理済みならスキップ
        // （同じノードが複数回ヒープに入る可能性があるため）
        if cost > distances[node] {
            continue;
        }
        
        // 現在のノードから行ける全ての隣接ノードをチェック
        for &(next_node, edge_cost) in &graph[node] {
            // 新しいコストを計算
            // 例：C(cost=2) → D(edge_cost=1) なら new_cost = 2 + 1 = 3
            let new_cost = cost + edge_cost;
            
            // より短い経路が見つかったら更新
            if new_cost < distances[next_node] {
                // 最短距離を更新
                distances[next_node] = new_cost;
                
                // 【重要】どこから来たかを記録
                // 例：ノードD(3)に、ノードC(2)から来た場合
                //     previous[3] = Some(2)
                previous[next_node] = Some(node);
                
                // ヒープに新しいコストでこのノードを追加
                heap.push(Reverse((new_cost, next_node)));
            }
        }
    }
    
    // 最短距離の配列と、経路復元用の配列を両方返す
    // 例：([0, 5, 2, 3, 6], [None, Some(0), Some(0), Some(2), Some(2)])
    //     distances: A=0, B=5, C=2, D=3, E=6
    //     previous: A=始点, B=Aから, C=Aから, D=Cから, E=Cから
    (distances, previous)
}

// 経路を復元する関数
fn reconstruct_path(previous: &[Option<usize>], start: usize, end: usize) -> Vec<usize> {
    // 経路を格納するベクター
    let mut path = Vec::new();
    
    // 現在のノード（最初は終点から開始）
    let mut current = end;
    
    // 終点から始点まで逆向きに辿る
    // 例：A(0)→D(3)の経路を復元する場合
    //     previous = [None, Some(0), Some(0), Some(2), ...]
    //     D(3)はC(2)から来た、C(2)はA(0)から来た
    while current != start {
        // 現在のノードを経路に追加
        // 1回目：path = [3] (D)
        // 2回目：path = [3, 2] (D, C)
        path.push(current);
        
        // previous配列から「どこから来たか」を取得
        match previous[current] {
            // Some(prev)の場合：prevノードから来た
            // 例：previous[3] = Some(2) → D(3)はC(2)から来た
            Some(prev) => {
                // currentをprevに更新して、さらに遡る
                // 1回目：current = 3 → 2
                // 2回目：current = 2 → 0
                current = prev;
            },
            // Noneの場合：経路が存在しない（到達不可能）
            None => return vec![],
        }
    }
    
    // 最後に始点を追加
    // path = [3, 2, 0] (D, C, A)
    path.push(start);
    
    // 逆順になっているので正順に直す
    // path = [3, 2, 0] → [0, 2, 3] (A, C, D)
    path.reverse();
    
    // 完成した経路を返す
    // 例：[0, 2, 3] → A→C→Dの経路
    path
}

// ヘルパー関数: ノード番号を文字に変換
fn node_to_char(node: usize) -> char {
    (b'A' + node as u8) as char
}

// ヘルパー関数: 経路を文字列に変換
fn path_to_string(path: &[usize]) -> String {
    path.iter()
        .map(|&node| node_to_char(node).to_string())
        .collect::<Vec<_>>()
        .join(" → ")
}

fn main() {
    println!("=== 📦 配送ネットワークの最適化 ===");
    println!("ダイクストラ法による最短経路探索");
    println!("\n練習問題は src/dijkstra_practice.rs にあります！");
    println!("実行: cargo run --bin dijkstra_practice\n");
    
    // グラフの構築（隣接リスト表現）
    let graph: Graph = vec![
        vec![(1, 5), (2, 2)],           // A: B(5), C(2)
        vec![(0, 5), (2, 3), (3, 7)],   // B: A(5), C(3), D(7)
        vec![(0, 2), (1, 3), (3, 1), (4, 4)], // C: A(2), B(3), D(1), E(4)
        vec![(1, 7), (2, 1), (4, 6)],   // D: B(7), C(1), E(6)
        vec![(2, 4), (3, 6)],           // E: C(4), D(6)
    ];
    
    println!("配送センター間のネットワーク:");
    println!("    A(0)");
    println!("   /   \\");
    println!("  5     2");
    println!(" /       \\");
    println!("B(1)--3--C(2)");
    println!(" \\      / \\");
    println!("  7    1   4");
    println!("   \\  /     \\");
    println!("   D(3)-----E(4)");
    println!("        6");
    println!();
    
    // 基本のダイクストラ法
    println!("=== 基本実装: センターAからの最小コスト ===");
    let distances = dijkstra(&graph, 0);
    
    for (i, &dist) in distances.iter().enumerate() {
        if dist == u32::MAX {
            println!("A → {}: 到達不可能", node_to_char(i));
        } else {
            println!("A → {}: {}万円", node_to_char(i), dist);
        }
    }
    println!();
    
    // 経路復元付きダイクストラ法
    println!("=== 経路復元: 最短経路の表示 ===");
    let (distances_with_path, previous) = dijkstra_with_path(&graph, 0);
    // println!("{:?}",(distances_with_path, previous));
    for end in 1..graph.len() {
        if distances_with_path[end] != u32::MAX {
            let path = reconstruct_path(&previous, 0, end);
            if !path.is_empty() {
                println!("A → {}: {} (コスト: {}万円)",
                    node_to_char(end),
                    path_to_string(&path),
                    distances_with_path[end]
                );
            }
        }
    }
    println!();
    
    // 応用: 任意の2点間の最短経路
    println!("=== 応用: B→Eの最短経路 ===");
    let (distances_from_b, previous_from_b) = dijkstra_with_path(&graph, 1);
    let path_b_to_e = reconstruct_path(&previous_from_b, 1, 4);
    
    if !path_b_to_e.is_empty() {
        println!("B → E: {} (コスト: {}万円)",
            path_to_string(&path_b_to_e),
            distances_from_b[4]
        );
    } else {
        println!("B → E: 経路が見つかりません");
    }
    
    println!("\n実装のポイント:");
    println!("1. BinaryHeapは最大ヒープなので、Reverseで最小ヒープにする");
    println!("2. 既に処理済みのノードはスキップ（重要な最適化）");
    println!("3. u32::MAXを無限大として使用");
    println!("4. 経路復元にはpreviousベクターを使用");
}