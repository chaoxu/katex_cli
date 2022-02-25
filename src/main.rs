use std::io;
use std::io::prelude::*;
fn main() {
    let stdin = io::stdin();
    let mut lines = Vec::<String>::new();
    for line in stdin.lock().lines() {
        lines.push(line.unwrap());
    }

    let input_string = lines.join("").to_owned();
    let input = &input_string[..];

    // parse the dom
    let dom = tl::parse(input, tl::ParserOptions::default()).unwrap();

    let mut final_output = Vec::<String>::new();
    
    // loop over every output in tl and collect
    let elements = dom.query_selector("span.math").unwrap();

    let macros_key_value: [(String, String); 13] = [
        ("\\C".to_string(),"\\mathbb{C}".to_string()),
        ("\\F".to_string(),"\\mathbb{F}".to_string()),
        ("\\e".to_string(), "\\varepsilon".to_string()),
        ("\\eps".to_string(), "\\varepsilon".to_string()),
        ("\\mex".to_string(), "\\mathop{\\operatorname{mex}}".to_string()),
        ("\\lcm".to_string(), "\\mathop{\\operatorname{lcm}}".to_string()),
        ("\\dist".to_string(), "\\mathop{\\operatorname{dist}}".to_string()),
        ("\\bigtriangleright".to_string(), "{\\mathop{\\Large \\triangleright}}".to_string()),
        ("\\bigtriangleleft".to_string(), "{\\mathop{\\Large \\triangleleft}}".to_string()),
        ("\\set".to_string(),"\\left\\{ #1 \\right\\}".to_string()),
        ("\\floor".to_string(),"\\left\\lfloor #1 \\right\\rfloor".to_string()),
        ("\\ceil".to_string(),"\\left\\lceil #1 \\right\\rceil".to_string()),
        ("\\abs".to_string(),"\\left\\| #1 \\right\\|".to_string())
    ];

    let macros = std::collections::HashMap::<String, String>::from(macros_key_value);

    let opts_base = katex::Opts::builder().
                       output_type(katex::opts::OutputType::Html).
                       throw_on_error(false).
                       macros(macros).
                       build().
                       unwrap();

    let mut opts = opts_base.clone();
    let mut display_opts = opts_base.clone();

    opts.set_display_mode(false);
    display_opts.set_display_mode(true);

    let mut out_start = 0;
    for element in elements {
        let node = element.get(dom.parser()).unwrap().as_tag().unwrap();
        let inner_stuff = node.inner_text(dom.parser());

        let raw = node.raw().as_bytes();

        let start = raw.as_ptr() as usize - input.as_ptr() as usize;
        let end = start + raw.len();

        // compute the katex options
        let mut option = &opts;
        if node.attributes().is_class_member("display") {
            option = &display_opts;
        }

        // compute the katex output
        let katex_output = katex::render_with_opts(&inner_stuff, &option).unwrap();

        // push the output to vec
        final_output.push(input[out_start..start].to_string());       
        out_start = end+1;
        final_output.push(katex_output);
    }
    final_output.push(input[out_start..].to_string());

    println!("{}", final_output.join(""))

}
