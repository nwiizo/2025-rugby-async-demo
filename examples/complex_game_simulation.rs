/// 複雑なゲームシミュレーション - Rust 2024 Edition
///
/// このファイルは、現実的な試合状況を反映した複雑な意思決定システムを実装します。
///
/// # 実装されている変数
///
/// - **試合時間**: 経過時間と残り時間
/// - **スコア差**: 現在の得点差
/// - **フィールドポジション**: フィールド上の位置（Own22, Midfield等）
/// - **天候**: 天気と風の状態
/// - **チーム疲労度**: 個別およびチーム全体の疲労
/// - **ゲームルール**: 15人制、7人制等のルールセット
/// - **ボール所持状況**: 連続フェーズ数
/// - **その他**: ペナルティ数、イエローカード等
use std::time::Duration;
use tokio::time::sleep;

// =============================================================================
// ゲーム状態の型定義
// =============================================================================

/// フィールド上の位置
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldPosition {
    /// 自陣22mライン内（危険地帯）
    Own22,
    /// 自陣22m～ハーフライン
    OwnHalf,
    /// 中盤（ハーフライン付近）
    Midfield,
    /// 敵陣ハーフ
    OppositionHalf,
    /// 敵陣22mライン内（得点圏内）
    Opposition22,
}

impl FieldPosition {
    /// この位置からのリスク評価（0.0-1.0）
    pub fn risk_level(&self) -> f32 {
        match self {
            FieldPosition::Own22 => 0.9,          // 非常に危険
            FieldPosition::OwnHalf => 0.6,        // やや危険
            FieldPosition::Midfield => 0.5,       // 中立
            FieldPosition::OppositionHalf => 0.3, // 攻撃的
            FieldPosition::Opposition22 => 0.1,   // 得点チャンス
        }
    }

    /// キックの推奨度（0.0-1.0）
    pub fn kick_preference(&self) -> f32 {
        match self {
            FieldPosition::Own22 => 0.8, // キック推奨
            FieldPosition::OwnHalf => 0.6,
            FieldPosition::Midfield => 0.4,
            FieldPosition::OppositionHalf => 0.2,
            FieldPosition::Opposition22 => 0.0, // ランプレー推奨
        }
    }
}

/// 天候の状態
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Weather {
    /// 晴天（理想的）
    Sunny,
    /// 曇り
    Cloudy,
    /// 雨天（滑りやすい）
    Rainy,
    /// 強風
    Windy,
    /// 雨+風（最悪条件）
    StormyRain,
}

impl Weather {
    /// パスの成功率への影響（0.0-1.0）
    pub fn pass_difficulty(&self) -> f32 {
        match self {
            Weather::Sunny => 0.0,
            Weather::Cloudy => 0.1,
            Weather::Rainy => 0.3,
            Weather::Windy => 0.4,
            Weather::StormyRain => 0.6,
        }
    }

    /// キックの推奨度
    pub fn kick_preference(&self) -> f32 {
        match self {
            Weather::Sunny => 0.3,
            Weather::Cloudy => 0.4,
            Weather::Rainy => 0.6, // 雨天時はキック推奨
            Weather::Windy => 0.2, // 風が強いとキックは難しい
            Weather::StormyRain => 0.1,
        }
    }
}

/// 風の状態
#[derive(Debug, Clone, Copy)]
pub struct Wind {
    /// 風速（m/s）
    pub speed: f32,
    /// 風向き（度、0=北、90=東、180=南、270=西）
    pub direction: f32,
}

/// 疲労度レベル
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FatigueLevel {
    /// フレッシュ（0-20%疲労）
    Fresh,
    /// 軽度の疲労（20-40%）
    Moderate,
    /// 疲労（40-70%）
    Tired,
    /// 極度の疲労（70-100%）
    Exhausted,
}

impl FatigueLevel {
    /// 疲労度から判断（0.0-1.0）
    pub fn from_percentage(fatigue: f32) -> Self {
        if fatigue < 0.2 {
            FatigueLevel::Fresh
        } else if fatigue < 0.4 {
            FatigueLevel::Moderate
        } else if fatigue < 0.7 {
            FatigueLevel::Tired
        } else {
            FatigueLevel::Exhausted
        }
    }

    /// パフォーマンスへの影響（0.0-1.0、1.0が最高）
    pub fn performance_multiplier(&self) -> f32 {
        match self {
            FatigueLevel::Fresh => 1.0,
            FatigueLevel::Moderate => 0.85,
            FatigueLevel::Tired => 0.65,
            FatigueLevel::Exhausted => 0.4,
        }
    }
}

