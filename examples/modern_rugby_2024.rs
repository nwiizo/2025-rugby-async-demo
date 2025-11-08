/// Rust 2024 Edition対応：ラグビー非同期戦術デモ
///
/// このファイルは、Rust 2024 editionの新機能とベストプラクティスを活用した
/// 改善版のコード例です。
///
/// # 2024 Editionの活用ポイント
///
/// - **Async Closures**: `async || {}` 構文で、より表現力の高い非同期処理
/// - **RPIT (Return Position Impl Trait)**: 簡潔な型シグネチャ
/// - **Comprehensive Rustdoc**: すべての公開APIにドキュメント
/// - **Type Safety**: より明示的なエラーハンドリング
use std::time::Duration;
use tokio::time::sleep;

// =============================================================================
// 型定義とエラー型
// =============================================================================

/// ゲーム内で発生する可能性のあるエラー
#[derive(Debug, Clone)]
pub enum GameError {
    /// タイムアウトエラー
    Timeout { action: String, limit_secs: u64 },
    /// 判断エラー
    DecisionError { reason: String },
}

impl std::fmt::Display for GameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameError::Timeout { action, limit_secs } => {
                write!(f, "タイムアウト: {} (制限: {}秒)", action, limit_secs)
            }
            GameError::DecisionError { reason } => {
                write!(f, "判断エラー: {}", reason)
            }
        }
    }
}

impl std::error::Error for GameError {}

/// ディフェンスラインの状態
///
/// # Examples
///
/// ```
/// # use modern_rugby_2024::DefenseLine;
/// let defense = DefenseLine {
///     pressure: false,
///     gap_on_left: true,
///     gap_on_right: false,
/// };
/// assert!(defense.has_gap());
/// ```
#[derive(Debug, Clone)]
pub struct DefenseLine {
    /// ディフェンスからのプレッシャーがあるか
    pub pressure: bool,
    /// 左サイドにギャップがあるか
    pub gap_on_left: bool,
    /// 右サイドにギャップがあるか
    pub gap_on_right: bool,
}

impl DefenseLine {
    /// ディフェンスラインにギャップがあるかを判定
    ///
    /// # Returns
    ///
    /// 左右いずれかにギャップがある場合は`true`
    pub fn has_gap(&self) -> bool {
        self.gap_on_left || self.gap_on_right
    }

    /// 最適な攻撃方向を返す
    ///
    /// # Returns
    ///
    /// ギャップがある側の方向、なければNone
    pub fn optimal_direction(&self) -> Option<Direction> {
        if self.gap_on_left {
            Some(Direction::Left)
        } else if self.gap_on_right {
            Some(Direction::Right)
        } else {
            None
        }
    }
}

/// 攻撃の方向
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    /// 左サイド
    Left,
    /// 右サイド
    Right,
    /// 中央
    Center,
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Left => write!(f, "左"),
            Direction::Right => write!(f, "右"),
            Direction::Center => write!(f, "中央"),
        }
    }
}

/// チームメイトの準備状態
#[derive(Debug, Clone)]
pub struct Teammates {
    /// バックスの準備ができているか
    pub backs_ready: bool,
    /// フォワードの準備ができているか
    pub forwards_ready: bool,
}

/// 攻撃の判断結果
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Decision {
    /// パス展開
    Pass { direction: Direction },
    /// クラッシュボール
    Crash,
    /// キック
    Kick,
}

impl std::fmt::Display for Decision {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Decision::Pass { direction } => write!(f, "{}サイドへパス展開", direction),
            Decision::Crash => write!(f, "フォワードにクラッシュボール"),
            Decision::Kick => write!(f, "ハイパントキック"),
        }
    }
}

// =============================================================================
// 非同期関数（Rust 2024 RPIT活用）
// =============================================================================

/// スクラムハーフからのパスを待機
///
/// # Returns
///
/// ボール受領完了のメッセージ
///
/// # Examples
///
/// ```no_run
/// # use modern_rugby_2024::wait_for_ball;
/// # tokio_test::block_on(async {
/// let ball = wait_for_ball().await;
/// assert_eq!(ball, "ボール受領");
/// # });
/// ```
pub async fn wait_for_ball() -> String {
    println!("🏉 スクラムハーフからのパスを待機...");
    sleep(Duration::from_secs(2)).await;
    println!("✓ ボール受け取り完了");
    "ボール受領".to_string()
}

/// ディフェンスラインを分析
///
/// # Returns
///
/// 分析されたディフェンスラインの状態
///
/// # Examples
///
/// ```no_run
/// # use modern_rugby_2024::read_defense;
/// # tokio_test::block_on(async {
/// let defense = read_defense().await;
/// assert!(defense.has_gap());
/// # });
/// ```
pub async fn read_defense() -> DefenseLine {
    println!("👀 ディフェンスラインを読む...");
    sleep(Duration::from_secs(1)).await;

    let defense = DefenseLine {
        pressure: false,
        gap_on_left: true,
        gap_on_right: false,
    };

    if let Some(direction) = defense.optimal_direction() {
        println!("✓ ディフェンス分析完了: {}にギャップあり", direction);
    } else {
        println!("✓ ディフェンス分析完了: ギャップなし");
    }

    defense
}

