use std::collections::{VecDeque, HashMap};

// TODO: BFSを使って、スタートから各マスへの最短距離を計算する関数
fn bfs_distance_map(grid: &Vec<Vec<char>>, start: (isize, isize), target_len: usize) -> HashMap<(usize, usize), usize> {
    // ヒント:
    // 1. キューとvisitedを準備
    // 2. スタート地点をキューに入れる
    // 3. BFSで探索しながら、各マスへの距離を記録
    // 4. 壁('#')は通れない、'.'と'S'と'T'は通れる
    // 5. 結果をHashMapで返す（座標 → 距離）
    let mut queue = VecDeque::new();
    let mut visited: HashMap<(isize, isize), bool> = HashMap::new();
    let directions:[(isize, isize);4] = [(0,1),(1,0),(0,-1),(-1,0)];
    let mut result = HashMap::new();
    queue.push_back((start,0));
    visited.insert(start, true);
    while !queue.is_empty() {
        let (front, dis) = queue.pop_front().unwrap();
        if result.len() == target_len {
            break;
        };
        for direction in directions {
            // 修正: 不要な型キャストを削除（frontは既に(isize, isize)型）
            // let (row, col) = front as (isize, isize);  // 修正前: 不要な型キャスト
            let (row, col) = front;  // 修正後: frontは既に(isize, isize)型なので直接使用
            let (next_r, next_c) = direction;
            let target_row = row + next_r;
            let target_col = col + next_c;

            if visited.contains_key(&(target_row, target_col)) {
                continue;
            }

            if target_row < 0 || target_col < 0 {
                continue;
            };

            if (grid.len() - 1) < target_row as usize || (grid[target_row as usize ].len() - 1) < target_col as usize {
                continue;
            }

            if grid[target_row as usize][target_col as usize] == '#' {
                continue;
            } 


            if grid[target_row as usize][target_col as usize] == 'T' {
                result.insert(((target_row  as usize, target_col as usize)), dis + 1);
            }

            visited.insert((target_row, target_col), true);
            queue.push_back(((target_row, target_col), dis + 1));

        };
    };
    return result;
    
}

// TODO: 複数のターゲットの中で最も近いものを見つける関数
fn find_nearest_target(distances: &HashMap<(usize, usize), usize>, targets: &Vec<(usize, usize)>) -> Option<((usize, usize), usize)> {
    // ヒント:
    // 1. 各ターゲットについて、distancesから距離を取得
    // 2. 到達可能なターゲットの中で最小距離のものを返す
    // 3. どのターゲットにも到達できない場合はNone
    
    // ========== 修正前のコード ==========
    // let mut a: &usize = &usize::MAX;
    // let mut current_target: (usize, usize) = (0,0);
    // for target in targets {
    //     if a > distances.get(target).unwrap() {  // 問題: unwrap()でパニックの可能性
    //         a = distances.get(target).unwrap();
    //         current_target = *target;
    //     };
    // }
    // if a < &usize::MAX {
    //     let current_dis = *a;
    //     println!("{:?}{}",current_target, current_dis);  // デバッグ用出力
    //     return Some((current_target, current_dis))
    // }
    // None
    // =====================================
    
    // ========== 修正後のコード ==========
    // 修正点1: より安全で明確な実装に変更
    let mut min_distance = usize::MAX;
    let mut nearest_target = None;
    
    for target in targets {
        // 修正点2: unwrap()を使わずif letパターンで安全にチェック
        // 到達不可能なターゲットの場合はスキップ（パニックを防ぐ）
        if let Some(&distance) = distances.get(target) {
            if distance < min_distance {
                min_distance = distance;
                nearest_target = Some((*target, distance));
            }
        }
    }
    
    // 修正点3: デバッグ用printlnを削除し、シンプルなreturnに変更
    nearest_target
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
    let mut start: (isize, isize) = (0, 0);
    let mut targets = Vec::new();
    
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'S' {
                start = (i as isize, j as isize);
                println!("スタート: ({}, {})", i, j);
            } else if grid[i][j] == 'T' {
                targets.push((i, j));
                println!("宝物{}: ({}, {})", targets.len(), i, j);
            }
        }
    }
    println!();
    
    // TODO: 実装が完成したらコメントを外す
    let distances = bfs_distance_map(&grid, start, targets.len());
    println!("{:?}",distances);
    // 
    println!("各宝物への距離:");
    for (idx, target) in targets.iter().enumerate() {
        match distances.get(target) {
            Some(dist) => println!("  宝物{}: 距離 {}", idx + 1, dist),
            None => println!("  宝物{}: 到達不可能", idx + 1),
        }
    }
    println!();
    // 
    match find_nearest_target(&distances, &targets) {
        Some((pos, dist)) => {
            println!("最も近い宝物: ({}, {}) [距離: {}]", pos.0, pos.1, dist);
        },
        None => {
            println!("どの宝物にも到達できません");
        }
    }
    
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