/// ゲームルール
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameRules {
    /// 15人制ラグビー（80分）
    Fifteens,
    /// 7人制ラグビー（14分）
    Sevens,
    /// 10人制ラグビー
    Tens,
}

impl GameRules {
    /// 試合時間（秒）
    pub fn match_duration_secs(&self) -> u32 {
        match self {
            GameRules::Fifteens => 80 * 60,
            GameRules::Sevens => 14 * 60,
            GameRules::Tens => 60 * 60,
        }
    }

    /// 疲労の蓄積速度（1分あたりの疲労度増加）
    pub fn fatigue_rate(&self) -> f32 {
        match self {
            GameRules::Fifteens => 0.0125, // 80分で100%
            GameRules::Sevens => 0.05,     // 14分でも高強度
            GameRules::Tens => 0.0167,
        }
    }
}

/// スコア状況
#[derive(Debug, Clone, Copy)]
pub struct Score {
    /// 自チームの得点
    pub own: u32,
    /// 相手チームの得点
    pub opposition: u32,
}

impl Score {
    /// 点差（正の値=リード、負の値=ビハインド）
    pub fn difference(&self) -> i32 {
        self.own as i32 - self.opposition as i32
    }

    /// 緊急性評価（0.0-1.0）
    pub fn urgency(&self, time_remaining_secs: u32) -> f32 {
        let diff = self.difference();
        let minutes_left = time_remaining_secs as f32 / 60.0;

        if diff > 14 {
            // 大量リード：守りのプレー
            0.2
        } else if diff > 7 {
            // リード：安定したプレー
            0.4
        } else if diff.abs() <= 7 {
            // 接戦：バランス
            0.6
        } else if diff < -7 && minutes_left < 10.0 {
            // ビハインド＆残り時間少ない：緊急
            0.9
        } else {
            // ビハインド：やや緊急
            0.7
        }
    }
}

/// チーム全体の疲労状態
#[derive(Debug, Clone)]
pub struct TeamFatigue {
    /// フォワードの平均疲労度（0.0-1.0）
    pub forwards: f32,
    /// バックスの平均疲労度（0.0-1.0）
    pub backs: f32,
}

impl TeamFatigue {
    /// 全体の疲労度
    pub fn overall(&self) -> f32 {
        self.forwards * 0.6 + self.backs * 0.4 // フォワードの方が重要
    }

    /// 疲労度レベル
    pub fn level(&self) -> FatigueLevel {
        FatigueLevel::from_percentage(self.overall())
    }
}

/// ゲーム全体の状態
#[derive(Debug, Clone)]
pub struct GameState {
    /// 試合ルール
    pub rules: GameRules,
    /// 経過時間（秒）
    pub elapsed_time_secs: u32,
    /// スコア
    pub score: Score,
    /// フィールドポジション
    pub position: FieldPosition,
    /// 天候
    pub weather: Weather,
    /// 風
    pub wind: Wind,
    /// チーム疲労度
    pub fatigue: TeamFatigue,
    /// 連続フェーズ数
    pub consecutive_phases: u32,
    /// ペナルティ数（自チーム）
    pub penalties_conceded: u32,
    /// イエローカード人数
    pub yellow_cards: u32,
    /// ディフェンスライン
    pub defense: DefenseLine,
    /// チームメイト
    pub teammates: Teammates,
}

impl GameState {
    /// 残り時間（秒）
    pub fn time_remaining_secs(&self) -> u32 {
        let total = self.rules.match_duration_secs();
        total.saturating_sub(self.elapsed_time_secs)
    }

    /// 時間のプレッシャー（0.0-1.0）
    pub fn time_pressure(&self) -> f32 {
        let remaining = self.time_remaining_secs() as f32;
        let total = self.rules.match_duration_secs() as f32;
        1.0 - (remaining / total)
    }
}

/// ディフェンスラインの状態
#[derive(Debug, Clone)]
pub struct DefenseLine {
    pub pressure: bool,
    pub gap_on_left: bool,
    pub gap_on_right: bool,
    /// ディフェンスの整列度（0.0-1.0）
    pub alignment: f32,
}

/// チームメイトの状態
#[derive(Debug, Clone)]
pub struct Teammates {
    pub backs_ready: bool,
    pub forwards_ready: bool,
    /// サポートプレーヤーの数
    pub support_count: u32,
}

