
use lazy_static::lazy_static;

use ark_std::str::FromStr;
use ark_bn254::Fr as F;

//------------------------------------------------------------------------------

fn unsafe_field_from_str(x: &str) -> F {
  F::from_str(x).unwrap()
}

//------------------------------------------------------------------------------

lazy_static! {
  pub static ref EXTERNAL_ROUND_CONST: [F; 24] =
    EXTERNAL_ROUND_CONST_STR.into_iter().map( unsafe_field_from_str ).collect::<Vec<F>>().try_into().unwrap();
}

lazy_static! {
  pub static ref INTERNAL_ROUND_CONST: [F; 56] =
    INTERNAL_ROUND_CONST_STR.into_iter().map( unsafe_field_from_str ).collect::<Vec<F>>().try_into().unwrap();
}

//------------------------------------------------------------------------------

static EXTERNAL_ROUND_CONST_STR: [&str; 24] =
  [ "20036611579827150559091469005844175073625940102952070649817884191797764107075"
  , "12833584042949159986565784794014151972247796182705941809242049488642050965764"
  , "20460265239335923814507658708649753625122635556480788121847315929099732630160"
  , "2327433556176440050072122789158937095349015830966296688052004527413893088211"
  , "18928950063089170035907725405866336756839578021518946851617532132424258251714"
  , "1498343560697108822498273562774300261830404487630763796790963549875888044170"
  , "10894060797649430325830102265601652830696874259935838091685396334824506870974"
  , "10724282231429846920698162462239380161919181405938370346044877808218935812722"
  , "12900822782410361357346122719301635014682733276151991957222022994303272655598"
  , "12281187892168564254318627327513968787860396596558174241050192282039059721015"
  , "20650821582781615100585145227212437582142977940055542470636589623435638972974"
  , "10629768372315475914205126397268924392851627409938344639024631693382468091238"
  , "16917851076945834679745654051495359311013500015444379399867812928847812812510"
  , "5864181025763406655115777223889072827293022750325008731169039770623910915277"
  , "10954281474369541712624748628862576225398375445664590376470032986861704444851"
  , "13047511983320021446527738191176357616721874803157974120155396052149484216634"
  , "17384764166393886603954067639141617082485023441458666471633998399316404519396"
  , "2362259915869885612166233838782699563177506073802792748904427749547307239091"
  , "3449611877995076637446149781868130604254776727586032907573864421212958534148"
  , "8105036071757751706553034047983212897131570345551297243248892142702129409678"
  , "4057622885794382454162574102227879765926272192471614700281108386186208378108"
  , "19253655284986776930766796883935217167812408888646382124396693895764793065618"
  , "14562077372515720927783340475811850803822078325552042696568002577429594161808"
  , "7014243471382661219949882659461009076113783570411660059539677464905457050066"
  ];

//------------------------------------------------------------------------------

static INTERNAL_ROUND_CONST_STR: [&str; 56] =
  [ "9863412385841952362091849999302344670551536862597895561278739824943955436428"
  , "12647165719356049717276082841324782064533712829755618494899697790036659206896"
  , "7086910375195160419007414095698529055230556352178368086440281946317648386880"
  , "3534616497502400285225524656623573157846352519939301302938694874082052072531"
  , "7614193770971378304733014379518957809467088529876055633743098033063991937988"
  , "5524517084220026554140171636731805466222414790303448093874175843707842149296"
  , "6465594825680448812389988135904471523354179427789234651961801931723687100590"
  , "13069961729933398081974430219710353490873542026760226429212509555519019152029"
  , "20442858075434477469830277030056901344749316967720547608602986649544991223254"
  , "2545061208850080799716748664877827143438576541109233212439269028123126995283"
  , "10590804083025333327978466704873260869373451484886744716135293454855421776182"
  , "2971801848580094041349173303826341900320100639936132696391170461147651791599"
  , "6457488363353205666653234338157121013068090150457850441006724736538329117150"
  , "6828448758803946230526683011338306528814963654399318058641145666219257935090"
  , "21220571338383411884578252416062534137998565615530587243864120751033430171639"
  , "3026330605417380631686907051427895875297995068925155493166857059368062462182"
  , "731369258216230615295136666016875570919026708063642185179381726677850241678"
  , "3976056293038938116541801753309319302894026070844887224129384450572018141696"
  , "18191771098193274629372635247562817907522214770330118615218435763543549998849"
  , "5725653743840821273222706240430501486301231493172435748281584336969930288424"
  , "11315632284569557454743403424858271195359234409995993988812038704370836780935"
  , "1554548474507260015710470936845569353450016357484523407182389210520378329681"
  , "4043703307392904605083268263455220115794158610223459176828550627728253474746"
  , "5147341731800747507000152527876279909425699775431448328748957228886538246347"
  , "316000240737142429159443531715814797123772606639483557601339985787766020188"
  , "9201639581870600108429525788569689133841787929111628476773761218482387877013"
  , "76452313668236664875560655448227033496493983051536430171258861062052191226"
  , "5503629712269636063700501314848921574392241749467380315893984871270736181455"
  , "6403315002483104841708481320831358853734637389275574672824410469941470947501"
  , "18767770632814906108167838116228345063195454403262264488888241387554264238660"
  , "373847975877840160326133445669241099117560443926620032996032499044947786206"
  , "21583421457099893419202355352294471270616995660291443431182584120442735921995"
  , "9983253793391637270425117523681631923584433357117997691348261314215409417977"
  , "15636580057941931488663342701921482616732902735747145545560675417041959528168"
  , "13987037342707768451704041319080865947736730484739494822133564732867708182129"
  , "17012807810939334924306150791435050869173026502739324471672493058506335677602"
  , "20445006141370558272839096434547759512337597485395700957462476332341263470795"
  , "8919512276260973838722894975323643726180891487427800112399328013257265098884"
  , "5121935724947519798577565968119435009768355186616305612599005246528173860072"
  , "14390107556389236497954661819339690487046314759239155128415464040636056995180"
  , "15584006578697113728990467783042934569634784959449117771168091586065677667289"
  , "1894113083403807703900670244801865115743187370284525340414788037116278755190"
  , "10705487296676443016268478693800719585977460625398657826409495078078625785779"
  , "19873128801331021326057263165428495080386730321288221112754217581989746046187"
  , "9535868612309525085410638759100096355215966759273780340615254181304597180876"
  , "19343524237095964073059512688602276354221693507832098135091590955813103860375"
  , "6822002759027581661397720296774923367218318024525870595858692787302610345838"
  , "20926315677022871764300366782923548421637739455364254388830334528197327379743"
  , "21136323309320513006223521228388165385265667945268848018308200096388016784087"
  , "6225919113652258029432904641421409709120407088870082365870799895213524311811"
  , "21311480157558702908741067111471547471593313246226635294078364811203226453373"
  , "14198190262525448445981718442399104668575545443532459171696901751739593839857"
  , "12459931960399741184291910088669153631848894479486118281597822681015713751918"
  , "6907332911214986094624912982222847460679912779076715919016253686198271600395"
  , "3393759605179403680839415959192212038674751416897927769901727596993250674985"
  , "16080342864802322672358399934807873723002376453489101187414456500323890285012"
  ];

//------------------------------------------------------------------------------