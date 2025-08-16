use std::collections::{VecDeque, HashMap};

// TODO: BFSを使って、スタートから各マスへの最短距離を計算する関数
fn bfs_distance_map(grid: &Vec<Vec<char>>, start: (isize, isize)) -> HashMap<(usize, usize), usize> {
    // ヒント:
    // 1. キューとvisitedを準備
    // 2. スタート地点をキューに入れる
    // 3. BFSで探索しながら、各マスへの距離を記録
    // 4. 壁('#')は通れない、'.'と'S'と'T'は通れる
    // 5. 結果をHashMapで返す（座標 → 距離）
    let mut queue: VecDeque<((isize, isize), isize)> = VecDeque::new();
    let mut visited: HashMap<(isize, isize), bool> = HashMap::new();
    let mut distances = HashMap::new();
    let directions:[(i8, i8);4] = [(0,1),(1,0),(0,-1),(-1,0)];
    queue.push_back((start,0));
    visited.insert(start, true);
    while !queue.is_empty() {
        for direction in directions {
            let front = queue.pop_front().unwrap();
            let (row, col) = front as ((isize, isize), isize);
            let (next_r, next_c) = direction;
            

            
        }
    }

    
    // TODO: ここに実装
    
    distances
}

// TODO: 複数のターゲットの中で最も近いものを見つける関数
fn find_nearest_target(distances: &HashMap<(usize, usize), usize>, targets: &Vec<(usize, usize)>) -> Option<((usize, usize), usize)> {
    // ヒント:
    // 1. 各ターゲットについて、distancesから距離を取得
    // 2. 到達可能なターゲットの中で最小距離のものを返す
    // 3. どのターゲットにも到達できない場合はNone
    
    // TODO: ここに実装
    
    None
}

fn main() {
    println!("=== BFS学習: 最も近い宝物を探す問題 ===\n");
    
    // マップの定義
    // 'S' = スタート, 'T' = 宝物（ターゲット）, '#' = 壁, '.' = 通路
    let grid = vec![
        vec!['#', '#', '#', '#', '#', '#', '#', '#', '#'],
        vec!['#', 'S', '.', '.', '#', 'T', '.', '.', '#'],
        vec!['#', '#', '#', '.', '#', '.', '#', '.', '#'],
        vec!['#', 'T', '.', '.', '.', '.', '#', '.', '#'],
        vec!['#', '.', '#', '#', '#', '.', '#', '.', '#'],
        vec!['#', '.', '.', '.', '.', '.', '.', '.', '#'],
        vec!['#', '#', '#', '.', '#', '#', '#', 'T', '#'],
        vec!['#', '#', '#', '#', '#', '#', '#', '#', '#'],
    ];
    
    println!("マップ:");
    for row in &grid {
        println!("{}", row.iter().collect::<String>());
    }
    println!();
    
    // スタート地点とターゲット（宝物）の位置を探す
    let mut start = (0, 0);
    let mut targets = Vec::new();
    
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'S' {
                start = (i, j);
                println!("スタート: ({}, {})", i, j);
            } else if grid[i][j] == 'T' {
                targets.push((i, j));
                println!("宝物{}: ({}, {})", targets.len(), i, j);
            }
        }
    }
    println!();
    
    // TODO: 実装が完成したらコメントを外す
    // let distances = bfs_distance_map(&grid, start);
    // 
    // println!("各宝物への距離:");
    // for (idx, target) in targets.iter().enumerate() {
    //     match distances.get(target) {
    //         Some(dist) => println!("  宝物{}: 距離 {}", idx + 1, dist),
    //         None => println!("  宝物{}: 到達不可能", idx + 1),
    //     }
    // }
    // println!();
    // 
    // match find_nearest_target(&distances, &targets) {
    //     Some((pos, dist)) => {
    //         println!("最も近い宝物: ({}, {}) [距離: {}]", pos.0, pos.1, dist);
    //     },
    //     None => {
    //         println!("どの宝物にも到達できません");
    //     }
    // }
    
    println!("\n問題説明:");
    println!("- BFSで全マスへの最短距離を計算");
    println!("- 複数の宝物の中で最も近いものを見つける");
    println!("- これはPaizaでよく出る「最も近い目標を探す」パターン");
    
    println!("\n実装のヒント:");
    println!("1. bfs_distance_map:");
    println!("   - 通常のBFSだが、ゴールで止まらない");
    println!("   - 全ての到達可能なマスへの距離を計算");
    println!("   - HashMapに(座標, 距離)を保存");
    println!("\n2. find_nearest_target:");
    println!("   - targets.iter()でループ");
    println!("   - distances.get()で距離を取得");
    println!("   - 最小値を更新していく");
}