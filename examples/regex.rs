// examples/hello.rs
extern crate regex;
#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::borrow::Cow;

fn reformat_dates(before: &str) -> Cow<str> {
    lazy_static! {
        static ref ISO8601_DATE_REGEX : Regex = Regex::new(
            //r"(?P<y>\d{4})-(?P<m>\d{2})-(?P<d>\d{2})"
            //r"/^(?:(?:(?:https?|ftp):)?\/\/)(?:\S+(?::\S*)?@)?(?:(?!(?:10|127)(?:\.\d{1,3}){3})(?!(?:169\.254|192\.168)(?:\.\d{1,3}){2})(?!172\.(?:1[6-9]|2\d|3[0-1])(?:\.\d{1,3}){2})(?:[1-9]\d?|1\d\d|2[01]\d|22[0-3])(?:\.(?:1?\d{1,2}|2[0-4]\d|25[0-5])){2}(?:\.(?:[1-9]\d?|1\d\d|2[0-4]\d|25[0-4]))|(?:(?:[a-z0-9\u00a1-\uffff][a-z0-9\u00a1-\uffff_-]{0,62})?[a-z0-9\u00a1-\uffff]\.)+(?:[a-z\u00a1-\uffff]{2,}\.?))(?::\d{2,5})?(?:[/?#]\S*)?$/i"
            //r"(?:(?:(?:https?|ftp):)?\/\/)(?:\S+(?::\S*)?@)?(?:(?!(?:10|127)(?:\.\d{1,3}){3})(?!(?:169\.254|192\.168)(?:\.\d{1,3}){2})(?!172\.(?:1[6-9]|2\d|3[0-1])(?:\.\d{1,3}){2})(?:[1-9]\d?|1\d\d|2[01]\d|22[0-3])(?:\.(?:1?\d{1,2}|2[0-4]\d|25[0-5])){2}(?:\.(?:[1-9]\d?|1\d\d|2[0-4]\d|25[0-4]))|(?:(?:[a-z0-9\u00a1-\uffff][a-z0-9\u00a1-\uffff_-]{0,62})?[a-z0-9\u00a1-\uffff]\.)+(?:[a-z\u00a1-\uffff]{2,}\.?))(?::\d{2,5})?(?:[/?#]\S*)?$/i"
            //r"(?:(?:https?|ftp):)/i"
            //r"(?:(?:https?|ftp):)"
            r"(https?://(www\.)?[-a-zA-Z0-9@:%._\+~#=]{2,256}\.[a-z]{2,6}\b([-a-zA-Z0-9@:%_\+.~#?&//=]*))"
            ).unwrap();
    }

    //let caps = ISO8601_DATE_REGEX.captures(before).unwrap();
    //let text1 = caps.get(1).map_or("", |m| m.as_str());
    println!();
    for caps in ISO8601_DATE_REGEX.captures_iter(before) {
        println!("capture = {:?}", caps);
        println!();
    }
    println!();

    ISO8601_DATE_REGEX.replace_all(before, "FUCK YA")
}

fn main() {
    println!("Hello from an example!");
    let text = r#"
<p>Mit seinen Fördermillionen inszeniert sich Google als Gönner in Renaissancemanier. Der Datenkonzern immunisiert sich damit gegen Kritik der Medienbranche an seinen Geschäftspraktiken. Zugleich nutzt Google seine Nachrichteninitiative, um die Verlage noch enger an sein Produkt-Ökosystem zu binden. In unserem aktuellen Podcast erzählen wir davon, wie unsere monatelange Recherche zustande gekommen ist und was wir dabei noch so alles herausgefunden haben. Außerdem reden wir darüber, was die Google News Initiative für die Medienlandschaft in Europa bedeutet.</p>
<p>Hier ist <a href="https://cdn.netzpolitik.org/wp-upload/2018/10/NPP152-GoogleNewsInitiative.mp3">die mp3-Datei zum Download</a>. Alternativ bieten wir <a href="https://cdn.netzpolitik.org/wp-upload/2018/10/NPP152-GoogleNewsInitiative.ogg">eine ogg-Datei</a> an:</p>
<p><audio class="wp-audio-shortcode" id="audio-204205-19" preload="none" style="width: 100%;" controls="controls"><source type="audio/mpeg" src="https://cdn.netzpolitik.org/wp-upload/2018/10/NPP152-GoogleNewsInitiative.mp3?_=19" /><a href="https://cdn.netzpolitik.org/wp-upload/2018/10/NPP152-GoogleNewsInitiative.mp3">https://netzpolitik.org/wp-upload/2018/10/NPP152-GoogleNewsInitiative.mp3</a></audio></p>
<p>——————</p>
<p><em>Die Recherche zu Googles Geld und seinem Einfluss auf Journalismus und Medien in Europa ist ein Projekt von Alexander Fanta und Ingo Dachwitz. Unser Dank gebührt unseren Praktikant*innen <a href="https://netzpolitik.org/author/julianp/">Julian Pütz</a>, <a href="https://netzpolitik.org/author/leo-thuer/">Leo Thüer</a>, <a href="https://netzpolitik.org/author/jannik-mertens/">Jannik Mertens</a> und <a href="https://netzpolitik.org/author/wiebke-denkena/">Wiebke Denkena</a> für die Datenarbeit sowie <a href="http://oohaa.de/" data-versionurl="http://web.archive.org/web/20181004150302/http://oohaa.de/" data-versiondate="2018-10-04T15:03:04+00:00" data-amber-behavior="" data-versionurl="http://web.archive.org/web/20180926014304/http://oohaa.de/" data-versiondate="2018-09-26T01:43:05+00:00" data-amber-behavior="">Oliver Hinzmann</a> für die Illustration und <a href="https://lucahammer.com/" data-versionurl="http://web.archive.org/web/20181004145302/https://lucahammer.com/" data-versiondate="2018-10-04T14:53:04+00:00" data-amber-behavior="" data-versionurl="http://web.archive.org/web/20180926013807/https://lucahammer.com/" data-versiondate="2018-09-26T01:38:09+00:00" data-amber-behavior="">Luca Hammer</a> für die interaktive Grafik. Ein halbes Jahr lang haben wir Informationen über Googles (Digital) News Initiative und den dazugehörigen Innovationsfonds gesammelt, eine Datenbank der geförderten Projekte aufgebaut und analysiert, Interviews mit Expertinnen und Experten, Verantwortlichen und Mittelempfänger*innen geführt. Am Ende sind sie überzeugt: Die Nachrichteninitiative bringt spannende Projekte hervor und doch ist sie ein Problem. </em></p>
<p><em>Bisher erschienene Texte:</em></p>";"#;

    //let body = "Hahahahah https://github.com/servo/rust-url\r\nURL parser for Rust https://docs.rs/url/. you know";
    let _after = reformat_dates(&text);
    println!();
    //println!("body = {}", after);
    println!();
}
