// Copyright 2020 Kazuyuki HIDA <kazhida@abplus.com>
// All right reserved.
//

use crate::switch::Shape::Rectangle;
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

/// # キースイッチ
///
/// x, yは左上を頂点とする絶対座標（キーの中心位置を示す）。
/// rは反時計回りのdegreeで、キーの中心で回転するものとする。
/// x, y, w, hの単位は、いわゆる1u。
///
/// Keyboard Layout Editor のraw-dataは、右に進み、改行時に左端に戻るという規則に
/// タートル・グラフィックスの要素を加えたものなので、ここでの仕様とかなり違うけど、
/// 変換処理実装時に頑張る
#[derive(Debug)]
pub struct Switch {
    shape: Shape,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    r: f32,
    actions: Vec<Action, U4>,
    default_action: Action
}

impl Switch {

    /// 何もしないダミーキー（Device内の初期値とかに使用する）
    pub fn dummy() -> Self {
        Self {
            shape: Rectangle,
            x: 0.0,
            y: 0.0,
            w: 0.0,
            h: 0.0,
            r: 0.0,
            actions: Vec::new(),
            default_action: NoOp
        }
    }

    /// 位置を指定してインスタンスを生成
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            shape: Rectangle,
            x,
            y,
            w: 1.0,
            h: 1.0,
            r: 0.0,
            actions: Vec::new(),
            default_action: Trans
        }
    }

    /// 位置を指定してインスタンスを生成
    pub fn new_with_shape(shape: Shape, x: f32, y: f32) -> Self {
        Self {
            shape,
            x,
            y,
            w: match shape {
                Shape::IsoEnter => 1.25,
                Shape::Rectangle => 1.0
            },
            h: match shape {
                Shape::IsoEnter => 2.0,
                Shape::Rectangle => 1.0
            },
            r: 0.0,
            actions: Vec::new(),
            default_action: Trans
        }
    }

    /// 位置と幅を指定してインスタンスを生成
    pub fn new_with_width(x: f32, y: f32, w: f32) -> Self {
        Self {
            shape: Rectangle,
            x,
            y,
            w,
            h: 1.0,
            r: 0.0,
            actions: Vec::new(),
            default_action: Trans
        }
    }

    /// 位置と大きさを指定してインスタンスを生成
    pub fn new_with_size(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self {
            shape: Rectangle,
            x,
            y,
            w,
            h,
            r: 0.0,
            actions: Vec::new(),
            default_action: Trans
        }
    }

    /// その場で回転
    pub fn rotate(&mut self, r: f32) -> &mut Self {
        self.r = r;
        self
    }

    /// アクションを追加
    ///
    /// 4レイヤーまで追加出来るがそれを超えると無視される
    pub fn append_action(&mut self, a: Action) -> &mut Self {
        let _ = self.actions.push(a);
        self
    }

    pub fn default_action(&mut self, a: Action) -> &mut Self {
        self.default_action = a;
        self
    }
}

impl Default for Switch {
    fn default() -> Self { Switch::dummy() }
}

