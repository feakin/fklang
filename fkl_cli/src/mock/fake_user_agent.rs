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
  pub os: String,
  pub os_version: String,
  pub browser: String,
  pub browser_version: String,
  pub device: String,
  pub device_version: String,
  pub engine: String,
  pub engine_version: String,
}

impl UserAgent {
  // pub fn new() -> Self {
  //   let os = Self::browser();
  //   let os_version = Self::os_version(&os);
  //   let browser = Self::browser();
  //   let browser_version = Self::browser_version(&browser);
  //   let device = Self::device();
  //   let device_version = Self::device_version(&device);
  //   let engine = Self::engine();
  //   let engine_version = Self::engine_version(&engine);
  //
  //   Self {
  //     os,
  //     os_version,
  //     browser,
  //     browser_version,
  //     device,
  //     device_version,
  //     engine,
  //     engine_version,
  //   }
  // }

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

  pub fn os_version(browser: &str) -> String {
    let os_version = match browser {
      "chrome" => Self::weighted_key_from_object(&CHROME_OS_WEIGHTED),
      "firefox" => Self::weighted_key_from_object(&FIREFOX_OS_WEIGHTED),
      "opera" => Self::weighted_key_from_object(&OPERA_OS_WEIGHTED),
      "safari" => Self::weighted_key_from_object(&SAFARI_OS_WEIGHTED),
      _ => String::new(),
    };

    os_version
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
    let os_version = UserAgent::os_version("chrome");
    assert!(os_version == "win" || os_version == "mac" || os_version == "lin");
  }
}