/// 攻撃判断の種類
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TacticalDecision {
    /// パス展開
    PassSpread { direction: Direction },
    /// クラッシュボール
    Crash,
    /// ハイキック
    Kick { kick_type: KickType },
    /// クイックタップ
    QuickTap,
    /// モール形成
    Maul,
    /// スクラム
    Scrum,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
    Center,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KickType {
    /// ハイパント（高く蹴り上げる）
    HighPunt,
    /// タッチキック（タッチラインへ）
    Touch,
    /// グラバー（地面を転がす）
    Grubber,
    /// クロスフィールド
    Crossfield,
}

impl std::fmt::Display for TacticalDecision {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TacticalDecision::PassSpread { direction } => {
                write!(f, "{:?}サイドへパス展開", direction)
            }
            TacticalDecision::Crash => write!(f, "クラッシュボール"),
            TacticalDecision::Kick { kick_type } => {
                write!(f, "{:?}キック", kick_type)
            }
            TacticalDecision::QuickTap => write!(f, "クイックタップ"),
            TacticalDecision::Maul => write!(f, "モール形成"),
            TacticalDecision::Scrum => write!(f, "スクラム"),
        }
    }
}

// =============================================================================
// 複雑な意思決定ロジック
// =============================================================================

/// 複雑な状況分析を行う
async fn analyze_game_state(state: &GameState) -> String {
    println!("\n=== 詳細な状況分析 ===");
    println!(
        "⏱️  経過時間: {}分{}秒 / 残り: {}分{}秒",
        state.elapsed_time_secs / 60,
        state.elapsed_time_secs % 60,
        state.time_remaining_secs() / 60,
        state.time_remaining_secs() % 60
    );
    println!(
        "📊 スコア: {} - {} (差: {:+}点)",
        state.score.own,
        state.score.opposition,
        state.score.difference()
    );
    println!(
        "📍 フィールド位置: {:?} (リスク: {:.0}%)",
        state.position,
        state.position.risk_level() * 100.0
    );
    println!("🌤️  天候: {:?}", state.weather);
    println!(
        "💨 風: {:.1}m/s 方向{:.0}°",
        state.wind.speed, state.wind.direction
    );
    println!(
        "😓 疲労度: FW {:.0}% / BK {:.0}% (全体: {:?})",
        state.fatigue.forwards * 100.0,
        state.fatigue.backs * 100.0,
        state.fatigue.level()
    );
    println!("🔄 連続フェーズ: {}", state.consecutive_phases);
    println!(
        "⚠️  ペナルティ: {} / イエローカード: {}",
        state.penalties_conceded, state.yellow_cards
    );

    sleep(Duration::from_millis(500)).await;
    "分析完了".to_string()
}

/// 複雑な意思決定を行う
pub async fn make_complex_decision(state: &GameState) -> TacticalDecision {
    println!("\n🧠 複雑な状況判断を開始...\n");

    // 各要素の分析
    let position_risk = state.position.risk_level();
    let time_pressure = state.time_pressure();
    let score_urgency = state.score.urgency(state.time_remaining_secs());
    let fatigue_impact = 1.0 - state.fatigue.overall();
    let weather_difficulty = state.weather.pass_difficulty();

    println!("📐 リスク評価:");
    println!("  - ポジションリスク: {:.0}%", position_risk * 100.0);
    println!("  - 時間プレッシャー: {:.0}%", time_pressure * 100.0);
    println!("  - スコア緊急性: {:.0}%", score_urgency * 100.0);
    println!("  - 疲労影響: {:.0}%", (1.0 - fatigue_impact) * 100.0);
    println!("  - 天候難易度: {:.0}%", weather_difficulty * 100.0);

    sleep(Duration::from_millis(300)).await;

    // ケース1: 危険地帯でのプレー
    if matches!(state.position, FieldPosition::Own22) && state.defense.pressure {
        println!("\n⚠️  危険！自陣22mでプレッシャー → タッチキック");
        return TacticalDecision::Kick {
            kick_type: KickType::Touch,
        };
    }

    // ケース2: 点差が大きく時間が少ない
    if state.score.difference() < -7 && state.time_remaining_secs() < 600 {
        println!("\n🚨 ビハインド＆残り時間わずか → クイックタップで速攻");
        return TacticalDecision::QuickTap;
    }

    // ケース3: 大量リードで守りたい
    if state.score.difference() > 14 && time_pressure > 0.75 {
        println!("\n🛡️  大量リード＆終盤 → 安全なキック");
        return TacticalDecision::Kick {
            kick_type: KickType::Touch,
        };
    }

    // ケース4: 疲労が激しい
    if matches!(state.fatigue.level(), FatigueLevel::Exhausted) && state.teammates.forwards_ready {
        println!("\n😓 極度の疲労 → シンプルなクラッシュボール");
        return TacticalDecision::Crash;
    }

    // ケース5: 悪天候
    if matches!(state.weather, Weather::Rainy | Weather::StormyRain)
        && state.teammates.forwards_ready
    {
        println!("\n🌧️  悪天候 → フォワード中心のプレー");
        return TacticalDecision::Crash;
    }

    // ケース6: 得点圏内
    if matches!(state.position, FieldPosition::Opposition22)
        && state.defense.gap_on_left
        && state.teammates.backs_ready
    {
        println!("\n🎯 得点圏内でギャップ発見 → パス展開");
        return TacticalDecision::PassSpread {
            direction: Direction::Left,
        };
    }

    // ケース7: 連続フェーズが多い
    if state.consecutive_phases > 10 {
        println!("\n🔄 長い連続フェーズ → キックでリセット");
        return TacticalDecision::Kick {
            kick_type: KickType::HighPunt,
        };
    }

    // デフォルト: バランスの取れた判断
    if state.defense.gap_on_left && state.teammates.backs_ready {
        println!("\n✅ 標準的状況 → パス展開");
        TacticalDecision::PassSpread {
            direction: Direction::Left,
        }
    } else if state.teammates.forwards_ready {
        println!("\n💪 フォワードでゲイン");
        TacticalDecision::Crash
    } else {
        println!("\n⚡ キックでフィールドポジション確保");
        TacticalDecision::Kick {
            kick_type: KickType::Touch,
        }
    }
}

