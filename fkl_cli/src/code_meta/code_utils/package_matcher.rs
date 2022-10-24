/* Rewrite from Java version
 * Copyright 2014-2021 TNG Technology Consulting GmbH
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
use regex::Regex;

pub fn is_package_match(package_identifier: String, text: &str) -> bool {
    let package = convert_to_regex(package_identifier);
    let regex = Regex::new(package.as_str())
        .expect("regex error");

    regex.is_match(text)
}

pub fn is_assert_match(package_identifier: String, text: &str, assert_package: String) -> bool {
    let package = convert_to_regex(package_identifier);
    let regex = Regex::new(package.as_str())
        .expect("regex error");

    if let Some(caps) = regex.captures(text) {
        for ma in caps.iter() {
            if let Some(match_) = ma {
                if match_.as_str() == assert_package {
                    return true
                }
            }
        }
    }

    return false;
}

pub fn convert_to_regex(package_identifier: String) -> String {
    let replaced = package_identifier
        .replace("(**)", "#%#%#")
        .replace("*", "\\w+")
        .replace(".", "\\.")
        .replace("#%#%#", "(\\w+(?:\\.\\w+)*)")
        .replace("\\.\\.", "(?:(?:^\\w*)?\\.(?:\\w+\\.)*(?:\\w*$)?)?");

    format!("^{}$", replaced)
}

#[cfg(test)]
mod tests {
    use crate::code_meta::package_matcher::{is_package_match, is_assert_match};

    #[test]
    fn should_match() {
        let values = vec![
            "some.arbitrary.pkg | some.arbitrary.pkg | true",
            "some.arbitrary.pkg | some.thing.different | false",
            "some..pkg | some.arbitrary.pkg | true",
            "some..middle..pkg | some.arbitrary.middle.more.pkg | true",
            "*..pkg | some.arbitrary.pkg | true",
            "some..* | some.arbitrary.pkg | true",
            "*..pkg | some.arbitrary.pkg.toomuch | false",
            "toomuch.some..* | some.arbitrary.pkg | false",
            "*..wrong | some.arbitrary.pkg | false",
            "some..* | wrong.arbitrary.pkg | false",
            "..some | some | true",
            "some.. | some | true",
            "*..some | some | false",
            "some..* | some | false",
            "..some | asome | false",
            "some.. | somea | false",
            "*.*.* | wrong.arbitrary.pkg | true",
            "*.*.* | wrong.arbitrary.pkg.toomuch | false",
            "some.arbi*.pk*.. | some.arbitrary.pkg.whatever | true",
            "some.arbi*.. | some.brbitrary.pkg | false",
            "some.*rary.*kg.. | some.arbitrary.pkg.whatever | true",
            "some.*rary.. | some.arbitrarz.pkg | false",
            "some.pkg | someepkg | false",
            "..pkg.. | some.random.pkg.maybe.anywhere | true",
            "..p.. | s.r.p.m.a | true",
            "*..pkg..* | some.random.pkg.maybe.anywhere | true",
            "*..p..* | s.r.p.m.a | true"];

        for value in values {
            let split = value.split(" | ");
            let vec = split.collect::<Vec<&str>>();

            let assert: bool = vec[2]
                .parse()
                .expect("convert bool error");

            assert_eq!(assert, is_package_match(vec[0].to_string(), vec[1]));
        }
    }

    #[test]
    fn should_match_group() {
        let assert_match = is_assert_match("some.(*).pkg".to_string(),
                                           "some.arbitrary.pkg",
                                           "arbitrary".to_string()
        );

        assert_eq!(true, assert_match);
    }
}
