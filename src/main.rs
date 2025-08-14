use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

// ログエントリを表す構造体
#[derive(Debug)]
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
        // ここを実装してください
        todo!()
    }
    
    // HTTPステータスコードのカテゴリを返す（2xx, 3xx, 4xx, 5xx）
    fn status_category(&self) -> String {
        // ここを実装してください
        todo!()
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
    fn from_entries(entries: &[LogEntry]) -> LogAnalytics {
        // ヒント:
        // - HashMapやHashSetを活用
        // - IPアドレスの重複を除去してカウント
        // - パスごとのアクセス数を集計
        // - エラーリクエスト（4xx, 5xx）を収集
        
        // ここを実装してください
        todo!()
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
        // TODO: path_countsをソートして上位5件を表示
        
        println!("\n[エラーリクエスト数]: {}", self.error_requests.len());
    }
    
    // TODO: 最もアクセスの多いパスTOP Nを返す
    fn top_paths(&self, n: usize) -> Vec<(&String, &usize)> {
        // ヒント:
        // - HashMapをVecに変換してソート
        // - sort_by_key()やsort_by()を使用
        // - 上位N件を返す
        
        // ここを実装してください
        todo!()
    }
}

// TODO: ログファイルを解析する関数
fn analyze_log_file(path: &str) -> Result<LogAnalytics, Box<dyn std::error::Error>> {
    // ヒント:
    // - BufReaderで1行ずつ読み込み
    // - 各行をLogEntry::from_log_line()でパース
    // - パースに失敗した行は警告を出してスキップ
    // - 成功した行からLogAnalyticsを生成
    
    // ここを実装してください
    todo!()
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
    // 成功したら統計を表示
    // 失敗したらエラーメッセージを表示
    
    // ここを実装してください
    todo!()
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