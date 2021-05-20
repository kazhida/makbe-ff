// Copyright 2020 Kazuyuki HIDA <kazhida@abplus.com>
// All right reserved.
//

use crate::switch::Shape::Rectangle;
use keyberon::action::Action;
use heapless::Vec;
use heapless::consts::U4;

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
#[derive(Debug, Clone)]
pub struct Switch {
    shape: Shape,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    r: f32,
    actions: Vec<Action, U4>
}

impl Switch {

    /// 位置を指定してインスタンスを生成
    fn new(x: f32, y: f32) -> Self {
        Self {
            shape: Rectangle,
            x,
            y,
            w: 1.0,
            h: 1.0,
            r: 0.0,
            actions: Vec::new()
        }
    }

    /// 位置と幅を指定してインスタンスを生成
    fn new_with_width(x: f32, y: f32, w: f32) -> Self {
        Self {
            shape: Rectangle,
            x,
            y,
            w,
            h: 1.0,
            r: 0.0,
            actions: Vec::new()
        }
    }

    /// 位置と大きさを指定してインスタンスを生成
    fn new_with_size(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self {
            shape: Rectangle,
            x,
            y,
            w,
            h,
            r: 0.0,
            actions: Vec::new()
        }
    }

    /// 形状を変更
    fn shape(&mut self, shape: Shape) -> &mut Self {
        self.shape = shape;
        self
    }

    /// 位置を移動
    fn move_to(&mut self, x: f32, y: f32) -> &mut Self {
        self.x = x;
        self.y = y;
        self
    }

    /// その場で回転
    fn rotate(&mut self, r: f32) -> &mut Self {
        self.r = r;
        self
    }

    /// アクションを追加
    ///
    /// 4レイヤーまで追加出来るがそれを超えると無視される
    fn action(&mut self, a: Action) -> &mut Self {
        self.actions.push(a);
        self
    }
}
