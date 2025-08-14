use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};

// 学生の成績データを表す構造体
#[derive(Debug)]
struct Student {
    _name: String,
    math: u8,
    english: u8,
    science: u8,
}

impl Student {
    // TODO: CSVの1行から学生データを生成
    // ヒント: 
    // - String型のlineを受け取り、Result<Student, String>を返す
    // - split(',')でCSVを分割
    // - parse::<u32>()で文字列を数値に変換
    fn from_csv_line(line: &str) -> Result<Student, String> {
        let parts: Vec<&str> = line.split(',').map(str::trim).collect();
        if parts.len() != 4 {
            return Err(format!("Invalid CSV line: {}", line));
        }
        
        let name = parts[0].to_string();
        // 数値のパース時にエラーが発生した場合は適切なエラーメッセージを返す
        let math = parts[1].parse::<u8>().map_err(|e| format!("Math parse error: {}", e))?;
        let english = parts[2].parse::<u8>().map_err(|e| format!("English parse error: {}", e))?;
        let science = parts[3].parse::<u8>().map_err(|e| format!("Science parse error: {}", e))?;
        
        // どれかでも100を超える場合はエラー
        if math > 100 || english > 100 || science > 100 {
            return Err(format!("Scores must be between 0 and 100: {}", line));
        }
        Ok(Student { _name: name, math, english, science })
    }

    
    // TODO: 3教科の合計点を計算
    fn total(&self) -> u16 {
        return self.math as u16 + self.english as u16 + self.science as u16;
    }
}

// 成績統計を表す構造体
#[derive(Debug)]
struct Statistics {
    total_students: usize,
    average_math: f64,
    average_english: f64,
    average_science: f64,
    highest_total: u16,
    lowest_total: u16,
}

impl Statistics {
    // TODO: 学生のリストから統計情報を計算
    fn from_students(students: &[Student]) -> Option<Statistics> {
        // ヒント:
        // - studentsが空の場合はNoneを返す
        // - iter()やmap()、sum()を活用
        // - max_by_key()やmin_by_key()で最高/最低点を取得
        if students.is_empty() {
            return None
        };
        let total_students = students.len();
        let (highest_total, lowest_total, average_math, average_english, average_science) = students.iter().fold(
            (0, u16::MAX, 0.0, 0.0, 0.0),
            // ここで各学生の合計点と平均点を計算
            |(highest, lowest, sum_math, sum_english, sum_science), student| {
                // 
                let total = student.total();
                let math = student.math as f64;
                let english = student.english as f64;
                let science = student.science as f64;
                // 合計点と平均点を更新
                (
                    highest.max(total),
                    lowest.min(total),
                    sum_math + math,
                    sum_english + english,
                    sum_science + science,
                )
            },
        );
        return Some(Statistics {
            total_students,
            average_math: average_math / total_students as f64,
            average_english: average_english / total_students as f64,
            average_science: average_science / total_students as f64,
            highest_total,
            lowest_total: if lowest_total == u16::MAX { 0 } else { lowest_total },
        })
    }
    
    // 統計情報を表示
    fn display(&self) {
        println!("=== 成績統計 ===");
        println!("学生数: {}", self.total_students);
        println!("数学平均点: {:.1}", self.average_math);
        println!("英語平均点: {:.1}", self.average_english);
        println!("理科平均点: {:.1}", self.average_science);
        println!("最高合計点: {}", self.highest_total);
        println!("最低合計点: {}", self.lowest_total);
    }
}

// TODO: CSVファイルから学生データを読み込む関数
// ヒント:
// - Result<Vec<Student>, Box<dyn std::error::Error>>を返す
// - ファイルの最初の行（ヘッダー）はスキップ
// - 各行でStudent::from_csv_line()を呼び出し
// - エラーが発生した行はスキップ（または詳細なエラーメッセージを返す）
fn load_students_from_csv(path: &str) -> Result<Vec<Student>, Box<dyn std::error::Error>> {
    // 各行の読み込みとパース処理を実装
    let content = fs::read_to_string(path)?;
    let mut students = Vec::new();
    for line in content.lines().skip(1) {
        match Student::from_csv_line(line) {
            Ok(student) => {
                students.push(student);
            }
            Err(e) => {
                // エラー処理: パース失敗の行を表示
                eprintln!("Error parsing line '{}': {}", line, e);
                continue; // エラーが発生した行はスキップ
            }
        }
    }
    Ok(students)
}

// 既存の実装（ユーザー作成版）
#[allow(dead_code)]
fn main_old() {
    println!("CSVファイルから成績データを読み込み中...\n");
    
    // load_students_from_csv()を呼び出し
    let students = match load_students_from_csv("grades.csv") {
        Ok(students) => students,
        Err(e) => {
            eprintln!("Error loading students: {}", e);
            return;
        }
    };

    // 統計構造体を生成
    let stats = match Statistics::from_students(&students) {
        Some(stats) => stats,
        None => {
            println!("学生データが空です。統計情報を計算できません。");
            return;
        }
    };

    // 統計情報を表示
    stats.display();
}

// ===== メモリ効率改善版 =====

// 修正1: Box<str>を使用してメモリ効率を改善
#[derive(Debug)]
struct StudentOptimized {
    name: Box<str>,  // 修正1: String(24バイト) → Box<str>(16バイト)
    math: u8,
    english: u8,
    science: u8,
}

impl StudentOptimized {
    // 修正2: 静的エラーメッセージとイテレータの直接使用
    fn from_csv_line(line: &str) -> Result<Self, &'static str> {
        // 修正3: Vec<&str>を作らず、イテレータを直接使用
        let mut parts = line.split(',').map(str::trim);
        
