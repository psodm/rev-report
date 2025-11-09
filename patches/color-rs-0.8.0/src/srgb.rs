// Copyright 2013 The color-rs developers. For a full listing of the authors,
// refer to the AUTHORS file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use channel::Channel;
use color_space::{D65, D50, Mat3, MatrixColorSpace, TransferFunction, Vec3};
use num_traits::{Float, cast};
use yxy::Yxy;
use rgb::{Rgb, ToRgb};
use alpha::{Rgba, ToRgba, Srgba, ToSrgba};
use xyz::{Xyz, ToXyz};
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub struct Srgb<T> { pub r: T, pub g: T, pub b: T }

impl<T> Srgb<T> {
    #[inline]
    pub fn new(r: T, g: T, b: T) -> Srgb<T> {
        Srgb { r: r, g: g, b: b }
    }
}

pub trait ToSrgb {
    fn to_srgb<T: Channel>(&self) -> Srgb<T>;
}

impl<T: Channel> ToSrgb for Srgb<T>{
    fn to_srgb<U: Channel>(&self) -> Srgb<U>{
        Srgb{ r: self.r.to_channel(), g: self.g.to_channel(), b: self.b.to_channel() }
    }
}

impl<T: Channel + Float> ToRgb for Srgb<T> {
    fn to_rgb<U: Channel>(&self) -> Rgb<U> {
        Rgb{
            r: Srgb::to_linear(self.r).to_channel(),
            g: Srgb::to_linear(self.g).to_channel(),
            b: Srgb::to_linear(self.b).to_channel(),
        }
    }
}

impl<T: Channel + Float> ToRgba for Srgb<T> {
    fn to_rgba<U: Channel>(&self) -> Rgba<U> {
        Rgba{
            c: Rgb{
                r: Srgb::to_linear(self.r).to_channel(),
                g: Srgb::to_linear(self.g).to_channel(),
                b: Srgb::to_linear(self.b).to_channel(),
            },
            a: 1f32.to_channel(),
        }
    }
}

impl<T: Channel> ToSrgba for Srgb<T> {
    #[inline]
    fn to_srgba<U: Channel>(&self) -> Srgba<U>{
        Srgba{c: self.to_srgb(), a: 1f32.to_channel()}
    }
}

impl<T: Channel + Float + Clone + Debug> ToXyz for Srgb<T> {
    type WhitePoint = D65;
    fn to_xyz<U: Channel + Float + Debug>(&self) -> Xyz<U, D65> {
        let xyz: Vec3<T> = Srgb::to_xyz_matrix() * self.clone().to_rgb().into();
        Xyz::new(xyz[0].to_channel(), xyz[1].to_channel(), xyz[2].to_channel())
    }
}

impl<T: Channel + Float> MatrixColorSpace for Srgb<T> {
    type WhitePoint = D65;
    type ChannelTy = T;

    fn red() -> Yxy<T, D50> {
        Yxy::new(0.6400.to_channel(), 0.3300.to_channel(), 0.212656.to_channel())
    }
    fn green() -> Yxy<T, D50> {
        Yxy::new(0.3000.to_channel(), 0.6000.to_channel(), 0.715158.to_channel())
    }
    fn blue() -> Yxy<T, D50> {
        Yxy::new(0.1500.to_channel(), 0.0600.to_channel(), 0.072186.to_channel())
    }
    fn to_xyz_matrix() -> Mat3<T>{
        Mat3([
            0.4124564.to_channel(),  0.3575761.to_channel(),  0.1804375.to_channel(),
            0.2126729.to_channel(),  0.7151522.to_channel(),  0.0721750.to_channel(),
            0.0193339.to_channel(),  0.1191920.to_channel(),  0.9503041.to_channel(),
        ])
    }
    fn to_rgb_matrix() -> Mat3<T>{
        Mat3([
            3.2404542.to_channel(), (-1.5371385).to_channel(), (-0.4985314).to_channel(),
            (-0.9692660).to_channel(),  1.8760108.to_channel(),  0.0415560.to_channel(),
            0.0556434.to_channel(), (-0.2040259).to_channel(),  1.0572252.to_channel(),
        ])
    }
}

