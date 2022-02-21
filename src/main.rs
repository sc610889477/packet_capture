extern crate pnet;
use pnet::datalink;

fn main() {
    // キャプチャに使うインターフェース名を1番目のコマンド引数から取得する．0番目はプログラムの名前
    // unwrapはエラーメッセージの出力が成功したかどうかをチェックする．失敗した場合はエラーメッセージを出力する
    let arg = std::env::args().nth(1).unwrap();

    // すべてのネットワークインターフェースを取得する
    let interfaces = datalink::interfaces();

    // 使いたいインターフェースが存在すれば値はOption<T>型として取得し，存在しない場合はNoneで返す
    let interface = interfaces.iter().find(|iface| iface.name == arg);
    match interface {
        None => {
            println!("Failed get Interface!");
            std::process::exit(1);
        },
        Some(interface) => Some(interface)
    };
    println!("Selected Interface Name: {}", interface.unwrap().name);
}
