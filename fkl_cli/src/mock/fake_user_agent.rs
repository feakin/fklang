use phf::phf_map;
use rand::Rng;

/**
 * Copyright (c) 2022 Faker
 *
 * This is a version of the original code migrated to TypeScript and modified
 * by the Faker team.
 *
 * Check LICENSE for more details about the copyright.
 *
 * -----------------------------------------------------------------------------
 *
 * Copyright (c) 2012-2014 Jeffrey Mealo
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
 * THE SOFTWARE.
 *
 * -----------------------------------------------------------------------------
 *
 * Based loosely on Luka Pusic's PHP Script:
 * http://360percents.com/posts/php-random-user-agent-generator/
 *
 * The license for that script is as follows:
 *
 * "THE BEER-WARE LICENSE" (Revision 42):
 *
 * <pusic93@gmail.com> wrote this file. As long as you retain this notice you
 * can do whatever you want with this stuff. If we meet some day, and you think
 * this stuff is worth it, you can buy me a beer in return. Luka Pusic
 */

/// init os with weighted
static CHROME_OS_WEIGHTED: phf::Map<&'static str, f32> = phf_map! {
    "win" => 0.89,
    "mac" => 0.09,
    "lin" => 0.02,
};

static FIREFOX_OS_WEIGHTED: phf::Map<&'static str, f32> = phf_map! {
    "win" => 0.83,
    "mac" => 0.16,
    "lin" => 0.01,
};

static OPERA_OS_WEIGHTED: phf::Map<&'static str, f32> = phf_map! {
    "win" => 0.91,
    "mac" => 0.03,
    "lin" => 0.06,
};

static SAFARI_OS_WEIGHTED: phf::Map<&'static str, f32> = phf_map! {
    "win" => 0.04,
    "mac" => 0.96,
    "lin" => 0.00
};

// bowser weighted
//  chrome: 0.45132810566,
//  iexplorer: 0.27477061836,
//  firefox: 0.19384170608,
//  safari: 0.06186781118,
//  opera: 0.01574236955,
static BORWSER_WEIGHTED: phf::Map<&'static str, f32> = phf_map! {
    "chrome" => 0.45132810566,
    "iexplorer" => 0.27477061836,
    "firefox" => 0.19384170608,
    "safari" => 0.06186781118,
    "opera" => 0.01574236955,
};

pub struct UserAgent {
  // pub os: String,
  // pub browser: String,
  // pub version: String,
}

impl UserAgent {
  pub fn random() -> String {
    let browser = UserAgent::browser();
    let os = UserAgent::os(&browser);

    match &*browser {
      "chrome" => UserAgent::chrome(&os),
      "firefox" => UserAgent::firefox(&os),
      "opera" => UserAgent::opera(&os),
      "safari" => UserAgent::safari(&os),
      _ => UserAgent::chrome(&os),
    }
  }

  pub fn weighted_key_from_object(obj: &phf::Map<&str, f32>) -> String {
    let mut rng = rand::thread_rng();
    let mut total = 0.0;
    let mut key = String::new();
    let weight = rng.gen::<f32>();

    for (k, v) in obj {
      total += v;
      if weight <= total {
        key = k.to_string();
        break;
      }
    }

    key
  }

  pub fn browser() -> String {
    let browser = Self::weighted_key_from_object(&BORWSER_WEIGHTED);
    browser
  }

  pub fn os(browser: &str) -> String {
    let os_version = match browser {
      "chrome" => Self::weighted_key_from_object(&CHROME_OS_WEIGHTED),
      "firefox" => Self::weighted_key_from_object(&FIREFOX_OS_WEIGHTED),
      "opera" => Self::weighted_key_from_object(&OPERA_OS_WEIGHTED),
      "safari" => Self::weighted_key_from_object(&SAFARI_OS_WEIGHTED),
      _ => String::new(),
    };

    os_version
  }

