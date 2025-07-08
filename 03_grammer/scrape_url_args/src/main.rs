use std::fs;

// 如果想让错误传播，可以把所有的 unwrap() 换成 ? 操作符，
// 并让 main() 函数返回一个 Result，如下所示
fn main() -> Result<(), Box<dyn std::error::Error>> {
    for arg in std::env::args() {
        println!("{}", arg);
    }

    let args = std::env::args().skip(1).collect::<Vec<_>>();

    if args.is_empty() {
        println!("Usage: scrape_url_args <url> <output file>");
        std::process::exit(1);
    }

    let url = &args[0];
    let output = &args[1];

    println!("Fetching url:{}", url);
    let body = reqwest::blocking::get(url)?.text()?;

    println!("Converting html to markdown...");
    let md = html2md::parse_html(&body);

    fs::write(output, md.as_bytes())?;
    println!("Converted markdown has been saved in {}.", output);

    Ok(())
}
