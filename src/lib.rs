use std::process;
use serde::{Deserialize,Serialize};

//翻译参数
#[derive(Debug)]
pub struct TransArgs {
    to_lang: String,
    src: String,
}

impl TransArgs {
    fn new(to_lang: &str, src: &str) -> TransArgs {
        TransArgs{
            to_lang: String::from(to_lang),
            src: String::from(src),
        }
    }
    //从输入参数种生成翻译参数
    pub fn from(args: Vec<String>) -> Option<TransArgs >{
        if args.len() >= 3  {
            let to_lang = match args.get(1) {
                Some(lang) => GLang::match_google_lang(lang),
                None => {
                    eprintln!("请指定要翻译成什么语言。");
                    process::exit(1);
                }
            };
            let input_words:String = args[2..].join(" ");
            return Some(TransArgs::new(to_lang, input_words.as_str()));
        }
        return None;
    }
}



//谷歌翻译
pub struct GoogleTrans;
impl GoogleTrans {
    //翻译
    pub fn trans(trans_args: &TransArgs) {
        let url = format!(
            "https://translate.google.cn/translate_a/single?client=gtx&dt=t&dj=1&ie=UTF-8&sl=auto&tl={}&q={}",
            trans_args.to_lang,
            trans_args.src
        );
        reqwest::blocking::get(url).unwrap_or_else(|_|{
            eprintln!("请求出现问题啦~");
            process::exit(1);
        }).json::<GoogleTransRes>().unwrap_or_else(|_|{
            eprintln!("解析结果出现问题！");
            process::exit(1);
        }).display_res();
    }

}

//谷歌翻译结果
#[derive(Debug, Deserialize, Serialize)]
struct TransRes {
    trans: String,
    orig: String,
    backend: usize,
}

//谷歌翻译结果响应
#[derive(Debug, Deserialize, Serialize)]
struct GoogleTransRes {
    sentences: Vec<TransRes>,
    src: String,
}

impl GoogleTransRes {
    //显示翻译结果
    fn display_res(&self) {
        let first_sentence = self.sentences.get(0).unwrap_or_else(||process::exit(1));
        println!("{}", first_sentence.trans);
    }
}

//当前支持的翻译语种
struct GLang;
impl GLang {
    //匹配翻译结果语言
    fn match_google_lang(lang: &str) -> &str {
        match lang {
            "zh" => "zh_CN",
            "en" => "en_US",
            _ => {
                eprintln!(
                    "输入的语种不支持。\n当前支持的语言：\n  -{}\n  -{}",
                    "中文: zh",
                    "英文: en"
                );
                process::exit(0);
            }
        }
    }
}