  pub fn firefox(os: &str) -> String {
    let firefox_ver = format!(
      "{}{}",
      rand::thread_rng().gen_range(5..15),
      Self::random_revision(2)
    );
    let gecko_ver = format!("Gecko/20100101 Firefox/{}", firefox_ver);
    let proc = Self::random_proc(os);
    let os_ver = match os {
      "win" => format!(
        "(Windows NT {}; {})",
        VersionString::nt(),
        proc
      ),
      "mac" => format!(
        "(Macintosh; {} Mac OS X {}",
        proc,
        VersionString::osx()
      ),
      "lin" => format!("(X11; Linux {})", proc),
      _ => String::new(),
    };

    format!(
      "Mozilla/5.0 {} rv:{} {}",
      os_ver,
      firefox_ver.trim_end_matches("2"),
      gecko_ver
    )
  }

  pub fn chrome(os: &str) -> String {
    let safari = VersionString::safari();
    let os_ver = match os {
      "mac" => format!(
        "(Macintosh; {} Mac OS X {})",
        Self::random_proc("mac"),
        VersionString::osx()
      ),
      "win" => format!(
        "(Windows; U; Windows NT {})",
        VersionString::nt()
      ),
      "lin" => format!(
        "(X11; Linux {})",
        Self::random_proc(os)
      ),
      _ => String::new(),
    };

    format!(
      "Mozilla/5.0 {} AppleWebKit/{} (KHTML, like Gecko) Chrome/{} Safari/{}",
      os_ver,
      safari,
      VersionString::chrome(),
      safari
    )
  }

  pub fn safari(os: &str) -> String {
    let safari = VersionString::safari();
    let ver = format!(
      "{}.{}.{}",
      rand::thread_rng().gen_range(4..7),
      rand::thread_rng().gen_range(0..1),
      rand::thread_rng().gen_range(0..10),
    );
    let os_ver = match os {
      "mac" => format!(
        "(Macintosh; {} Mac OS X {} rv:{}; {})",
        Self::random_proc("mac"),
        VersionString::osx(),
        rand::thread_rng().gen_range(2..6),
        random_lang()
      ),
      "win" => format!(
        "(Windows; U; Windows NT {})",
        VersionString::nt()
      ),
      _ => String::new(),
    };

    format!(
      "Mozilla/5.0 {} AppleWebKit/{} (KHTML, like Gecko) Version/{} Safari/{}",
      os_ver,
      safari,
      ver,
      safari
    )
  }

  pub fn opera(os: &str) -> String {
    let presto_ver = format!(
      " Presto/{} Version/{}",
      VersionString::presto(),
      VersionString::presto2()
    );
    let os_ver = match os {
      "win" => format!(
        "(Windows NT {}; U; {}{})",
        VersionString::nt(),
        random_lang(),
        presto_ver
      ),
      "lin" => format!(
        "(X11; Linux {}; U; {}{})",
        Self::random_proc(os),
        random_lang(),
        presto_ver
      ),
      "mac" => format!(
        "(Macintosh; Intel Mac OS X {}; U; {} Presto/{} Version/{})",
        VersionString::osx(),
        random_lang(),
        VersionString::presto(),
        VersionString::presto2()
      ),
      _ => String::new(),
    };

    format!(
      "Opera/{}.{} {}",
      rand::thread_rng().gen_range(9..14),
      rand::thread_rng().gen_range(0..99),
      os_ver
    )
  }

  fn random_revision(dots: i32) -> String {
    let mut return_val = String::new();
    for _ in 0..dots {
      return_val.push_str(&format!(".{}", rand::thread_rng().gen_range(0..9)));
    }
    return_val
  }

  fn random_proc(arch: &str) -> String {
    let procs = match arch {
      "lin" => vec!["i686", "x86_64"],
      "mac" => vec!["Intel", "PPC", "U; Intel", "U; PPC"],
      "win" => vec!["", "WOW64", "Win64; x64"],
      _ => vec![],
    };

    let proc = procs[rand::thread_rng().gen_range(0..procs.len())];
    proc.to_string()
  }
}