// =============================================================================
// メイン実行
// =============================================================================

#[tokio::main]
async fn main() {
    println!("🏉 複雑なゲームシミュレーション - Rust 2024 Edition\n");
    println!("{}", "=".repeat(60));

    // シナリオ1: 接戦の終盤
    println!("\n【シナリオ1】接戦の終盤、自陣でボール確保");
    let state1 = GameState {
        rules: GameRules::Fifteens,
        elapsed_time_secs: 75 * 60, // 75分経過
        score: Score {
            own: 21,
            opposition: 24,
        }, // 3点ビハインド
        position: FieldPosition::OwnHalf,
        weather: Weather::Cloudy,
        wind: Wind {
            speed: 3.0,
            direction: 90.0,
        },
        fatigue: TeamFatigue {
            forwards: 0.65,
            backs: 0.50,
        },
        consecutive_phases: 3,
        penalties_conceded: 8,
        yellow_cards: 0,
        defense: DefenseLine {
            pressure: true,
            gap_on_left: false,
            gap_on_right: false,
            alignment: 0.8,
        },
        teammates: Teammates {
            backs_ready: true,
            forwards_ready: true,
            support_count: 5,
        },
    };

    analyze_game_state(&state1).await;
    let decision1 = make_complex_decision(&state1).await;
    println!("\n✨ 最終判断: {}", decision1);

    println!("\n{}", "=".repeat(60));

    // シナリオ2: 悪天候、得点圏内
    println!("\n【シナリオ2】雨天、敵陣22m内でチャンス");
    let state2 = GameState {
        rules: GameRules::Fifteens,
        elapsed_time_secs: 35 * 60,
        score: Score {
            own: 14,
            opposition: 10,
        },
        position: FieldPosition::Opposition22,
        weather: Weather::Rainy,
        wind: Wind {
            speed: 8.0,
            direction: 180.0,
        },
        fatigue: TeamFatigue {
            forwards: 0.40,
            backs: 0.35,
        },
        consecutive_phases: 12,
        penalties_conceded: 3,
        yellow_cards: 0,
        defense: DefenseLine {
            pressure: false,
            gap_on_left: true,
            gap_on_right: false,
            alignment: 0.6,
        },
        teammates: Teammates {
            backs_ready: true,
            forwards_ready: true,
            support_count: 7,
        },
    };

    analyze_game_state(&state2).await;
    let decision2 = make_complex_decision(&state2).await;
    println!("\n✨ 最終判断: {}", decision2);

    println!("\n{}", "=".repeat(60));
    println!("\n✅ シミュレーション完了！");
    println!("\n💡 このシミュレーションは、複数の変数を考慮した");
    println!("   現実的な意思決定プロセスを示しています。");
}