impl<T: Channel + Float> TransferFunction for Srgb<T>{
    type ChannelTy = T;
    fn from_linear(x: T) -> T {
        if x > cast(0.0031308).unwrap() {
            cast::<f32, T>(1.055).unwrap() * x.powf(cast(1. / 2.4).unwrap()) - cast(0.055).unwrap()
        }else{
            cast::<f32, T>(12.95).unwrap() * x
        }
    }

    fn to_linear(x: T) -> T {
        if x > cast(0.04045).unwrap() {
            ((x + cast(0.055).unwrap()) / cast(1.055).unwrap()).powf(cast(2.4).unwrap())
        }else{
            x / cast(12.92).unwrap()
        }
    }
}


/// SVG 1.0 color constants: http://www.w3.org/TR/SVG/types.html#ColorKeywords
pub mod consts {
    use Srgb;

    pub static ALICEBLUE:               Srgb<u8> = Srgb { r: 0xF0, g: 0xF8, b: 0xFF };
    pub static ANTIQUEWHITE:            Srgb<u8> = Srgb { r: 0xFA, g: 0xEB, b: 0xD7 };
    pub static AQUA:                    Srgb<u8> = Srgb { r: 0x00, g: 0xFF, b: 0xFF };
    pub static AQUAMARINE:              Srgb<u8> = Srgb { r: 0x7F, g: 0xFF, b: 0xD4 };
    pub static AZURE:                   Srgb<u8> = Srgb { r: 0xF0, g: 0xFF, b: 0xFF };
    pub static BEIGE:                   Srgb<u8> = Srgb { r: 0xF5, g: 0xF5, b: 0xDC };
    pub static BISQUE:                  Srgb<u8> = Srgb { r: 0xFF, g: 0xE4, b: 0xC4 };
    pub static BLACK:                   Srgb<u8> = Srgb { r: 0x00, g: 0x00, b: 0x00 };
    pub static BLANCHEDALMOND:          Srgb<u8> = Srgb { r: 0xFF, g: 0xEB, b: 0xCD };
    pub static BLUE:                    Srgb<u8> = Srgb { r: 0x00, g: 0x00, b: 0xFF };
    pub static BLUEVIOLET:              Srgb<u8> = Srgb { r: 0x8A, g: 0x2B, b: 0xE2 };
    pub static BROWN:                   Srgb<u8> = Srgb { r: 0xA5, g: 0x2A, b: 0x2A };
    pub static BURLYWOOD:               Srgb<u8> = Srgb { r: 0xDE, g: 0xB8, b: 0x87 };
    pub static CADETBLUE:               Srgb<u8> = Srgb { r: 0x5F, g: 0x9E, b: 0xA0 };
    pub static CHARTREUSE:              Srgb<u8> = Srgb { r: 0x7F, g: 0xFF, b: 0x00 };
    pub static CHOCOLATE:               Srgb<u8> = Srgb { r: 0xD2, g: 0x69, b: 0x1E };
    pub static CORAL:                   Srgb<u8> = Srgb { r: 0xFF, g: 0x7F, b: 0x50 };
    pub static CORNFLOWERBLUE:          Srgb<u8> = Srgb { r: 0x64, g: 0x95, b: 0xED };
    pub static CORNSILK:                Srgb<u8> = Srgb { r: 0xFF, g: 0xF8, b: 0xDC };
    pub static CRIMSON:                 Srgb<u8> = Srgb { r: 0xDC, g: 0x14, b: 0x3C };
    pub static CYAN:                    Srgb<u8> = Srgb { r: 0x00, g: 0xFF, b: 0xFF };
    pub static DARKBLUE:                Srgb<u8> = Srgb { r: 0x00, g: 0x00, b: 0x8B };
    pub static DARKCYAN:                Srgb<u8> = Srgb { r: 0x00, g: 0x8B, b: 0x8B };
    pub static DARKGOLDENROD:           Srgb<u8> = Srgb { r: 0xB8, g: 0x86, b: 0x0B };
    pub static DARKGRAY:                Srgb<u8> = Srgb { r: 0xA9, g: 0xA9, b: 0xA9 };
    pub static DARKGREEN:               Srgb<u8> = Srgb { r: 0x00, g: 0x64, b: 0x00 };
    pub static DARKKHAKI:               Srgb<u8> = Srgb { r: 0xBD, g: 0xB7, b: 0x6B };
    pub static DARKMAGENTA:             Srgb<u8> = Srgb { r: 0x8B, g: 0x00, b: 0x8B };
    pub static DARKOLIVEGREEN:          Srgb<u8> = Srgb { r: 0x55, g: 0x6B, b: 0x2F };
    pub static DARKORANGE:              Srgb<u8> = Srgb { r: 0xFF, g: 0x8C, b: 0x00 };
    pub static DARKORCHID:              Srgb<u8> = Srgb { r: 0x99, g: 0x32, b: 0xCC };
    pub static DARKRED:                 Srgb<u8> = Srgb { r: 0x8B, g: 0x00, b: 0x00 };
    pub static DARKSALMON:              Srgb<u8> = Srgb { r: 0xE9, g: 0x96, b: 0x7A };
    pub static DARKSEAGREEN:            Srgb<u8> = Srgb { r: 0x8F, g: 0xBC, b: 0x8F };
    pub static DARKSLATEBLUE:           Srgb<u8> = Srgb { r: 0x48, g: 0x3D, b: 0x8B };
    pub static DARKSLATEGRAY:           Srgb<u8> = Srgb { r: 0x2F, g: 0x4F, b: 0x4F };
    pub static DARKTURQUOISE:           Srgb<u8> = Srgb { r: 0x00, g: 0xCE, b: 0xD1 };
    pub static DARKVIOLET:              Srgb<u8> = Srgb { r: 0x94, g: 0x00, b: 0xD3 };
    pub static DEEPPINK:                Srgb<u8> = Srgb { r: 0xFF, g: 0x14, b: 0x93 };
    pub static DEEPSKYBLUE:             Srgb<u8> = Srgb { r: 0x00, g: 0xBF, b: 0xFF };
    pub static DIMGRAY:                 Srgb<u8> = Srgb { r: 0x69, g: 0x69, b: 0x69 };
    pub static DODGERBLUE:              Srgb<u8> = Srgb { r: 0x1E, g: 0x90, b: 0xFF };
    pub static FIREBRICK:               Srgb<u8> = Srgb { r: 0xB2, g: 0x22, b: 0x22 };
    pub static FLORALWHITE:             Srgb<u8> = Srgb { r: 0xFF, g: 0xFA, b: 0xF0 };
    pub static FORESTGREEN:             Srgb<u8> = Srgb { r: 0x22, g: 0x8B, b: 0x22 };
    pub static FUCHSIA:                 Srgb<u8> = Srgb { r: 0xFF, g: 0x00, b: 0xFF };
    pub static GAINSBORO:               Srgb<u8> = Srgb { r: 0xDC, g: 0xDC, b: 0xDC };
    pub static GHOSTWHITE:              Srgb<u8> = Srgb { r: 0xF8, g: 0xF8, b: 0xFF };
    pub static GOLD:                    Srgb<u8> = Srgb { r: 0xFF, g: 0xD7, b: 0x00 };
    pub static GOLDENROD:               Srgb<u8> = Srgb { r: 0xDA, g: 0xA5, b: 0x20 };
    pub static GRAY:                    Srgb<u8> = Srgb { r: 0x80, g: 0x80, b: 0x80 };
    pub static GREEN:                   Srgb<u8> = Srgb { r: 0x00, g: 0x80, b: 0x00 };
    pub static GREENYELLOW:             Srgb<u8> = Srgb { r: 0xAD, g: 0xFF, b: 0x2F };
    pub static HONEYDEW:                Srgb<u8> = Srgb { r: 0xF0, g: 0xFF, b: 0xF0 };
    pub static HOTPINK:                 Srgb<u8> = Srgb { r: 0xFF, g: 0x69, b: 0xB4 };
    pub static INDIANRED:               Srgb<u8> = Srgb { r: 0xCD, g: 0x5C, b: 0x5C };
    pub static INDIGO:                  Srgb<u8> = Srgb { r: 0x4B, g: 0x00, b: 0x82 };
    pub static IVORY:                   Srgb<u8> = Srgb { r: 0xFF, g: 0xFF, b: 0xF0 };
    pub static KHAKI:                   Srgb<u8> = Srgb { r: 0xF0, g: 0xE6, b: 0x8C };
    pub static LAVENDER:                Srgb<u8> = Srgb { r: 0xE6, g: 0xE6, b: 0xFA };
    pub static LAVENDERBLUSH:           Srgb<u8> = Srgb { r: 0xFF, g: 0xF0, b: 0xF5 };
    pub static LAWNGREEN:               Srgb<u8> = Srgb { r: 0x7C, g: 0xFC, b: 0x00 };
    pub static LEMONCHIFFON:            Srgb<u8> = Srgb { r: 0xFF, g: 0xFA, b: 0xCD };
    pub static LIGHTBLUE:               Srgb<u8> = Srgb { r: 0xAD, g: 0xD8, b: 0xE6 };
    pub static LIGHTCORAL:              Srgb<u8> = Srgb { r: 0xF0, g: 0x80, b: 0x80 };
    pub static LIGHTCYAN:               Srgb<u8> = Srgb { r: 0xE0, g: 0xFF, b: 0xFF };
    pub static LIGHTGOLDENRODYELLOW:    Srgb<u8> = Srgb { r: 0xFA, g: 0xFA, b: 0xD2 };
    pub static LIGHTGREEN:              Srgb<u8> = Srgb { r: 0x90, g: 0xEE, b: 0x90 };
    pub static LIGHTGREY:               Srgb<u8> = Srgb { r: 0xD3, g: 0xD3, b: 0xD3 };
    pub static LIGHTPINK:               Srgb<u8> = Srgb { r: 0xFF, g: 0xB6, b: 0xC1 };
    pub static LIGHTSALMON:             Srgb<u8> = Srgb { r: 0xFF, g: 0xA0, b: 0x7A };
    pub static LIGHTSEAGREEN:           Srgb<u8> = Srgb { r: 0x20, g: 0xB2, b: 0xAA };
    pub static LIGHTSKYBLUE:            Srgb<u8> = Srgb { r: 0x87, g: 0xCE, b: 0xFA };
    pub static LIGHTSLATEGRAY:          Srgb<u8> = Srgb { r: 0x77, g: 0x88, b: 0x99 };
    pub static LIGHTSTEELBLUE:          Srgb<u8> = Srgb { r: 0xB0, g: 0xC4, b: 0xDE };
    pub static LIGHTYELLOW:             Srgb<u8> = Srgb { r: 0xFF, g: 0xFF, b: 0xE0 };
    pub static LIME:                    Srgb<u8> = Srgb { r: 0x00, g: 0xFF, b: 0x00 };
    pub static LIMEGREEN:               Srgb<u8> = Srgb { r: 0x32, g: 0xCD, b: 0x32 };
    pub static LINEN:                   Srgb<u8> = Srgb { r: 0xFA, g: 0xF0, b: 0xE6 };
    pub static MAGENTA:                 Srgb<u8> = Srgb { r: 0xFF, g: 0x00, b: 0xFF };
    pub static MAROON:                  Srgb<u8> = Srgb { r: 0x80, g: 0x00, b: 0x00 };
    pub static MEDIUMAQUAMARINE:        Srgb<u8> = Srgb { r: 0x66, g: 0xCD, b: 0xAA };
    pub static MEDIUMBLUE:              Srgb<u8> = Srgb { r: 0x00, g: 0x00, b: 0xCD };
    pub static MEDIUMORCHID:            Srgb<u8> = Srgb { r: 0xBA, g: 0x55, b: 0xD3 };
    pub static MEDIUMPURPLE:            Srgb<u8> = Srgb { r: 0x93, g: 0x70, b: 0xDB };
    pub static MEDIUMSEAGREEN:          Srgb<u8> = Srgb { r: 0x3C, g: 0xB3, b: 0x71 };
    pub static MEDIUMSLATEBLUE:         Srgb<u8> = Srgb { r: 0x7B, g: 0x68, b: 0xEE };
    pub static MEDIUMSPRINGGREEN:       Srgb<u8> = Srgb { r: 0x00, g: 0xFA, b: 0x9A };
    pub static MEDIUMTURQUOISE:         Srgb<u8> = Srgb { r: 0x48, g: 0xD1, b: 0xCC };
    pub static MEDIUMVIOLETRED:         Srgb<u8> = Srgb { r: 0xC7, g: 0x15, b: 0x85 };
    pub static MIDNIGHTBLUE:            Srgb<u8> = Srgb { r: 0x19, g: 0x19, b: 0x70 };
    pub static MINTCREAM:               Srgb<u8> = Srgb { r: 0xF5, g: 0xFF, b: 0xFA };
    pub static MISTYROSE:               Srgb<u8> = Srgb { r: 0xFF, g: 0xE4, b: 0xE1 };
    pub static MOCCASIN:                Srgb<u8> = Srgb { r: 0xFF, g: 0xE4, b: 0xB5 };
    pub static NAVAJOWHITE:             Srgb<u8> = Srgb { r: 0xFF, g: 0xDE, b: 0xAD };
    pub static NAVY:                    Srgb<u8> = Srgb { r: 0x00, g: 0x00, b: 0x80 };
    pub static OLDLACE:                 Srgb<u8> = Srgb { r: 0xFD, g: 0xF5, b: 0xE6 };
    pub static OLIVE:                   Srgb<u8> = Srgb { r: 0x80, g: 0x80, b: 0x00 };
    pub static OLIVEDRAB:               Srgb<u8> = Srgb { r: 0x6B, g: 0x8E, b: 0x23 };
    pub static ORANGE:                  Srgb<u8> = Srgb { r: 0xFF, g: 0xA5, b: 0x00 };
    pub static ORANGERED:               Srgb<u8> = Srgb { r: 0xFF, g: 0x45, b: 0x00 };
    pub static ORCHID:                  Srgb<u8> = Srgb { r: 0xDA, g: 0x70, b: 0xD6 };
    pub static PALEGOLDENROD:           Srgb<u8> = Srgb { r: 0xEE, g: 0xE8, b: 0xAA };
    pub static PALEGREEN:               Srgb<u8> = Srgb { r: 0x98, g: 0xFB, b: 0x98 };
    pub static PALEVIOLETRED:           Srgb<u8> = Srgb { r: 0xDB, g: 0x70, b: 0x93 };
    pub static PAPAYAWHIP:              Srgb<u8> = Srgb { r: 0xFF, g: 0xEF, b: 0xD5 };
    pub static PEACHPUFF:               Srgb<u8> = Srgb { r: 0xFF, g: 0xDA, b: 0xB9 };
    pub static PERU:                    Srgb<u8> = Srgb { r: 0xCD, g: 0x85, b: 0x3F };
    pub static PINK:                    Srgb<u8> = Srgb { r: 0xFF, g: 0xC0, b: 0xCB };
    pub static PLUM:                    Srgb<u8> = Srgb { r: 0xDD, g: 0xA0, b: 0xDD };
    pub static POWDERBLUE:              Srgb<u8> = Srgb { r: 0xB0, g: 0xE0, b: 0xE6 };
    pub static PURPLE:                  Srgb<u8> = Srgb { r: 0x80, g: 0x00, b: 0x80 };
    pub static RED:                     Srgb<u8> = Srgb { r: 0xFF, g: 0x00, b: 0x00 };
    pub static ROSYBROWN:               Srgb<u8> = Srgb { r: 0xBC, g: 0x8F, b: 0x8F };
    pub static ROYALBLUE:               Srgb<u8> = Srgb { r: 0x41, g: 0x69, b: 0xE1 };
    pub static SADDLEBROWN:             Srgb<u8> = Srgb { r: 0x8B, g: 0x45, b: 0x13 };
    pub static SALMON:                  Srgb<u8> = Srgb { r: 0xFA, g: 0x80, b: 0x72 };
    pub static SANDYBROWN:              Srgb<u8> = Srgb { r: 0xFA, g: 0xA4, b: 0x60 };
    pub static SEAGREEN:                Srgb<u8> = Srgb { r: 0x2E, g: 0x8B, b: 0x57 };
    pub static SEASHELL:                Srgb<u8> = Srgb { r: 0xFF, g: 0xF5, b: 0xEE };
    pub static SIENNA:                  Srgb<u8> = Srgb { r: 0xA0, g: 0x52, b: 0x2D };
    pub static SILVER:                  Srgb<u8> = Srgb { r: 0xC0, g: 0xC0, b: 0xC0 };
    pub static SKYBLUE:                 Srgb<u8> = Srgb { r: 0x87, g: 0xCE, b: 0xEB };
    pub static SLATEBLUE:               Srgb<u8> = Srgb { r: 0x6A, g: 0x5A, b: 0xCD };
    pub static SLATEGRAY:               Srgb<u8> = Srgb { r: 0x70, g: 0x80, b: 0x90 };
    pub static SNOW:                    Srgb<u8> = Srgb { r: 0xFF, g: 0xFA, b: 0xFA };
    pub static SPRINGGREEN:             Srgb<u8> = Srgb { r: 0x00, g: 0xFF, b: 0x7F };
    pub static STEELBLUE:               Srgb<u8> = Srgb { r: 0x46, g: 0x82, b: 0xB4 };
    pub static TAN:                     Srgb<u8> = Srgb { r: 0xD2, g: 0xB4, b: 0x8C };
    pub static TEAL:                    Srgb<u8> = Srgb { r: 0x00, g: 0x80, b: 0x80 };
    pub static THISTLE:                 Srgb<u8> = Srgb { r: 0xD8, g: 0xBF, b: 0xD8 };
    pub static TOMATO:                  Srgb<u8> = Srgb { r: 0xFF, g: 0x63, b: 0x47 };
    pub static TURQUOISE:               Srgb<u8> = Srgb { r: 0x40, g: 0xE0, b: 0xD0 };
    pub static VIOLET:                  Srgb<u8> = Srgb { r: 0xEE, g: 0x82, b: 0xEE };
    pub static WHEAT:                   Srgb<u8> = Srgb { r: 0xF5, g: 0xDE, b: 0xB3 };
    pub static WHITE:                   Srgb<u8> = Srgb { r: 0xFF, g: 0xFF, b: 0xFF };
    pub static WHITESMOKE:              Srgb<u8> = Srgb { r: 0xF5, g: 0xF5, b: 0xF5 };
    pub static YELLOW:                  Srgb<u8> = Srgb { r: 0xFF, g: 0xFF, b: 0x00 };
    pub static YELLOWGREEN:             Srgb<u8> = Srgb { r: 0x9A, g: 0xCD, b: 0x32 };
}