use std::collections::{VecDeque, HashSet};

// =============================================================================
// 🏝️ 島の探索問題: BFSとDFSの実装練習
// =============================================================================
//
// 海と島で構成されたマップが与えられます。
// '.' = 海、'#' = 陸地
// 
// 【問題】
// 1. 島の数を数える（連結した陸地を1つの島とする）
// 2. 最大の島の面積を求める
// 3. 指定された2つの島の間の最短距離を求める
//
// この問題はDFSとBFS両方で解くことができます！
// 両方のアプローチを実装してみましょう。

// TODO: DFSを使って島を探索する関数
fn dfs_explore_island(
    grid: &Vec<Vec<char>>,
    visited: &mut HashSet<(usize, usize)>,
    start_row: usize,
    start_col: usize,
) -> usize {
    println!("start");
    println!("{},{}", start_row, start_col);
    // ヒント:
    // 1. スタック（Vec）を使うか、再帰で実装
    // 2. start位置から始めて、連結している全ての陸地を訪問
    // 3. 訪問した陸地の数（面積）を返す
    // 4. visitedに訪問済みを記録（他の島と区別するため）
    visited.insert((start_row, start_col));
    let directions: [(isize, isize); 4] = [(1,0),(0,1),(-1,0),(0,-1)];
    let mut size = 0;
    println!("for にはいる");
    // print!("({},{})", start_row, start_col);
    for direction in directions {
        let (next_row, next_col) = direction;
        let target_row = ( start_row as isize ) + next_row;
        let target_col = ( start_col as isize ) + next_col;
        let is_visited = visited.contains(&(target_row as usize, target_col as usize));
        println!("({},{})", target_row, target_col);
        println!("({},{})", grid[target_row as usize][target_col as usize] == '.', is_visited);
        if target_row < 0 || target_col < 0 || is_visited || grid[target_row as usize][target_col as usize] == '.' {
            println!("ここでブレイク");
            return size
        }
        println!("{}", grid[target_row as usize][target_col as usize]);
        if grid[target_row as usize][target_col as usize]  == '#' {
            size = size + dfs_explore_island(grid, visited, target_row as usize, target_col as usize);
        } 
        break;
    }
    return size
}

// TODO: DFSを使って全ての島を見つける関数
fn count_islands_dfs(grid: &Vec<Vec<char>>) -> (usize, usize) {
    // ヒント:
    // 1. グリッド全体をスキャン
    // 2. 未訪問の陸地を見つけたら、dfs_explore_islandで探索
    // 3. 島の数と最大面積を追跡
    // 返り値: (島の数, 最大面積)
    let mut i = 0;
    let mut j = 0;
    let mut max_size = 0;
    let mut island_count = 0;
    let mut visited: HashSet<(usize,usize), > = HashSet::new();
    let mut result = (0,0);
    while i < grid.len() {
        while j < grid[i].len() {
            let is_visited = visited.contains(&(i,j));
            let is_sea = grid[i][j] == '.';
            if is_visited || is_sea {
                continue;
            } 
            if grid[i][j] == '#' {
                let size = dfs_explore_island(grid, &mut visited, i, j);
                println!("{}", size);
                island_count += 1;
                if size > max_size {
                    max_size = size;
                }
                result = (island_count, max_size);
            };
            j = j+1;
        }
        i = i+1;
    };
    return result;
}

// TODO: BFSを使って島を探索する関数
fn bfs_explore_island(
    grid: &Vec<Vec<char>>,
    visited: &mut HashSet<(usize, usize)>,
    start_row: usize,
    start_col: usize,
) -> usize {
    // ヒント:
    // 1. キュー（VecDeque）を使う
    // 2. start位置から始めて、層ごとに探索
    // 3. 訪問した陸地の数（面積）を返す
    // 4. visitedに訪問済みを記録
    
    // TODO: ここにBFSの実装を書く
    0
}

// TODO: BFSを使って全ての島を見つける関数
fn count_islands_bfs(grid: &Vec<Vec<char>>) -> (usize, usize) {
    // ヒント:
    // count_islands_dfsと同じロジックだが、
    // dfs_explore_islandの代わりにbfs_explore_islandを使う
    
    // TODO: ここに実装を書く
    (0, 0)
}

