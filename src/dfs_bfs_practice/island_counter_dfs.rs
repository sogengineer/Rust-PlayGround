use std::collections::{HashSet};

// TODO: この関数を実装してください
// DFS（深度優先探索）で1つの島を探索し、訪問済みにする
fn dfs(grid: &Vec<Vec<char>>, visited: &mut HashSet<(usize, usize)>, row: usize, col: usize) {
    // ヒント:
    // 1. 現在地を訪問済みにする
    // 2. 4方向を調べる
    // 3. 次の条件を満たしたら再帰的にdfsを呼ぶ:
    //    - 範囲内
    //    - 陸地（'1'）
    //    - 未訪問
    visited.insert((row, col));
    // println!("inserted row:{} col:{}", row, col);
    let directions: [(isize, isize); 4] = [(0,1), (1,0), (0,-1), (-1,0)];
    for direction in directions {
        let (target_row, target_col) = direction;
        let next_row = row as isize + target_row;
        let next_col = col as isize + target_col;

        if next_row < 0 || next_col < 0 {
            continue;
        };
        let new_row = next_row as usize;
        let new_col = next_col as usize;
        if new_row > (grid.len() - 1) || new_col > (grid[new_row].len() -1) {
            continue;
        }
        let is_land = grid[new_row][new_col] == '1';
        if is_land && !visited.contains(&(new_row, new_col)) {
            dfs(grid,visited, new_row, new_col);
        }
    }
}

// TODO: この関数を実装してください  
// グリッド全体を調べて島の数を数える
fn count_islands(grid: &Vec<Vec<char>>) -> usize {
    // ヒント:
    // 1. visitedを作成
    // 2. 全マスをループ
    // 3. 陸地('1')かつ未訪問なら:
    //    - dfsで島全体を探索
    //    - 島の数を+1
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut island_count = 0;
    // let directions =  [(1, 0), (0, 1), (-1, 0), (0, -1)];
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '1' && visited.contains(&(i,j)) == false {
                dfs(grid, &mut visited, i, j);
                island_count = island_count + 1;
            }
        }
    }
    
    island_count
}

fn main() {
    println!("=== DFS学習: 島の数を数える問題 ===\n");
    
    // テストケース1: 2つの島
    let grid1 = vec![
        vec!['1', '1', '0', '0', '0'],
        vec!['1', '1', '0', '0', '0'],
        vec!['0', '0', '1', '0', '0'],
        vec!['0', '0', '0', '1', '1'],
    ];
    
    println!("グリッド1:");
    for row in &grid1 {
        println!("{}", row.iter().collect::<String>());
    }

    let result1 = count_islands(&grid1);
    println!("島の数: {}\n", result1);


    
    // テストケース2: 3つの島
    let grid2 = vec![
        vec!['1', '0', '0', '1', '0'],
        vec!['0', '0', '0', '0', '0'],
        vec!['0', '1', '1', '0', '0'],
        vec!['0', '1', '0', '0', '1'],
    ];
    
    println!("グリッド2:");
    for row in &grid2 {
        println!("{}", row.iter().collect::<String>());
    }
    
    let result2 = count_islands(&grid2);
    println!("島の数: {}\n", result2);
    
    println!("\n問題説明:");
    println!("'1' = 陸地, '0' = 水");
    println!("隣接する陸地（上下左右）は同じ島とみなす");
    println!("斜めは隣接とみなさない");
    
    println!("\n実装のヒント:");
    println!("1. count_islands関数:");
    println!("   - 全マスをチェック");
    println!("   - 陸地かつ未訪問を見つけたら島カウント+1");
    println!("   - その陸地からDFSで島全体を訪問済みにする");
    println!("\n2. dfs関数:");
    println!("   - 現在地を訪問済みにする");
    println!("   - 4方向の隣接マスをチェック");
    println!("   - 陸地なら再帰的にDFS");
    
    println!("\nDFSの特徴:");
    println!("- 再帰を使って深く探索");
    println!("- BFSと違いキューは使わない");
    println!("- 1つの島を全部訪問してから次の島へ");
}