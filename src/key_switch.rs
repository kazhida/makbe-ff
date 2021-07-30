// Copyright 2020 Kazuyuki HIDA <kazhida@abplus.com>
// All right reserved.
//

use keyberon::action::Action;
use heapless::Vec;
use heapless::consts::U4;
use keyberon::action::Action::{NoOp, Trans};

/// # キーの形状
///
/// 下が長いEnterの呼び名が分からないので含めていない
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Shape {
    Rectangle,
    IsoEnter
}

/// スイッチの位置的情報を表すデータ
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Position {
    /// 横方向
    pub x: i32,
    /// 縦方向
    pub y: i32,
    /// 幅
    pub w: i32,
    /// 高さ
    pub h: i32,
    /// 回転の中心（横方向）
    pub rx: i32,
    /// 回転の中心（縦方向）
    pub ry: i32,
    /// 回転角度（degree）
    pub r: i32
}

impl Position {

    pub fn internal_value(v: f32) -> i32 {
        (v * 256.0) as i32
    }

    pub fn new(x: f32, y: f32, w: f32, h: f32, r: f32, rx: f32, ry: f32) -> Self {
        Self {
            x: Self::internal_value(x),
            y: Self::internal_value(y),
            w: Self::internal_value(w),
            h: Self::internal_value(h),
            rx: Self::internal_value(rx),
            ry: Self::internal_value(ry),
            r: Self::internal_value(r)
        }
    }
}

/// # キースイッチ
///
/// x, yは左上を頂点とする絶対座標（キーの中心位置を示す）。
/// rは反時計回りのdegreeで、キーの中心で回転するものとする。
/// x, y, w, hの単位は、いわゆる1u。
///
/// Keyboard Layout Editor のraw-dataは、右に進み、改行時に左端に戻るという規則に
/// タートル・グラフィックスの要素を加えたものなので、ここでの仕様とかなり違うけど、
/// 変換処理実装時に頑張る
#[derive(Debug, Eq, PartialEq)]
pub struct KeySwitch {
    pub shape: Shape,
    pub position: Position,
    pub actions: Vec<Action, U4>,
    default_action: Action
}

impl KeySwitch {

    pub fn apply<F>(mut self, mut f: F) -> Self
        where
            F: FnMut(&mut KeySwitch) -> &mut KeySwitch
    {
        f(&mut self);
        self
    }

    /// 何もしないダミーキー（Device内の初期値とかに使用する）
    pub fn dummy() -> Self {
        Self {
            shape: Shape::Rectangle,
            position: Position::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
            actions: Vec::new(),
            default_action: NoOp
        }
    }

    /// 位置を指定してインスタンスを生成
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            shape: Shape::Rectangle,
            position: Position::new(x, y, 1.0, 1.0, 0.0, 0.0, 0.0),
            actions: Vec::new(),
            default_action: Trans
        }
    }

    /// 位置を指定してインスタンスを生成
    pub fn new_with_shape(shape: Shape, x: f32, y: f32) -> Self {
        Self {
            shape,
            position: Position::new(
                x,
                y,
                match shape {
                    Shape::IsoEnter => 1.25,
                    Shape::Rectangle => 1.0
                },
                match shape {
                    Shape::IsoEnter => 2.0,
                    Shape::Rectangle => 1.0
                },
                0.0,
                0.0,
                0.0
            ),
            actions: Vec::new(),
            default_action: Trans
        }
    }

    /// 位置と幅を指定してインスタンスを生成
    pub fn new_with_width(x: f32, y: f32, w: f32) -> Self {
        Self {
            shape: Shape::Rectangle,
            position: Position::new(x, y, w, 1.0, 0.0, 0.0, 0.0),
            actions: Vec::new(),
            default_action: Trans
        }
    }

    /// 位置と大きさを指定してインスタンスを生成
    pub fn new_with_size(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self {
            shape: Shape::Rectangle,
            position: Position::new(x, y, w, h, 0.0, 0.0, 0.0),
            actions: Vec::new(),
            default_action: Trans
        }
    }

    /// その場で回転
    pub fn rotate(&mut self, r: f32) -> &mut Self {
        self.position.r = Position::internal_value(r);
        self.position.rx = self.position.x;
        self.position.ry = self.position.y;
        self
    }

    /// 中心を指定して回転
    pub fn rotate_at(&mut self, r: f32, rx: f32, ry: f32) -> &mut Self {
        self.position.r = Position::internal_value(r);
        self.position.rx = Position::internal_value(rx);
        self.position.ry = Position::internal_value(ry);
        self
    }

    /// アクションを追加
    pub fn append_action(&mut self, a: Action) -> &mut Self {
        let _ = self.actions.push(a);
        self
    }

    /// デフォルトアクションの設定
    pub fn default_action(&mut self, a: Action) -> &mut Self {
        self.default_action = a;
        self
    }

    /// レイヤを指定してアクションを取得
    pub fn action_at(&'static self, layer: usize) -> Option<&'static Action> {
        if layer < self.actions.len() {
            Some(&self.actions[layer])
        } else {
            None
        }
    }
}

impl Default for KeySwitch {
    fn default() -> Self { KeySwitch::dummy() }
}

#[macro_export]
macro_rules! switch_pool {
    ($(#[$top_attr:meta])* struct $Type:ident,
    $( $(#[$attr:meta])* switch $name:ident = $switch_expr:expr),
    + , ) => {

        paste::item! {
            $(#[$top_attr])*
            pub struct $Type {
                $(
                    $(#[$attr])*
                    pub $name: KeySwitch
                ),+
            }
        }

        impl $Type {
            /// Returns the pins for the device
            paste::item! {
                pub fn new() -> Self {
                    $Type {
                        $(
                        $(#[$attr])*
                        $name: $switch_expr
                        ),+
                    }
                }
            }
        }
    }
}