// TODO: BFSを使って2つの島の間の最短距離を求める
fn shortest_distance_between_islands(
    grid: &Vec<Vec<char>>,
    island1_start: (usize, usize),
    island2_start: (usize, usize),
) -> Option<usize> {
    // ヒント:
    // 1. まず島1の全ての陸地を特定（BFSかDFS）
    // 2. 島1の全ての陸地を開始点として、多始点BFSを実行
    // 3. 海を渡って島2の陸地に到達するまでの最短距離を求める
    // 4. 到達できない場合はNone
    
    // TODO: ここに実装を書く
    None
}

// ヘルパー関数: 4方向の隣接マスを取得
fn get_neighbors(row: usize, col: usize, rows: usize, cols: usize) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    
    for (dr, dc) in directions {
        let new_row = row as isize + dr;
        let new_col = col as isize + dc;
        
        if new_row >= 0 && new_row < rows as isize && 
           new_col >= 0 && new_col < cols as isize {
            neighbors.push((new_row as usize, new_col as usize));
        }
    }
    
    neighbors
}

fn main() {
    println!("=== 🏝️ 島の探索問題: DFSとBFS両方で解いてみよう！ ===\n");
    
    // テストマップ
    let grid = vec![
        vec!['#', '#', '.', '.', '.', '.', '#', '#', '#'],
        vec!['#', '.', '.', '.', '.', '.', '#', '.', '#'],
        vec!['.', '.', '.', '#', '#', '.', '#', '#', '#'],
        vec!['.', '.', '.', '#', '#', '.', '.', '.', '.'],
        vec!['.', '#', '.', '.', '.', '.', '.', '.', '.'],
        vec!['.', '#', '#', '.', '.', '#', '#', '#', '.'],
        vec!['.', '.', '.', '.', '.', '#', '.', '#', '.'],
        vec!['#', '#', '#', '.', '.', '#', '#', '#', '.'],
        vec!['#', '.', '#', '.', '.', '.', '.', '.', '.'],
    ];
    
    println!("マップ（'#' = 陸地, '.' = 海）:");
    for row in &grid {
        println!("{}", row.iter().collect::<String>());
    }
    println!();
    
    // DFSでの解法
    println!("=== DFSでの解法 ===");
    let (islands_dfs, max_area_dfs) = count_islands_dfs(&grid);
    println!("島の数: {}", islands_dfs);
    println!("最大の島の面積: {}", max_area_dfs);
    println!();
    
    // BFSでの解法
    println!("=== BFSでの解法 ===");
    let (islands_bfs, max_area_bfs) = count_islands_bfs(&grid);
    println!("島の数: {}", islands_bfs);
    println!("最大の島の面積: {}", max_area_bfs);
    println!();
    
    // 島間の最短距離（BFS）
    println!("=== 島間の最短距離 ===");
    // 左上の島(0,0)と中央の島(3,3)の間の距離
    match shortest_distance_between_islands(&grid, (0, 0), (3, 3)) {
        Some(distance) => println!("島(0,0)と島(3,3)の間の最短距離: {}", distance),
        None => println!("島(0,0)と島(3,3)は接続できません"),
    }
    
    println!("\n実装のヒント:");
    println!("1. DFS実装:");
    println!("   - 再帰またはスタックを使用");
    println!("   - 深さ優先で島を探索");
    println!("   - visitedで訪問済みを管理");
    println!();
    println!("2. BFS実装:");
    println!("   - キュー（VecDeque）を使用");
    println!("   - 幅優先で島を探索");
    println!("   - 同じくvisitedで管理");
    println!();
    println!("3. 最短距離:");
    println!("   - 多始点BFSが効率的");
    println!("   - 島1の全陸地からスタート");
    println!("   - 最初に島2に到達した時点が最短");
    println!();
    println!("両方の手法で同じ結果が得られることを確認してください！");
    
    // デバッグ用: get_neighbors関数のテスト
    println!("\nデバッグ: (1,1)の隣接マス:");
    for (r, c) in get_neighbors(1, 1, 9, 9) {
        println!("  ({}, {})", r, c);
    }
}