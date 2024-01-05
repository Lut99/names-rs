//  MOD.rs
//    by Lut99
//
//  Created:
//    05 Jan 2024, 16:12:55
//  Last edited:
//    05 Jan 2024, 16:40:42
//  Auto updated?
//    Yes
//
//  Description:
//!   Defines three-letter names.
//


/***** CASE MODULES *****/
/// Defines the three-letter names in UPPERCASE.
///
/// # Example
/// ```rust
/// use names::three::uppercase;
///
/// assert_eq!(uppercase::AMY, "AMY");
/// assert_eq!(uppercase::DAN, "DAN");
/// ```
#[cfg(feature = "three-uppercase")]
pub mod uppercase {
    /// Uppercase version of `Amy`.
    pub const AMY: &'static str = "AMY";
    /// Uppercase version of `Bob`.
    pub const BOB: &'static str = "BOB";
    /// Uppercase version of `Cho`.
    pub const CHO: &'static str = "CHO";
    /// Uppercase version of `Dan`.
    pub const DAN: &'static str = "DAN";
    /// Uppercase version of `Eve`.
    pub const EVE: &'static str = "EVE";
    /// Uppercase version of `Fey`.
    pub const FEY: &'static str = "FEY";
    /// Uppercase version of `Guy`.
    pub const GUY: &'static str = "GUY";
    /// Uppercase version of `Han`.
    pub const HAN: &'static str = "HAN";
    /// Uppercase version of `Ian`.
    pub const IAN: &'static str = "IAN";
    /// Uppercase version of `Joe`.
    pub const JOE: &'static str = "JOE";
    /// Uppercase version of `Ken`.
    pub const KEN: &'static str = "KEN";
    /// Uppercase version of `Lea`.
    pub const LEA: &'static str = "LEA";
    /// Uppercase version of `Mel`.
    pub const MEL: &'static str = "MEL";
    /// Uppercase version of `Noa`.
    pub const NOA: &'static str = "NOA";
    /// Uppercase version of `Oni`.
    pub const ONI: &'static str = "ONI";
    /// Uppercase version of `Pam`.
    pub const PAM: &'static str = "PAM";
    /// Uppercase version of `Qin`.
    pub const QIN: &'static str = "QIN";
    /// Uppercase version of `Ron`.
    pub const RON: &'static str = "RON";
    /// Uppercase version of `Sam`.
    pub const SAM: &'static str = "SAM";
    /// Uppercase version of `Tim`.
    pub const TIM: &'static str = "TIM";
    /// Uppercase version of `Uwe`.
    pub const UWE: &'static str = "UWE";
    /// Uppercase version of `Vic`.
    pub const VIC: &'static str = "VIC";
    /// Uppercase version of `Wes`.
    pub const WES: &'static str = "WES";
    /// Uppercase version of `Xin`.
    pub const XIN: &'static str = "XIN";
    /// Uppercase version of `Yas`.
    pub const YAS: &'static str = "YAS";
    /// Uppercase version of `Zoe`.
    pub const ZOE: &'static str = "ZOE";


    /// Lists all uppercase names in this module.
    ///
    /// # Returns
    /// A `'static` slice of [`&static str`](str)s that list all the names.
    #[inline]
    pub const fn all() -> &'static [&'static str] {
        &[AMY, BOB, CHO, DAN, EVE, FEY, GUY, HAN, IAN, JOE, KEN, LEA, MEL, NOA, ONI, PAM, QIN, RON, SAM, TIM, UWE, VIC, WES, XIN, YAS, ZOE]
    }
}

/// Defines the three-letter names in proper case (i.e., only first character is capitalized).
///
/// # Example
/// ```rust
/// use names::three::capitalcase;
///
/// assert_eq!(capitalcase::AMY, "Amy");
/// assert_eq!(capitalcase::DAN, "Dan");
/// ```
#[cfg(feature = "three-usualcase")]
pub mod usualcase {
    /// The name `Amy`.
    pub const AMY: &'static str = "Amy";
    /// The name `Bob`.
    pub const BOB: &'static str = "Bob";
    /// The name `Cho`.
    pub const CHO: &'static str = "Cho";
    /// The name `Dan`.
    pub const DAN: &'static str = "Dan";
    /// The name `Eve`.
    pub const EVE: &'static str = "Eve";
    /// The name `Fey`.
    pub const FEY: &'static str = "Fey";
    /// The name `Guy`.
    pub const GUY: &'static str = "Guy";
    /// The name `Han`.
    pub const HAN: &'static str = "Han";
    /// The name `Ian`.
    pub const IAN: &'static str = "Ian";
    /// The name `Joe`.
    pub const JOE: &'static str = "Joe";
    /// The name `Ken`.
    pub const KEN: &'static str = "Ken";
    /// The name `Lea`.
    pub const LEA: &'static str = "Lea";
    /// The name `Mel`.
    pub const MEL: &'static str = "Mel";
    /// The name `Noa`.
    pub const NOA: &'static str = "Noa";
    /// The name `Oni`.
    pub const ONI: &'static str = "Oni";
    /// The name `Pam`.
    pub const PAM: &'static str = "Pam";
    /// The name `Qin`.
    pub const QIN: &'static str = "Qin";
    /// The name `Ron`.
    pub const RON: &'static str = "Ron";
    /// The name `Sam`.
    pub const SAM: &'static str = "Sam";
    /// The name `Tim`.
    pub const TIM: &'static str = "Tim";
    /// The name `Uwe`.
    pub const UWE: &'static str = "Uwe";
    /// The name `Vic`.
    pub const VIC: &'static str = "Vic";
    /// The name `Wes`.
    pub const WES: &'static str = "Wes";
    /// The name `Xin`.
    pub const XIN: &'static str = "Xin";
    /// The name `Yas`.
    pub const YAS: &'static str = "Yas";
    /// The name `Zoe`.
    pub const ZOE: &'static str = "Zoe";


