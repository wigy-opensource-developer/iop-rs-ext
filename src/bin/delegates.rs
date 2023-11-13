use anyhow::Result;
use futures::{stream, StreamExt, TryStreamExt};

use iop_rs_ext::*;

const PHRASES: [&str; 53] = [
    "energy slide remind flip select merge blush clay giraffe doll easy grape",
    "very special misery now wage monitor range summer moment monkey piece stage",
    "vital angry ketchup purchase bar crop enable roof canal comfort vessel sentence",
    "marine ask aunt acid kit kiwi hire unlock embody bright live good",
    "hospital liar evoke quick name people early volcano eternal august stone pigeon",
    "purchase issue motor claim oven social result video elegant seed scare notice",
    "math stem major atom want glove fog health chef loyal window profit",
    "letter mushroom tomorrow tortoise this theory where material chicken asthma aisle long",
    "legal demise hurry announce original journey horn future like junior spare hair",
    "retire moon broom recipe crop mansion welcome enhance poem install lizard output",
    "thank draft online couple easy profit gun profit profit piece deputy seek",
    "soft tank nice program save talent blur corn enough eye clump social",
    "captain response verb bachelor hidden olive vault coyote initial laptop huge advance",
    "toe garlic upon two cloud vibrant olympic tooth imitate gown turkey tone",
    "nest pledge sheriff divert much load damage pottery force stool spoil neutral",
    "clay roof token beauty pluck black rose decline blouse settle element sorry",
    "level term shield common ribbon odor easy tourist mass chest share save",
    "repair lady home glad bar dinner brisk loud setup refuse render brass",
    "sail absurd butter holiday office juice exclude differ circle alley slam often",
    "share ability become provide biology remember cousin excuse only lend zone also",
    "sort rocket rare donate sleep roof adjust nothing wild flock hundred history",
    "domain once maze hungry sword combine like ghost calm protect pond tail",
    "liquid brief cake actress garbage sick near combine fold feel piece outdoor",
    "retire scout apple riot mammal flush educate close call relax cram junk",
    "mom rocket sweet scrap veteran champion tower one still tank language orbit",
    "skill life space rough beyond endless pen model critic weather crunch cruise",
    "stumble round simple glance cave vacant river happy soup reform prevent goddess",
    "head elbow oblige kiwi fog drill buddy sphere chat job olympic section",
    "lens come wrestle between clinic raw gospel shiver solve pupil enjoy guard",
    "trigger cherry ozone time learn print panic annual they cushion icon crime",
    "hazard heart elder donor border lion photo bid busy bullet wear fuel",
    "this book monitor cable episode unfair involve since essay shop square energy",
    "submit priority skate deer purity despair cotton prosper you twice merit age",
    "sudden fox captain will lottery quantum twist fold jump innocent dad decrease",
    "endless climb dial involve cause wool bracket edit humor park clarify drill",
    "cargo absurd tobacco park churn pitch fish lake install skirt excuse range",
    "hair broccoli divorce wood conduct fury consider scrap physical jungle card ability",
    "matrix alley bamboo vital build scare document come define poverty memory elegant",
    "test penalty grid turtle tiger comfort scrap glide sphere slam link mesh",
    "maple leopard sea wall tree citizen dinner expose slight want ritual shiver",
    "rotate omit rather pass position tongue cube minimum clap because boy defy",
    "use ethics index enjoy ordinary quote sport drastic outer female letter social",
    "inject stage voice radio monitor problem lobster live sample addict hint squeeze",
    "welcome enlist mouse skill pony recipe exclude filter delay foil disagree cycle",
    "patient skate sibling task mix spoon melody inform guess host ring loop",
    "damp agent comfort another goddess smoke bind forest describe pave must main",
    "soccer pole live viable mention hat remove ugly inject airport pilot disagree",
    "anxiety meadow palm trust pilot wild fossil control decrease suffer defense issue",
    "oyster toe agent screen wage garbage draw reveal pelican reject original genuine",
    "spot north furnace ordinary subway knee drive reopen bid shoulder black term",
    "trigger opera grocery post tumble enter festival cruise worth camera keen omit",
    "hotel aspect live balance decade farm vocal noble rebuild autumn owner true",
    "tornado earn six dad air size escape another border once eye pool",
];

#[tokio::main(flavor = "current_thread")]
pub async fn main() -> Result<()> {
    let api = &Api::new("https://test.explorer.hydraledger.io:4705/api/v2/")?;
    let wallets: Vec<_> = stream::iter(PHRASES.into_iter().enumerate())
        .map(|(idx, phrase)| async move {
            let wallet = ArkWallet::new(phrase, "HYD testnet")?;
            let addr = wallet.addr()?;
            let response = api.wallet(&addr).await;
            if let Ok(w) = &response {
                assert_eq!(&addr, &w.address);
            }
            Result::<(usize, Result<WalletResponse>)>::Ok((idx, response))
        })
        .buffer_unordered(10)
        .try_collect()
        .await?;

    for (idx, res) in wallets {
        match res {
            Ok(w) => println!("#{idx:#02}: balance: {}, nonce: {}", w.balance, w.nonce),
            Err(e) => println!("#{idx:#02}: error: {e}"),
        }
    }

    Ok(())
}
