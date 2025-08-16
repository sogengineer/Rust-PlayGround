use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::Arc;

// ログエントリを表す構造体
#[derive(Debug, Clone)]
struct LogEntry {
    ip_address: String,
    timestamp: String,
    method: String,
    path: String,
    status_code: u16,
    response_size: u64,
    user_agent: String,
}

impl LogEntry {
    // TODO: ログ行をパースしてLogEntryを生成
    // ヒント:
    // - 正規表現を使うか、文字列を順次パース
    // - Apache/Nginx形式のログをパース
    // - 例: "192.168.1.1 - - [10/Oct/2024:13:55:36 +0000] \"GET /api/users HTTP/1.1\" 200 1234 \"Mozilla/5.0...\""
    fn from_log_line(line: &str) -> Result<LogEntry, String> {
        // IPアドレスを取得
        let ip_end = line.find(' ').ok_or("IPアドレスが見つかりません")?;
        let ip_address = line[..ip_end].to_string();
        
        // タイムスタンプを取得 [...]の部分
        let timestamp_start = line.find('[').ok_or("タイムスタンプが見つかりません")?;
        let timestamp_end = line.find(']').ok_or("タイムスタンプの終端が見つかりません")?;
        let timestamp = line[timestamp_start + 1..timestamp_end].to_string();
        
        // リクエスト部分 "GET /path HTTP/1.1" を取得
        let request_start = line.find('"').ok_or("リクエストが見つかりません")?;
        let request_end = line[request_start + 1..].find('"')
            .ok_or("リクエストの終端が見つかりません")? + request_start + 1;
        let request = &line[request_start + 1..request_end];
        
        // リクエストをメソッド、パス、プロトコルに分割
        let request_parts: Vec<&str> = request.split_whitespace().collect();
        if request_parts.len() != 3 {
            return Err("リクエストの形式が不正です".into());
        }
        let method = request_parts[0].to_string();
        let path = request_parts[1].to_string();
        
        // リクエスト後の残りの部分をパース
        let after_request = &line[request_end + 1..];
        let remaining_parts: Vec<&str> = after_request.split_whitespace().collect();
        if remaining_parts.len() < 2 {
            return Err("ステータスコードとレスポンスサイズが見つかりません".into());
        }
        
        let status_code = remaining_parts[0].parse()
            .map_err(|_| "ステータスコードのパースに失敗しました")?;
        let response_size = remaining_parts[1].parse()
            .map_err(|_| "レスポンスサイズのパースに失敗しました")?;
        
        // User-Agentを取得（最後の引用符で囲まれた部分）
        let ua_start = after_request.find('"').unwrap_or(after_request.len());
        let user_agent = if ua_start < after_request.len() {
            let ua_end = after_request[ua_start + 1..].find('"')
                .unwrap_or(after_request.len() - ua_start - 1) + ua_start + 1;
            after_request[ua_start + 1..ua_end].to_string()
        } else {
            String::new()
        };

        Ok(LogEntry {
            ip_address,
            timestamp,
            method,
            path,
            status_code,
            response_size,
            user_agent,
        })
    }
    
    // HTTPステータスコードのカテゴリを返す（2xx, 3xx, 4xx, 5xx）
    fn status_category(&self) -> String {
        let category = match self.status_code {
            200..=299 => "2xx".into(),
            300..=399 => "3xx".into(),
            400..=499 => "4xx".into(),
            500..=599 => "5xx".into(),
            _ => "unknown".into(),
        };
        category
    }
}

// 解析結果を保持する構造体
#[derive(Debug)]
struct LogAnalytics {
    total_requests: usize,
    unique_ips: usize,
    status_counts: HashMap<String, usize>,  // "2xx", "3xx", "4xx", "5xx"
    path_counts: HashMap<String, usize>,    // パスごとのアクセス数
    total_bytes: u64,
    error_requests: Vec<LogEntry>,          // 4xx, 5xxのリクエスト
}

impl LogAnalytics {
    fn new() -> Self {
        LogAnalytics {
            total_requests: 0,
            unique_ips: 0,
            status_counts: HashMap::new(),
            path_counts: HashMap::new(),
            total_bytes: 0,
            error_requests: Vec::new(),
        }
    }
    
    // TODO: ログエントリのリストから統計を計算
    fn from_entries(mut analytics: LogAnalytics, entries: &[LogEntry]) -> LogAnalytics {
        // ヒント:
        // - HashMapやHashSetを活用
        // - IPアドレスの重複を除去してカウント
        // - パスごとのアクセス数を集計
        // - エラーリクエスト（4xx, 5xx）を収集
        let mut unique_ips = std::collections::HashSet::new();
        
        for entry in entries {
            // すべてのリクエストを処理
            analytics.total_requests += 1;
            
            // IPアドレスをHashSetに追加（重複は自動的に除去される）
            unique_ips.insert(entry.ip_address.clone());
            
            // ステータスコードのカテゴリ別カウント
            *analytics.status_counts.entry(entry.status_category()).or_insert(0) += 1;
            
            // パスごとのアクセス数をカウント（パスのみ、メソッドは含めない）
            *analytics.path_counts.entry(entry.path.clone()).or_insert(0) += 1;
            
            // レスポンスサイズの合計
            analytics.total_bytes += entry.response_size;
            
            // エラーリクエスト（4xx, 5xx）を収集
            if entry.status_code >= 400 {
                analytics.error_requests.push(entry.clone());
            }
        }
        
        // ユニークIP数を設定
        analytics.unique_ips = unique_ips.len();
        
        analytics
    }
    
