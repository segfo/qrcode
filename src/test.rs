use data_type;
use data_type::*;

// 文字列等のデータタイプ自動検出のテスト
#[test]
fn datatype_autodetection_test() {
    let url = "https://www.google.co.jp/search?safe=off&ei=WRPdW_3INIq-wAOHrrygDA&q=%E3%83%86%E3%82%B9%E3%83%88%E3%82%B1%E3%83%BC%E3%82%B91&oq=%E3%83%86%E3%82%B9%E3%83%88%E3%82%B1%E3%83%BC%E3%82%B91&gs_l=psy-ab.3...17357.21689.0.22021.23.15.8.0.0.0.121.1225.12j2.15.0....0...1c.1j4.64.psy-ab..1.20.1108.6..0j35i39k1j0i4k1j0i67k1.104.0p2cGopLn_Y";
    assert!(data_type::datatype_detect(url.as_bytes())==DataKind::Text,"アサーション失敗：DataKind==Textでなければならない。(URLのため)");

    let utf8 = "日本語";
    assert!(data_type::datatype_detect(utf8.as_bytes())==DataKind::Binary,"アサーション失敗：DataKind==Binaryでなければならない。(UTF-8文字列のため)");

    let binary = [0xff,0xfe,0x12,0x23,0xef,0x00,0xa4,0x7f,0x42];
    assert!(data_type::datatype_detect(&binary)==DataKind::Binary,"アサーション失敗：DataKind==Binaryでなければならない。(バイナリ列のため)");
}
