/*
問題1: 設定ファイルリーダー

【要件】
1. config.txt ファイルからkey=value形式の設定を読み込む
2. Config構造体に格納して表示
3. 適切なエラーハンドリングを実装

【学習ポイント】
- ファイル操作 (std::fs)
- Result型によるエラーハンドリング
- 文字列処理とパース
- 構造体とメソッド実装

【config.txtのサンプル形式】
host=localhost
port=8080
database=myapp_db
max_connections=100
debug_mode=true

【期待される出力】
設定ファイルを読み込み中...

設定内容:
ホスト: localhost
ポート: 8080
データベース: myapp_db
最大接続数: 100
デバッグモード: true
*/

use std::collections::HashMap;
use std::fs;

struct Config {
    host: String,
    port: u16,
    database: String,
    max_connections: u32,
    debug_mode: bool,
}

impl Config {
    fn from_file(path: &str) -> HashMap<String, String> {
        // TODO: ファイルを読み込む
        // ヒント: fs::read_to_string() を使用
        let file_contents = fs::read_to_string(path).unwrap_or_else(|e| {
            eprintln!("ファイル読み込みエラー: {}", e);
            String::new()
        });
        let lines = file_contents.lines();
        let mut key_value_pairs: HashMap<String, String> = HashMap::new();
        for line in lines {
            // 空行やコメント行をスキップ
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }
            // "=" で分割してハッシュマップを構築
            let parts: Vec<&str> = trimmed.split('=').map(str::trim).collect();
            let key = parts.get(0).map_or("", |s| *s).to_string();
            let value = parts.get(1).map_or("", |s| *s).to_string();
            if !key.is_empty() && !value.is_empty() {
                key_value_pairs.insert(key, value);
            }
        }
        key_value_pairs
    }
    fn new(
        host: String,
        port: u16,
        database: String,
        max_connections: u32,
        debug_mode: bool,
    ) -> Self {
        Config {
            host,
            port,
            database,
            max_connections,
            debug_mode,
        }
    }
    
    // 修正版: Result型を活用し、エラー処理を改善
    fn from_file_improved(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
        // 修正1: Result型を返すことで、エラーを呼び出し元で処理可能に
        let file_contents = fs::read_to_string(path)?; // 修正2: ?演算子でエラーを伝播
        
        let mut config_map: HashMap<String, String> = HashMap::new();
        
        for line in file_contents.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }
            
            // 修正3: splitn(2, '=')で最初の'='でのみ分割（値に'='が含まれる場合に対応）
            if let Some((key, value)) = trimmed.split_once('=') {
                // 修正4: split_once()を使用することで、Vecの生成を回避しメモリ効率を向上
                config_map.insert(key.trim().to_string(), value.trim().to_string());
            }
        }
        
        // 修正5: 直接Config構造体を生成して返す
        Ok(Config {
            host: config_map
                .get("host")
                .cloned()
                .unwrap_or_else(|| "localhost".to_string()),
            port: config_map
                .get("port")
                .and_then(|p| p.parse().ok())
                .unwrap_or(5432),
            database: config_map
                .get("database")
                .cloned()
                .unwrap_or_else(|| "mydb".to_string()),
            max_connections: config_map
                .get("max_connections")
                .and_then(|c| c.parse().ok())
                .unwrap_or(10),
            debug_mode: config_map
                .get("debug_mode")
                .map_or(false, |d| d == "true"),
        })
    }
    
    // 修正版: 構造体の表示メソッドを追加
    fn display(&self) {
        println!("設定内容:");
        println!("ホスト: {}", self.host);
        println!("ポート: {}", self.port);
        println!("データベース: {}", self.database);
        println!("最大接続数: {}", self.max_connections);
        println!("デバッグモード: {}", self.debug_mode);
    }
}

// 既存の実装（旧バージョン）
#[allow(dead_code)]
fn main_old() {
    println!("設定ファイルを読み込み中...\n");

    let config_match = Config::from_file("config.txt");
    if config_match.is_empty() {
        eprintln!("設定ファイルが空か、読み込みに失敗しました。");
        return
    }
    let config: Config = Config::new(
        config_match.get("host").cloned().unwrap_or("localhost".to_string()),
        config_match.get("port").and_then(|p| p.parse().ok()).unwrap_or(5432),
        config_match.get("database").cloned().unwrap_or("mydb".to_string()),
        config_match.get("max_connections").and_then(|c| c.parse().ok()).unwrap_or(10),
        config_match.get("debug_mode").map_or(false, |d| d == "true"),
    );
    println!("設定内容:");
    println!("ホスト: {}", config.host);
    println!("ポート: {}", config.port);
    println!("データベース: {}", config.database);
    println!("最大接続数: {}", config.max_connections);
    println!("デバッグモード: {}", config.debug_mode);
}

// 修正版の実装（改善バージョン）
fn main() {
    println!("設定ファイルを読み込み中...\n");
    
    // 修正6: Result型を使った適切なエラーハンドリング
    match Config::from_file_improved("config.txt") {
        Ok(config) => {
            // 修正7: 表示処理をメソッドに委譲（単一責任の原則）
            config.display();
        }
        Err(e) => {
            // 修正8: エラーの詳細情報を表示
            eprintln!("設定ファイルの読み込みに失敗しました: {}", e);
            std::process::exit(1);
        }
    }
}

// 主な修正点のまとめ：
// 1. Result型の活用: エラーを適切に伝播し、呼び出し元で処理
// 2. ?演算子の使用: unwrap_or_elseの代わりにエラー伝播
// 3. split_once()の使用: splitn(2, '=')相当の処理をより効率的に実装
// 4. メモリ効率の改善: 不要なVec生成を回避
// 5. 構造体の直接生成: HashMapを経由せず直接Config構造体を生成
// 6. 単一責任の原則: 表示処理を専用メソッドに分離
// 7. エラー処理の改善: 詳細なエラー情報の表示とプログラムの適切な終了