    /// Lists all usualcase names in this module.
    ///
    /// # Returns
    /// A `'static` slice of [`&static str`](str)s that list all the names.
    #[inline]
    pub const fn all() -> &'static [&'static str] {
        &[AMY, BOB, CHO, DAN, EVE, FEY, GUY, HAN, IAN, JOE, KEN, LEA, MEL, NOA, ONI, PAM, QIN, RON, SAM, TIM, UWE, VIC, WES, XIN, YAS, ZOE]
    }
}
#[cfg(feature = "three-usualcase")]
pub use usualcase::*;

/// Defines the three-letter names in lowercase.
///
/// # Example
/// ```rust
/// use names::three::uppercase;
///
/// assert_eq!(uppercase::AMY, "amy");
/// assert_eq!(uppercase::DAN, "dan");
/// ```
#[cfg(feature = "three-lowercase")]
pub mod lowercase {
    /// Lowercase version of `Amy`.
    pub const AMY: &'static str = "amy";
    /// Lowercase version of `Bob`.
    pub const BOB: &'static str = "bob";
    /// Lowercase version of `Cho`.
    pub const CHO: &'static str = "cho";
    /// Lowercase version of `Dan`.
    pub const DAN: &'static str = "dan";
    /// Lowercase version of `Eve`.
    pub const EVE: &'static str = "eve";
    /// Lowercase version of `Fey`.
    pub const FEY: &'static str = "fey";
    /// Lowercase version of `Guy`.
    pub const GUY: &'static str = "guy";
    /// Lowercase version of `Han`.
    pub const HAN: &'static str = "han";
    /// Lowercase version of `Ian`.
    pub const IAN: &'static str = "ian";
    /// Lowercase version of `Joe`.
    pub const JOE: &'static str = "joe";
    /// Lowercase version of `Ken`.
    pub const KEN: &'static str = "ken";
    /// Lowercase version of `Lea`.
    pub const LEA: &'static str = "lea";
    /// Lowercase version of `Mel`.
    pub const MEL: &'static str = "mel";
    /// Lowercase version of `Noa`.
    pub const NOA: &'static str = "noa";
    /// Lowercase version of `Oni`.
    pub const ONI: &'static str = "oni";
    /// Lowercase version of `Pam`.
    pub const PAM: &'static str = "pam";
    /// Lowercase version of `Qin`.
    pub const QIN: &'static str = "qin";
    /// Lowercase version of `Ron`.
    pub const RON: &'static str = "ron";
    /// Lowercase version of `Sam`.
    pub const SAM: &'static str = "sam";
    /// Lowercase version of `Tim`.
    pub const TIM: &'static str = "tim";
    /// Lowercase version of `Uwe`.
    pub const UWE: &'static str = "uwe";
    /// Lowercase version of `Vic`.
    pub const VIC: &'static str = "vic";
    /// Lowercase version of `Wes`.
    pub const WES: &'static str = "wes";
    /// Lowercase version of `Xin`.
    pub const XIN: &'static str = "xin";
    /// Lowercase version of `Yas`.
    pub const YAS: &'static str = "yas";
    /// Lowercase version of `Zoe`.
    pub const ZOE: &'static str = "zoe";


    /// Lists all lowercase names in this module.
    ///
    /// # Returns
    /// A `'static` slice of [`&static str`](str)s that list all the names.
    #[inline]
    pub const fn all() -> &'static [&'static str] {
        &[AMY, BOB, CHO, DAN, EVE, FEY, GUY, HAN, IAN, JOE, KEN, LEA, MEL, NOA, ONI, PAM, QIN, RON, SAM, TIM, UWE, VIC, WES, XIN, YAS, ZOE]
    }
}
