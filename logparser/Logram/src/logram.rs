use crate::dictionary::dictionary_builder;
use crate::match_token::token_match;

use std::path::PathBuf;
use fancy_regex::Regex as FancyRegex;
use std::error::Error;

pub struct LogParser{
    in_dir : PathBuf,
    out_dir : PathBuf,
    double_threshold : i32,
    tri_threshold : i32,
    log_format: String,
    preprocess_regex : Vec<FancyRegex>,
}

impl LogParser{
    pub fn new(
        indir:Option<String>,
        outdir:Option<String>,
        double_threshold:Option<i32>,
        tri_threshold:Option<i32>,
        log_format:String,
        preprocess_regex:Vec<String>,
    )->Self{
        let default_indir = "./".to_string();
        let default_outdir = "./result/".to_string();
        let default_double_threshold:i32 = 15;
        let default_tri_threshold:i32 = 10;

        let preprocess_regex: Vec<FancyRegex> = preprocess_regex.iter()
        .map(|pattern| FancyRegex::new(pattern).expect("Failed to compile regex pattern"))
        .collect();

        LogParser{
            in_dir: PathBuf::from(indir.unwrap_or(default_indir)),
            out_dir : PathBuf::from(outdir.unwrap_or(default_outdir)), 
            double_threshold: double_threshold.unwrap_or(default_double_threshold),
            tri_threshold: tri_threshold.unwrap_or(default_tri_threshold),
            log_format,
            preprocess_regex,
        }
    }

    pub fn parse(&self, log_file_basename: String)-> std::result::Result<(), Box<dyn Error>>{
        let log_file_path = self.in_dir.join(&log_file_basename);
        println!("Parsing file: {}", log_file_path.display());

        let (
            double_dictionary_list,
            tri_dictionary_list,
            all_token_list,
            all_message_list,
        ) = dictionary_builder(self.log_format.clone(), log_file_path, self.preprocess_regex.clone());

        // // *test
        // println!("doubleDictionaryList = \n\t{:?}",double_dictionaryList);
        // println!("triDictionaryList = \n\t{:?}", tri_dictionaryList);
        // println!("allTokenList = \n\t{:?}", all_tokenList);
        // println!("allMessageList = \n\t{:?}", all_messageList);

        token_match(
            &all_token_list,
            &double_dictionary_list,
            &tri_dictionary_list,
            self.double_threshold,
            self.tri_threshold,
            self.out_dir.to_str().unwrap(),
            &log_file_basename,
            &all_message_list,
        );
        println!("Parsing done.");
        Ok(())
    }
}