use std::io::{self, Write};

use clap::{ArgGroup, Parser};
use wikis::{MyClient, TopicSelector, TopicSelectorTerminal, TopicTaker, TopicTakerStdin};

#[derive(Parser, Debug)]
#[command(version, about = env!("CARGO_PKG_DESCRIPTION"), long_about = None)]
#[command(group(
        ArgGroup::new("Query input method")
        .args(["topic", "query_stdin"])
        .required(true)
))]
struct Args {
    /// Don't provide the link
    #[arg(long)]
    no_link: bool,

    /// Don't provide the summary
    #[arg(long)]
    no_summary: bool,

    /// Language edition of Wikipedia to use; defaults to en for English; supply language code from https://en.wikipedia.org/wiki/List_of_Wikipedias#Active_editions
    #[arg(long)]
    lang: Option<String>,

    /// Index of the topic to choose without prompting 
    #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..))]
    choice: Option<u8>,


    /// Open the Wikipedia page in default browser
    #[arg(long)]
    browser: bool,

    /// Take query from Stdin instead from arguments
    #[arg(long)]
    query_stdin: bool,

    /// No texts in prompts
    #[arg(long)]
    no_prompt_text: bool,

    /// Topic to search on the Wikipedia
    #[arg()]
    topic: Vec<String>,
}

fn main() {
    let args = Args::parse();

    let topic = if args.query_stdin {
        let tts = TopicTakerStdin{show_prompt_text: !args.no_prompt_text};
        if let Some(q) = tts.take_topic() {
            q
        } else {
            eprintln!("Error taking query from Stdin");
            return;
        }
    }else{
        args.topic.join(" ")
    };

    let lang = args.lang.unwrap_or("en".to_string());

    if !LANGS.contains(&lang) {
        eprintln!("Invalid lang code");
        return;
    }

    let client = MyClient::new();

    let (topics, links) = client.search(&lang, &topic);

    let choice = if topics.len() > 1 {
        if let Some(c) = args.choice.map(|t| t as usize) {
            if c < 1 || c > topics.len() {
                eprintln!("Index out of bound");
                return;
            }
            c - 1
        } else {
            let from_term = TopicSelectorTerminal {show_prompt_text: ! args.no_prompt_text}.select(&topics);

            match from_term {
                Some(c) => c,
                None => {
                    return;
                }
            }
        }
    } else if topics.len() == 1 {
        0
    } else {
        eprintln!("Nothing related to {} was found.", topic);
        return;
    };

    if args.browser {
        if webbrowser::open(&links[choice]).is_ok() {
            println!("Opening the article in browser");
        } else {
            eprintln!("Error opening the link using browser");
        }
        return;
    }

    let (title, link, summary) = client.summarize(&lang, &topics[choice], &links[choice]);

    print!("{}\n", title);

    if !args.no_link {
        print!("{}\n", link);
    }

    if !args.no_summary {
        print!("{}\n", summary);
    }

    io::stdout().flush().unwrap();
}

const LANGS:&str = "en
fr
de
es
ja
ru
pt
it
zh
fa
pl
ar
nl
uk
he
id
no
tr
ro
sr
cs
sv
ko
da
simple
fi
hu
vi
ca
bn
th
zh-yue
el
et
sw
bg
hi
ms
eu
az
hy
sh
sk
hr
uz
lt
eo
sl
ka
lv
be
gl
kk
ta
sq
ur
mk
ml
ceb
arz
af
la
bs
te
tl
is
nn
my
ha
mn
ckb
as
mr
bcl
ig
pa
oc
ast
jv
cy
be-tarask
kn
azb
tt
ky
ne
si
ga
als
zh-min-nan
tg
br
an
sco
lb
ku
ba
war
fy
so
ban
hif
gu
wuu
dtp
km
pnb
yo
io
lmo
mt
bar
ps
am
min
ary
cv
su
ce
ht
rw
nds
sa
or
bew
vec
ia
kaa
ang
ff
mg
qu
zh-classical
fo
zgh
szl
zu
dag
hyw
yi
ace
sd
bjn
mad
rue
mai
li
xmf
ts
diq
lld
fur
gd
co
nso
fiu-vro
sah
sc
scn
bh
anp
pam
lo
tk
crh
guc
ie
nap
pdc
gor
ve
mzn
bo
wa
lad
lij
pms
hsb
ks
ab
hak
ilo
pcd
vo
av
kab
sat
roa-rup
dsb
gv
ss
pap
tcy
wo
ay
chr
igl
csb
lg
frr
syl
xh
gn
frp
kl
guw
pcm
tn
vls
dz
eml
gag
dv
mi
rm
st
os
dty
cdo
got
smn
iu
jbo
tyv
nds-nl
map-bms
cbk-zam
kw
cu
tw
ext
gpe
haw
avk
lfn
nah
nv
new
om
udm
bat-smg
myv
rn
mni
skr
shi
bbc
ug
bxr
ee
kv
kus
olo
mhr
mdf
nqo
stq
vep
zea
se
awa
gan
glk
kg
lez
mwl
roa-tara
tum
bug
cr
jam
blk
pfl
pag
szy
sn
arc
atj
bpy
bi
ny
ln
rmy
sm
knc
mrj
iba
xal
krc
ltg
nrm
tly
gom
ady
fat
inh
mnw
nia
nov
rsk
tpi
za
bm
dga
koi
trv
shn
ami
chy
fj
kbp
btm
nr
fon
ki
lbe
pnt
kcg
ksh
gcr
gur
to
ch
din
ik
kbd
mos
pi
sg
tet
ti
tay
kge
pwn
tig
alt
srn
ty
ann
tdd
bdr";
