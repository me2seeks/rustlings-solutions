// 这与之前的 `from_into` 练习类似。
// 但这次，我们将实现(impl) `FromStr` 并返回错误，而不是回退到默认值。
// 此外，在实现 `FromStr` 之后，你可以使用字符串上的 `parse` 方法来生成实现该特性的类型的对象。
// 你可以在文档中阅读更多关于它的内容:
// https://doc.rust-lang.org/std/str/trait.FromStr.html

use std::num::ParseIntError;
use std::str::FromStr;
use std::u8;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: u8,
}

// 我们将把这种错误类型用于 `FromStr` 实现(impl)中。
#[derive(Debug, PartialEq)]
enum ParsePersonError {
    // 字段数量不正确
    BadLen,
    // 姓名字段为空
    NoName,
    // 来自 `parse::<u8>()` 的包装错误
    ParseInt(ParseIntError),
}

// TODO: 完成这个 `FromStr` 实现，以便能够从形如 "Mark, 20" 的字符串中解析出一个 `Person`。
// 注意，你需要使用类似 `"4".parse::<u8>()` 的方式将年龄部分解析为 `u8` 类型。
//
// 步骤：
// 1. 根据字符串中存在的逗号对给定字符串进行分割(split)。
// 2. 如果分割操作返回的元素少于或多于2个，返回错误 `ParsePersonError::BadLen`。
// 3. 将分割操作得到的第一个元素用作姓名。
// 4. 如果姓名为空，返回错误 `ParsePersonError::NoName`。
// 5. 将分割操作得到的第二个元素解析为 `u8` 类型作为年龄。
// 6. 如果年龄解析失败，返回错误 `ParsePersonError::ParseInt`。
impl FromStr for Person {
    type Err = ParsePersonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(',');
        let (Some(name), Some(age), None) = (split.next(), split.next(), split.next()) else {
            return Err(ParsePersonError::BadLen);
        };

        if name.is_empty() {
            return Err(ParsePersonError::NoName);
        }

        match age.parse::<u8>() {
            Ok(age) => Ok(Self {
                name: name.into(),
                age,
            }),
            Err(e) => Err(ParsePersonError::ParseInt(e)),
        }
    }
}

fn main() {
    let p = "Mark,20".parse::<Person>();
    println!("{p:?}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use ParsePersonError::*;

    #[test]
    fn empty_input() {
        assert_eq!("".parse::<Person>(), Err(BadLen));
    }

    #[test]
    fn good_input() {
        let p = "John,32".parse::<Person>();
        assert!(p.is_ok());
        let p = p.unwrap();
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 32);
    }

    #[test]
    fn missing_age() {
        assert!(matches!("John,".parse::<Person>(), Err(ParseInt(_))));
    }

    #[test]
    fn invalid_age() {
        assert!(matches!("John,twenty".parse::<Person>(), Err(ParseInt(_))));
    }

    #[test]
    fn missing_comma_and_age() {
        assert_eq!("John".parse::<Person>(), Err(BadLen));
    }

    #[test]
    fn missing_name() {
        assert_eq!(",1".parse::<Person>(), Err(NoName));
    }

    #[test]
    fn missing_name_and_age() {
        assert!(matches!(",".parse::<Person>(), Err(NoName | ParseInt(_))));
    }

    #[test]
    fn missing_name_and_invalid_age() {
        assert!(matches!(
            ",one".parse::<Person>(),
            Err(NoName | ParseInt(_)),
        ));
    }

    #[test]
    fn trailing_comma() {
        assert_eq!("John,32,".parse::<Person>(), Err(BadLen));
    }

    #[test]
    fn trailing_comma_and_some_string() {
        assert_eq!("John,32,man".parse::<Person>(), Err(BadLen));
    }
}