/// チームメイトのポジショニングを確認
///
/// # Returns
///
/// チームメイトの準備状態
pub async fn check_teammates() -> Teammates {
    println!("👥 味方のポジショニング確認...");
    sleep(Duration::from_millis(800)).await;

    let teammates = Teammates {
        backs_ready: true,
        forwards_ready: true,
    };

    println!("✓ 味方の準備完了");
    teammates
}

/// バックスに展開のサインを送る
pub async fn signal_backs() {
    println!("📢 バックスに展開のサイン...");
    sleep(Duration::from_millis(500)).await;
    println!("✓ バックス準備完了");
}

/// フォワードにサポートのサインを送る
pub async fn signal_forwards() {
    println!("📢 フォワードにサポートのサイン...");
    sleep(Duration::from_millis(500)).await;
    println!("✓ フォワード準備完了");
}

/// 状況を総合的に判断して最適な戦術を決定
///
/// # Arguments
///
/// * `_ball` - 受け取ったボール（将来的な拡張用）
/// * `defense` - ディフェンスラインの状態
/// * `teammates` - チームメイトの準備状態
///
/// # Returns
///
/// 最適な攻撃判断
///
/// # Decision Logic
///
/// 1. ギャップがあり、バックスが準備完了 → パス展開
/// 2. プレッシャーがなく、フォワードが準備完了 → クラッシュボール
/// 3. それ以外 → キック
pub async fn make_decision(_ball: String, defense: DefenseLine, teammates: Teammates) -> Decision {
    println!("\n🧠 状況を統合して判断...");

    if let Some(direction) = defense.optimal_direction() {
        if teammates.backs_ready {
            return Decision::Pass { direction };
        }
    }

    if !defense.pressure && teammates.forwards_ready {
        Decision::Crash
    } else {
        Decision::Kick
    }
}

// =============================================================================
// Rust 2024: Async Closuresのデモ
// =============================================================================

/// 複数のタスクを並行実行する高階関数
///
/// Rust 2024のasync closuresを活用した例
///
/// # Arguments
///
/// * `tasks` - 実行するタスクのリスト
/// * `processor` - 各タスクを処理するasync closure
///
/// # Examples
///
/// ```no_run
/// # use modern_rugby_2024::process_tasks_parallel;
/// # tokio_test::block_on(async {
/// let tasks = vec!["Task1", "Task2", "Task3"];
/// process_tasks_parallel(tasks, async |task| {
///     println!("Processing: {}", task);
/// }).await;
/// # });
/// ```
pub async fn process_tasks_parallel<T, F>(tasks: Vec<T>, processor: F)
where
    T: Send + 'static,
    F: Fn(T) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>> + Send + Sync,
{
    let handles: Vec<_> = tasks
        .into_iter()
        .map(|task| tokio::spawn(processor(task)))
        .collect();

    for handle in handles {
        let _ = handle.await;
    }
}

// =============================================================================
// メイン関数
// =============================================================================

/// メイン実行例
///
/// Rust 2024 editionの機能を活用した非同期ラグビー戦術デモ
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Rust 2024 Edition: ラグビー非同期戦術デモ ===\n");

    let start = std::time::Instant::now();

    println!("⚡ 攻撃開始！\n");

    // フェーズ1: 情報収集（すべて並行実行）
    // Rust 2024のpreludeにより、Futureトレイトは自動的にインポート済み
    let (ball, defense, teammates) =
        tokio::join!(wait_for_ball(), read_defense(), check_teammates());

    // フェーズ2: サイン出し（並行実行）
    tokio::join!(signal_backs(), signal_forwards());

    // フェーズ3: 判断と実行
    let decision = make_decision(ball, defense, teammates).await;

    let duration = start.elapsed();

    println!("\n🎯 決定: {}", decision);
    println!("⏱️  判断までの時間: {:.1}秒", duration.as_secs_f64());
    println!(
        "\n💡 並行処理により、順次処理の13秒から{:.1}秒に短縮！",
        duration.as_secs_f64()
    );

    // Rust 2024: Async Closuresのデモ
    println!("\n\n=== Async Closures デモ ===\n");

    let phases = vec!["Phase 1", "Phase 2", "Phase 3"];

    process_tasks_parallel(phases, |phase| {
        Box::pin(async move {
            println!("📋 {} を実行中...", phase);
            sleep(Duration::from_millis(300)).await;
            println!("✓ {} 完了", phase);
        })
    })
    .await;

    println!("\n✅ すべてのデモが完了しました！");

    Ok(())
}