        let name = parts.next()
            .ok_or("Missing name field")?
            .to_string()
            .into_boxed_str();  // 修正4: 余分なcapacityを持たないBox<str>に変換
        
        // 修正5: format!によるアロケーションを避け、静的エラーメッセージを使用
        let math = parts.next()
            .ok_or("Missing math score")?
            .parse::<u8>()
            .map_err(|_| "Invalid math score")?;
            
        let english = parts.next()
            .ok_or("Missing english score")?
            .parse::<u8>()
            .map_err(|_| "Invalid english score")?;
            
        let science = parts.next()
            .ok_or("Missing science score")?
            .parse::<u8>()
            .map_err(|_| "Invalid science score")?;
        
        if math > 100 || english > 100 || science > 100 {
            return Err("Score exceeds 100");
        }
        
        Ok(StudentOptimized { name, math, english, science })
    }
    
    #[inline]  // 修正6: 小さな関数をインライン化
    fn total(&self) -> u16 {
        self.math as u16 + self.english as u16 + self.science as u16
    }
}

// 修正7: ストリーミング統計計算（全データをメモリに保持しない）
struct StreamingStatistics {
    count: usize,
    sum_math: u64,
    sum_english: u64,
    sum_science: u64,
    highest_total: u16,
    lowest_total: u16,
}

impl StreamingStatistics {
    fn new() -> Self {
        StreamingStatistics {
            count: 0,
            sum_math: 0,
            sum_english: 0,
            sum_science: 0,
            highest_total: 0,
            lowest_total: u16::MAX,
        }
    }
    
    // 修正8: 1件ずつ処理してメモリ使用量を最小化
    fn add_student(&mut self, student: &StudentOptimized) {
        self.count += 1;
        self.sum_math += student.math as u64;
        self.sum_english += student.english as u64;
        self.sum_science += student.science as u64;
        
        let total = student.total();
        self.highest_total = self.highest_total.max(total);
        self.lowest_total = self.lowest_total.min(total);
    }
    
    fn display(&self) {
        if self.count == 0 {
            println!("データがありません");
            return;
        }
        
        println!("=== 成績統計（メモリ最適化版） ===");
        println!("学生数: {}", self.count);
        println!("数学平均点: {:.1}", self.sum_math as f64 / self.count as f64);
        println!("英語平均点: {:.1}", self.sum_english as f64 / self.count as f64);
        println!("理科平均点: {:.1}", self.sum_science as f64 / self.count as f64);
        println!("最高合計点: {}", self.highest_total);
        println!("最低合計点: {}", if self.lowest_total == u16::MAX { 0 } else { self.lowest_total });
    }
}

// 修正9: BufReaderでファイルをストリーミング読み込み（大規模ファイル対応）
fn process_csv_streaming(path: &str) -> std::io::Result<StreamingStatistics> {
    let file = File::open(path)?;
    // 修正10: BufReaderで行単位の読み込み（メモリ効率的）
    let reader = BufReader::new(file);
    let mut stats = StreamingStatistics::new();
    
    // 修正11: lines()イテレータで逐次処理（全体をメモリに読み込まない）
    for (index, line) in reader.lines().enumerate() {
        if index == 0 {
            continue; // ヘッダーをスキップ
        }
        
        let line = line?;
        match StudentOptimized::from_csv_line(&line) {
            Ok(student) => {
                stats.add_student(&student);
                // 修正12: studentは即座にスコープを抜けるので、メモリが解放される
            },
            Err(e) => eprintln!("Line {}: {}", index + 1, e),
        }
    }
    
    Ok(stats)
}

// メモリ効率改善版のメイン関数
fn main() {
    println!("メモリ効率改善版：CSVストリーミング処理\n");
    
    match process_csv_streaming("grades.csv") {
        Ok(stats) => stats.display(),
        Err(e) => eprintln!("ファイル読み込みエラー: {}", e),
    }
}

/* 
主なメモリ効率の改善点：

1. **String → Box<str>** (24バイト → 16バイト)
   - Stringは ptr + len + capacity の3つのフィールド
   - Box<str>は ptr + len の2つのフィールドのみ

2. **Vec<&str>の生成回避**
   - split().collect() を使わず、イテレータを直接処理
   - 一時的なVecのアロケーションを削減

3. **format!マクロの削減**
   - 動的な文字列生成を避け、静的エラーメッセージを使用
   - ヒープアロケーションの回数を削減

4. **BufReaderによるストリーミング処理**
   - ファイル全体をメモリに読み込まない
   - 大規模CSV（数GB）でも一定のメモリ使用量

5. **逐次統計計算**
   - 全学生データを保持せず、統計情報のみを更新
   - メモリ使用量がO(n) → O(1)に改善

これらの最適化により、特に大規模CSVファイルで
メモリ使用量を大幅に削減できます。
*/

/* 
問題2: CSVファイル読み込みと統計処理

【要件】
1. grades.csv ファイルから学生の成績データを読み込む
2. 各学生の3教科の合計点と平均点を計算
3. 全体の統計情報を計算して表示
4. 適切なエラーハンドリングを実装

【学習ポイント】
- CSV形式のパース処理
- イテレータとクロージャの活用  
- Option型とResult型の使い分け
- 統計処理の実装
- ファイル読み込みのエラーハンドリング

【grades.csvのサンプル形式】
名前,数学,英語,理科
田中太郎,85,72,90
山田花子,78,88,65
...

【ヒント】
- lines()とskip(1)でヘッダーをスキップ
- filter_map()で正常にパースできた行だけを処理
- collect::<Result<Vec<_>, _>>()でエラーを集約
*/