    // 統計情報を表示
    fn display(&self) {
        println!("=== ログ解析結果 ===");
        println!("総リクエスト数: {}", self.total_requests);
        println!("ユニークIP数: {}", self.unique_ips);
        println!("総転送バイト数: {} bytes", self.total_bytes);
        
        println!("\n[ステータスコード分布]");
        for (status, count) in &self.status_counts {
            println!("{}: {} requests", status, count);
        }
        
        println!("\n[人気のパス TOP 5]");
        // TODO: top_paths()を呼び出して表示
        let top_paths = self.top_paths(5);
        for (path, count) in top_paths {
            println!("{}: {} requests", path, count);
        }
        
        println!("\n[エラーリクエスト数]: {}", self.error_requests.len());

    }
    
    // TODO: 最もアクセスの多いパスTOP Nを返す
    fn top_paths(&self, n: usize) -> Vec<(&String, &usize)> {
        // ヒント:
        // - HashMapをVecに変換してソート
        let mut path_counts: Vec<(&String, &usize)> = self.path_counts.iter().collect();
        path_counts.sort_by(|a, b| b.1.cmp(a.1));
        path_counts.truncate(n);
        // - sort_by_key()やsort_by()を使用
        // - 上位N件を返す
        path_counts
    }
}

// TODO: ログファイルを解析する関数
fn analyze_log_file(path: &str) -> Result<LogAnalytics, Box<dyn std::error::Error>> {
    // ヒント:
    // - BufReaderでログファイルを1行ずつ読み込み
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // - 各行をLogEntry::from_log_line()でパース
    // - パースに失敗した行は警告を出してスキップ
    // - 成功した行からLogAnalyticsを生成
    let mut entries = Vec::new();
    
    for line in reader.lines() {
        match line {
            Ok(log_line) => {
                match LogEntry::from_log_line(&log_line) {
                    Ok(entry) => {
                        // 成功した場合はエントリをVecに追加
                        entries.push(entry);
                    }
                    Err(e) => {
                        eprintln!("ログ行のパースに失敗しました: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("行の読み込みに失敗しました: {}", e);
            }
        }
    }
    
    // すべてのエントリを一度に処理
    let analytics = LogAnalytics::new();
    Ok(LogAnalytics::from_entries(analytics, &entries))
}

// 時間帯別の集計を行う構造体（拡張課題）
#[derive(Debug)]
struct HourlyStats {
    hour_counts: HashMap<u8, usize>,  // 0-23時の時間帯別アクセス数
}

impl HourlyStats {
    // TODO: タイムスタンプから時間を抽出して集計
    fn from_entries(entries: &[LogEntry]) -> HourlyStats {
        // ヒント:
        // - timestamp文字列から時間部分を抽出
        // - "[10/Oct/2024:13:55:36 +0000]" → 13時
        
        // ここを実装してください（オプション）
        todo!()
    }
}

fn main() {
    println!("Webサーバーログファイルを解析中...\n");
    
    // TODO: analyze_log_file()を呼び出し、
    let analytics: Result<LogAnalytics, Box<dyn std::error::Error>> = analyze_log_file("access.log");
    // 成功したら統計を表示
    // 失敗したらエラーメッセージを表示
    match analytics {
        Ok(data) => {
            data.display();
        }
        Err(e) => {
            eprintln!("ログファイルの解析に失敗しました: {}", e);
        }
    }
}

/*
問題3: Webサーバーログの解析と集計

【要件】
1. access.log ファイルからWebサーバーのアクセスログを読み込む
2. 各種統計情報を計算（総リクエスト数、ユニークIP数、ステータスコード分布など）
3. 人気のパスTOP5を表示
4. エラーリクエスト（4xx, 5xx）を抽出

【学習ポイント】
- 複雑な文字列パース（正規表現or手動パース）
- HashMapとHashSetの活用
- ソートとランキング処理
- より実践的なエラーハンドリング
- 大規模ログファイルの効率的な処理

【access.logのサンプル形式（Apache/Nginx形式）】
192.168.1.1 - - [10/Oct/2024:13:55:36 +0000] "GET /api/users HTTP/1.1" 200 1234 "Mozilla/5.0..."
192.168.1.2 - - [10/Oct/2024:13:55:37 +0000] "POST /api/login HTTP/1.1" 401 567 "Mozilla/5.0..."
...

形式: IPアドレス - - [タイムスタンプ] "メソッド パス プロトコル" ステータスコード レスポンスサイズ "User-Agent"

【ヒント】
- split()やfind()を組み合わせて段階的にパース
- 正規表現を使う場合は regex クレートが便利（ただし今回は標準ライブラリで実装可能）
- HashSetでユニークIPをカウント
- Vec<(K, V)>に変換してからソート

【拡張課題】
- 時間帯別のアクセス数を集計
- 特定のIPアドレスからの過剰アクセスを検出
- レスポンスタイムの統計（ログに含まれる場合）
*/