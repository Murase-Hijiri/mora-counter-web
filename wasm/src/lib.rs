use std::sync::OnceLock;
use vibrato::{Dictionary, Tokenizer};
use wasm_bindgen::prelude::*;

const DICT_BYTES: &[u8] = include_bytes!("../data/system.dic");

static TOKENIZER: OnceLock<Tokenizer> = OnceLock::new();

#[wasm_bindgen]
pub fn init_vibrato() -> Result<(), JsValue> {
    let dict = Dictionary::read(DICT_BYTES)
        .map_err(|e| JsValue::from_str(&format!("Failed to load dictionary: {}", e)))?;

    let tokenizer = Tokenizer::new(dict);

    TOKENIZER
        .set(tokenizer)
        .map_err(|_| JsValue::from_str("Failed to set tokenizer"))?;

    Ok(())
}

#[wasm_bindgen]
pub fn count_mora(text: &str) -> Result<usize, JsValue> {
    let tokenizer = TOKENIZER
        .get()
        .ok_or_else(|| JsValue::from_str("Tokenizer not initialized"))?;

    let mut worker = tokenizer.new_worker();
    worker.reset_sentence(text);
    worker.tokenize();

    let mut mora_count = 0;

    for token in worker.token_iter() {
        let pron = match token.feature().split(',').nth(8) {
            Some(val) => val,
            None => &to_katakana(token.surface())?,
        };
        mora_count += count_katakana_mora(pron);
    }

    Ok(mora_count)
}

fn count_katakana_mora(katakana: &str) -> usize {
    let mut count = 0;

    for c in katakana.chars() {
        match c {
            'ァ' | 'ィ' | 'ゥ' | 'ェ' | 'ォ' | 'ャ' | 'ュ' | 'ョ' | 'ヮ' => {}
            'ッ' => count += 1,
            'ー' => count += 1,
            _ if c >= 'ア' && c <= 'ヺ' => {
                count += 1;
            }
            _ => {}
        }
    }

    count
}

fn to_katakana(input: &str) -> Result<String, JsValue> {
    let mut converted_text: String = input
        .chars()
        .map(|c| {
            let code = c as u32;
            // ひらがなのUnicode範囲: 0x3041 (ぁ) 〜 0x3096 (ん)
            if (0x3041..=0x3096).contains(&code) {
                // 0x60 (96) を足すとカタカナの範囲に移動する
                std::char::from_u32(code + 0x60).unwrap_or(c)
            } else {
                c
            }
        })
        .collect();

    let is_katakana = converted_text
        .chars()
        .map(|c| {
            let code = c as u32;
            if (0x30A1..=0x30FA).contains(&code) {
                return true;
            } else {
                return false;
            }
        })
        .reduce(|acc, is_char_katakana| acc && is_char_katakana)
        .unwrap_or(true);

    if !is_katakana {
        let tokenizer = TOKENIZER
            .get()
            .ok_or_else(|| JsValue::from_str("Tokenizer not initialized"))?;

        converted_text = input
            .chars()
            .map(|c| {
                let mut worker = tokenizer.new_worker();
                worker.reset_sentence(c.to_string());
                worker.tokenize();
                match worker.token(0).feature().split(',').nth(8) {
                    Some(val) => val,
                    None => "",
                }
            })
            .collect();
    }

    Ok(converted_text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_katakana_mora_works() {
        let str1 = "クリスマス";
        let str2 = "ファシズム";
        let str3 = "フック";
        let str4 = "ビブラート";
        let str5 = "マイゴ！";

        assert_eq!(count_katakana_mora(str1), 5);
        assert_eq!(count_katakana_mora(str2), 4);
        assert_eq!(count_katakana_mora(str3), 3);
        assert_eq!(count_katakana_mora(str4), 5);
        assert_eq!(count_katakana_mora(str5), 3);
    }

    #[test]
    fn to_katakana_works() {
        let str1 = "いか";
        let str2 = "ゲーム";
        let str3 = "いかゲーム";

        assert_eq!(to_katakana(str1).unwrap(), "イカ");
        assert_eq!(to_katakana(str2).unwrap(), "ゲーム");
        assert_eq!(to_katakana(str3).unwrap(), "イカゲーム");
    }
}
