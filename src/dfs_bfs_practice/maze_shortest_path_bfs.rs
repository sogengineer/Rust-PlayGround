use std::collections::{VecDeque, HashMap};

fn main() {
    println!("=== DFS/BFS学習: グリッド上の迷路探索 ===\n");
    
    // 迷路の定義
    // '#' = 壁, '.' = 通路, 'S' = スタート, 'G' = ゴール
    let maze = vec![
        vec!['S', '.', '#', '.', '.'],
        vec!['.', '.', '#', '.', '#'],
        vec!['.', '#', '.', '.', '.'],
        vec!['.', '.', '.', '#', '.'],
        vec!['#', '.', '.', '.', 'G'],
    ];
    
    println!("迷路:");
    for row in &maze {
        println!("{}", row.iter().collect::<String>());
    }
    println!();
    
    // ステップ1: スタートとゴールの位置を見つける
    let mut start = (0, 0);
    let mut goal = (0, 0);
    
    for i in 0..maze.len() {
        for j in 0..maze[0].len() {
            if maze[i][j] == 'S' {
                start = (i, j);
                println!("スタート位置: ({}, {})", i, j);
            }
            if maze[i][j] == 'G' {
                goal = (i, j);
                println!("ゴール位置: ({}, {})", i, j);
            }
        }
    }
    println!();
    
    // ステップ2: BFSに必要な準備
    // キュー: 次に調べる場所を入れておく
    let mut queue = VecDeque::new();
    
    // 訪問済みフラグ: 同じ場所を2度調べないようにする
    let mut visited: HashMap<(usize, usize), bool> = HashMap::new();
    
    // 4方向への移動（下、右、上、左）
    // isizeを使うことで、負の値も扱える
    let directions: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    
    // ステップ3: BFSの開始
    // スタート地点をキューに入れる（位置と距離）
    queue.push_back((start, 0));
    visited.insert(start, true);  // HashMapに訪問済みとして記録
    
    println!("BFSを開始します...");
    println!("探索の様子:");
    
    // BFSのメインループ
    let mut answer = None;  // 答えを保存する変数
    
    while !queue.is_empty() {
        // ステップ1: キューから1つ取り出す
        // キューには ((行, 列), 距離) が入っている
        let (current_position, distance) = queue.pop_front().unwrap();
        let (row, col) = current_position;  // 現在位置を分解
        
        println!("  現在地: ({}, {}) 距離: {}", row, col, distance);
        
        // ステップ2: ゴールに到着したかチェック
        if current_position == goal {
            println!("    → ゴール発見！");
            answer = Some(distance);
            break;  // ゴールに着いたので終了
        }
        
        // ステップ3: 4方向を調べる
        for (dr, dc) in directions.iter() {
            // 新しい座標を計算
            // usizeは0以上の値しか扱えないので、一旦isizeで計算
            let new_row = row as isize + dr;
            let new_col = col as isize + dc;
            
            // チェック1: 迷路の範囲内か？
            if new_row < 0 || new_row >= maze.len() as isize {
                continue;  // 範囲外なのでスキップ
            }
            if new_col < 0 || new_col >= maze[0].len() as isize {
                continue;  // 範囲外なのでスキップ
            }
            
            // isizeからusizeに安全に変換（ここまで来れば必ず0以上）
            let new_row = new_row as usize;
            let new_col = new_col as usize;
            
            // チェック2: 壁じゃないか？
            if maze[new_row][new_col] == '#' {
                continue;  // 壁なのでスキップ
            }
            
            // チェック3: すでに訪問済みじゃないか？
            if visited.contains_key(&(new_row, new_col)) {
                continue;  // 訪問済みなのでスキップ
            }
            
            // ここまで来たら、行ける場所！
            println!("    → ({}, {}) へ移動可能", new_row, new_col);
            
            // 訪問済みにする
            visited.insert((new_row, new_col), true);
            
            // キューに追加（距離は+1）
            queue.push_back(((new_row, new_col), distance + 1));
        }
    }
    
    // 結果を表示
    println!();
    match answer {
        Some(dist) => println!("最短距離: {}", dist),
        None => println!("ゴールに到達できません"),
    }
}