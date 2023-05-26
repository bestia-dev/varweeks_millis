// region: auto_md_to_doc_comments include README.md A //!
//! # varweeks_millis
//!
//! **2021 new date time units and formats: varweeks and millis**  
//! ***version: 1.0.13 date: 2023-05-26 author: [bestia.dev](https://bestia.dev) repository: [Github](https://github.com/bestia-dev/varweeks_millis)***  
//!
//! [![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-181-green.svg)]()
//! [![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-233-blue.svg)]()
//! [![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-29-purple.svg)]()
//! [![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)]()
//! [![Lines in tests](https://img.shields.io/badge/Lines_in_tests-100-orange.svg)]()
//!
//! [![crates.io](https://img.shields.io/crates/v/varweeks_millis.svg)](https://crates.io/crates/varweeks_millis)
//! [![Documentation](https://docs.rs/varweeks_millis/badge.svg)](https://docs.rs/varweeks_millis/)
//! [![crev reviews](https://web.crev.dev/rust-reviews/badge/crev_count/varweeks_millis.svg)](https://web.crev.dev/rust-reviews/crate/varweeks_millis/)
//! [![RustActions](https://github.com/bestia-dev/varweeks_millis/workflows/rust/badge.svg)](https://github.com/bestia-dev/varweeks_millis/)
//! [![latest doc](https://img.shields.io/badge/latest_docs-GitHub-orange.svg)](https://bestia-dev.github.io/varweeks_millis/varweeks_millis/index.html)
//! [![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/bestia-dev/varweeks_millis/blob/master/LICENSE)
//! ![Hits](https://bestia.dev/webpage_hit_counter/get_svg_image/763950777.svg)
//!
//! Hashtags: #rustlang #tutorial #datetime  
//! My projects on Github are more like a tutorial than a finished product: [bestia-dev tutorials](https://github.com/bestia-dev/tutorials_rust_wasm).
//!
//! ## Proposal for a new date and time units and formats
//!
//! varweeks and millis are my suggestion for new date-time units and formats.  
//! Here is a long read TL;DR about the date-time reform proposal:  
//! <https://github.com/bestia-dev/new_date_time_units_and_formats/>
//!
//! A super short explanation:  
//! Years (beginning and end) remain the same as in the CE - common era calendar.  
//! Months are obsolete and they are not used in the new format.  
//! varweek is similar to week. One year has 52 full varweeks with 7 days.  
//! The exception is the last 53rd varweek that has only 1 celebration day. For leap years there are 2 celebration days.  
//! The new year always starts with `01v 1d` - short pronunciation one-vee one-dee. Basically every year starts with a monday.  
//! varweek-days are similar to week-days, but without names. They use just numbers and the `d` unit.  
//! `1d` is the new name for Monday and `7d` is the new name for Sunday.  
//! The varweek_date format is global for every language and looks exactly like this:  
//! `2021c 52v 2d`  
//! The year must have 4 digits from 1000c to 9999c. The unit `c` stands for CE - common era. Space.  
//! The varweek must have 2 digits from 01v to 53v. The unit `v` stands for varweek. Space.  
//! The day must have 1 digit from 1d to 7d. The unit `d` stands for day. End.  
//!
//! Hours, minutes and seconds are obsolete.  
//! One day is divided into 1000md.  
//! Millis is the short name for milliday. The unit is `md`.  
//! For shorter time intervals there is microdays or micros, unit `μd`. `1md` has `1000μd`.  
//!
//! This crate contains functions to use with the new units varweeks and millis.  
//! It is dependent on the crate `chrono` for `NaiveDate` and `NaiveTime`.  
//!
//! ## Used in projects
//!
//! PWA wasm converter and lengthy explanation:  
//! <https://github.com/bestia-dev/new_date_time_units_and_formats/>  
//! PWA wasm clock:  
//! <https://github.com/bestia-dev/varweeks_millis_clock>  
//! ![screenshot](https://github.com/bestia-dev/varweeks_millis_clock/raw/main/images/compare_clocks.png)
//!
//! ## cargo crev reviews and advisory
//!
//! It is recommended to always use [cargo-crev](https://github.com/crev-dev/cargo-crev)  
//! to verify the trustworthiness of each of your dependencies.  
//! Please, spread this info.  
//! On the web use this url to read crate reviews. Example:  
//! <https://web.crev.dev/rust-reviews/crate/num-traits/>  
//!
//! ## Open-source and free as a beer
//!
//! My open-source projects are free as a beer (MIT license).  
//! I just love programming.  
//! But I need also to drink. If you find my projects and tutorials helpful, please buy me a beer by donating to my [PayPal](https://paypal.me/LucianoBestia).  
//! You know the price of a beer in your local bar ;-)  
//! So I can drink a free beer for your health :-)  
//! [Na zdravje!](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) [Alla salute!](https://dictionary.cambridge.org/dictionary/italian-english/alla-salute) [Prost!](https://dictionary.cambridge.org/dictionary/german-english/prost) [Nazdravlje!](https://matadornetwork.com/nights/how-to-say-cheers-in-50-languages/) 🍻
//!
//! [//bestia.dev](https://bestia.dev)  
//! [//github.com/bestia-dev](https://github.com/bestia-dev)  
//! [//bestiadev.substack.com](https://bestiadev.substack.com)  
//! [//youtube.com/@bestia-dev-tutorials](https://youtube.com/@bestia-dev-tutorials)  
//!
// endregion: auto_md_to_doc_comments include README.md A //!

mod micro_time_mod;
mod milli_time_mod;
mod varweek_date_mod;

pub use micro_time_mod::*;
pub use milli_time_mod::*;
pub use varweek_date_mod::*;