pub fn random_lang() -> String {
  let langs = vec![
    "en-US",
    "en-GB",
    "de-DE",
    "fr-FR",
    "es-ES",
    "it-IT",
    "ja-JP",
    "ko-KR",
    "zh-CN",
    "zh-TW",
  ];

  let lang = langs[rand::thread_rng().gen_range(0..langs.len())];
  lang.to_string()
}

pub struct VersionString {}

impl VersionString {
  pub fn net() -> String {
    let mut rng = rand::thread_rng();
    let net = format!(
      "{}.{}.{}.{}",
      rng.gen_range(1..=4),
      rng.gen_range(0..=9),
      rng.gen_range(10000..=99999),
      rng.gen_range(0..=9)
    );

    net
  }

  pub fn nt() -> String {
    let mut rng = rand::thread_rng();
    let nt = format!("{}.{}", rng.gen_range(5..=6), rng.gen_range(0..=3));

    nt
  }

  pub fn ie() -> String {
    let mut rng = rand::thread_rng();
    let ie = format!("{}", rng.gen_range(7..=11));

    ie
  }

  pub fn trident() -> String {
    let mut rng = rand::thread_rng();
    let trident = format!("{}.{}", rng.gen_range(3..=7), rng.gen_range(0..=1));

    trident
  }

  pub fn osx() -> String {
    let mut rng = rand::thread_rng();
    let osx = format!("10.{}.{}", rng.gen_range(5..=10), rng.gen_range(0..=9));

    osx
  }

  pub fn chrome() -> String {
    let mut rng = rand::thread_rng();
    let chrome = format!(
      "{}.0.{}.0",
      rng.gen_range(13..=39),
      rng.gen_range(800..=899)
    );

    chrome
  }

  pub fn presto() -> String {
    let mut rng = rand::thread_rng();
    let presto = format!("2.9.{}", rng.gen_range(160..=190));

    presto
  }

  pub fn presto2() -> String {
    let mut rng = rand::thread_rng();
    let presto2 = format!("{}.00", rng.gen_range(10..=12));

    presto2
  }

  pub fn safari() -> String {
    let mut rng = rand::thread_rng();
    let safari = format!(
      "{}.{}.{}",
      rng.gen_range(531..=538),
      rng.gen_range(0..=2),
      rng.gen_range(0..=2)
    );

    safari
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use phf::phf_map;

  #[test]
  fn test_weighted_key_from_object() {
    let obj = phf_map! {
      "win" => 0.89,
      "mac" => 0.09,
      "lin" => 0.02,
    };

    let key = UserAgent::weighted_key_from_object(&obj);
    assert!(key == "win" || key == "mac" || key == "lin");
  }

  #[test]
  fn test_browser() {
    let browser = UserAgent::browser();
    assert!(browser == "chrome" || browser == "iexplorer" || browser == "firefox" || browser == "safari" || browser == "opera");
  }

  #[test]
  fn test_os_version() {
    let os_version = UserAgent::os("chrome");
    assert!(os_version == "win" || os_version == "mac" || os_version == "lin");
  }

  #[test]
  fn test_chrome() {
    let chrome = UserAgent::chrome("win");
    println!("{}", chrome);
    assert!(chrome.contains("Mozilla/5.0 (Windows; U; Windows NT"));
    assert!(chrome.contains("Chrome"));
  }

  #[test]
  fn test_firefox() {
    let firefox = UserAgent::firefox("win");
    println!("{}", firefox);
    assert!(firefox.contains("Mozilla/5.0 (Windows NT"));
    assert!(firefox.contains("Firefox"));
  }

  #[test]
  fn test_safari() {
    let safari = UserAgent::safari("win");
    println!("{}", safari);
    assert!(safari.contains("Mozilla/5.0 (Windows;"));
    assert!(safari.contains("Safari"));
  }

  #[test]
  fn test_opera() {
    let opera = UserAgent::opera("win");
    assert!(opera.contains("Opera"));
  }

  #[test]
  fn test_random() {
    let random_agent = UserAgent::random();
    println!("{}", random_agent);
    assert!(random_agent.contains("Mozilla/5.0 "));
  }
}
